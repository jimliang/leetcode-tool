use std::path::PathBuf;

use anyhow::{bail, Result};
use async_std::{
    fs::{File, OpenOptions},
    io::BufWriter,
    io::WriteExt,
    process::Command,
};

use crate::{
    domain::{CodeSnippet, Question},
    guest::guest_output,
    meta::{MetaData, MetaDataMethod, MetaDataType},
    testcase::{parse_class_test_cases2, parse_test_cases},
};
use inflector::Inflector;

use super::{END_LINE, START_LINE};

struct WriteTemplate<'a> {
    question: &'a Question,
    snippet: &'a CodeSnippet,
    test_code: Option<String>,
    // import_code: Vec<String>,
    title: String,
}

impl<'a> WriteTemplate<'a> {
    fn new(question: &'a Question) -> Result<Self, anyhow::Error> {
        let snippet = match question.code_snippets.iter().find(|c| c.lang == "Rust") {
            Some(snippet) => snippet,
            None => bail!("Fail to get Rust code Snippet"),
        };

        let title = question.title_slug.replace('-', "_");
        Ok(Self {
            question,
            snippet,
            test_code: None,
            // import_code: vec![],
            title,
        })
    }

    fn generate_test_code(&mut self) -> Result<bool, anyhow::Error> {
        let meta: MetaData = serde_json::from_str(&self.question.meta_data)?;

        let v = match meta {
            MetaData::Base {
                name,
                params,
                r#return,
            } => {
                // let mut type_iter = params
                //     .iter()
                //     .map(|p| &p.r#type)
                //     .chain(std::iter::once(&r#return.r#type));

                // if type_iter.clone().any(|ty| match ty {
                //     MetaDataType::TreeNode => true,
                //     _ => false,
                // }) {
                //     self.import_code.push("use leetcode_tool::TreeNode;".into());
                // }
                // if type_iter.any(|ty| match ty {
                //     MetaDataType::ListNode => true,
                //     _ => false,
                // }) {
                //     self.import_code.push("use leetcode_tool::ListNode;".into());
                // }
                let test_cases = {
                    let test_cases_str = if let Some(s) = self.question.example_testcases.as_ref() {
                        s
                    } else {
                        &self.question.sample_test_case
                    };
                    parse_test_cases(test_cases_str)?
                };

                let mut output_iter =
                    guest_output(&self.question.translated_content).filter_map(|output| {
                        let o = match r#return.r#type {
                            MetaDataType::Integer => {
                                let num = output.parse().ok()?;
                                serde_json::Value::Number(num)
                            }

                            _ => serde_json::from_str(&output).ok()?,
                        };
                        format_val(&o, &r#return.r#type).ok()
                    });
                let method_name = name.to_snake_case();

                let test_cases = into_test_cases_iter(test_cases, params.len())
                    // .enumerate()
                    .map(|test_case| {
                        // if r#return.r#type == MetaDataType::Void {
                        //     format!(r"
                        //         let mut param{i} =
                        //         let res{i}
                        //     ")
                        // }
                        let params_str =
                            format_params(test_case.iter(), params.iter().map(|p| &p.r#type));
                        let expects = output_iter.next().unwrap_or_default();

                        format!("assert_eq!(Solution::{method_name}({params_str}), {expects});")
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                let test_code = format!(
                    r"
                #[test]
                    pub fn test_{method_name}() {{
                        {test_cases}
                }}
                "
                );

                self.test_code = Some(test_code);

                false
            }
            MetaData::Class {
                classname,
                constructor,
                methods,
                r#return: _,
            } => {
                // let struct_name = parse_struct_name(&self.snippet.code).unwrap_or("UnknowStruct");

                // self.import_code
                //     .push("use leetcode_tool::{TestObject, assert_object};".into());
                // self.import_code.push("use serde_json::Value;".into());

                let method_code = methods.iter().map(
                    |MetaDataMethod {
                         name,
                         params,
                         r#return,
                     }| {

                        let param_lines = params.iter().enumerate().map(|(i, param)| {
                            let s = match param.r#type {
                                MetaDataType::Integer => format!("params[{i}].as_i64().unwrap() as i32"),
                                _ => "".into(),
                            };

                            format!("let p{i} = {s};")
                        }).collect::<Vec<String>>();
                        let res = match r#return.r#type {
                            MetaDataType::Bool => Some("Value::Bool(res)".to_owned()),
                            MetaDataType::Integer => Some("Value::Number(res.into())".to_owned()),
                            _ => None,
                        };

                        let ps_code = param_lines.join("\n");
                        let ps_code2 = param_lines.iter().enumerate().map(|(i, _)| format!("p{i}")).collect::<Vec<String>>().join(",");

                        let body = if let Some(res) = res {
                            format!("{ps_code}\nlet res = self.{name}({ps_code2});\nreturn Some({res})")
                        } else {
                            format!("{ps_code}\nlet _ = self.{name}({ps_code2});")
                        };

                        format!("\"{name}\" => {{ {body} }}")
                     },
                ).collect::<Vec<String>>().join("\n");

                let mut output_iter = guest_output(&self.question.translated_content);

                let (methods_json, params_json) = get_class_output(self.question)?;

                let constructor_param = {
                    let params = get_first_value(params_json).unwrap();
                    format_params(
                        params.as_array().unwrap().iter(),
                        constructor.params.iter().map(|p| &p.r#type),
                    )
                };

                let methods_json = format!("r#\"{}\"#", methods_json);
                let params_json = format!("r#\"{}\"#", params_json);
                let excepts_json = match output_iter.next() {
                    Some(output) => format!("r#\"{}\"#", output),
                    None => "".to_owned(),
                };

                let test_code = format!(
                    r#"
            impl TestObject for {classname} {{
                fn call(&mut self, method: &str, params: &[Value]) -> Option<Value> {{
                    match method {{
                        {method_code}
                        _ => {{}},
                    }}
                    None
                }}
            }}

            #[test]
            pub fn test_{classname}() {{
                let methods = {methods_json};
                let params = {params_json};
                let excepts = {excepts_json};
                let obj = {classname}::new({constructor_param});
                assert_object(obj, methods, params, excepts);
            }}
            "#,
                );

                self.test_code = Some(test_code);
                true
            }
        };

        Ok(v)
    }

    fn get_doc_code(&self) -> String {
        let Question {
            title_slug,
            translated_title,
            difficulty,
            translated_content,
            ..
        } = self.question;

        let md_lines = html2md::parse_html(translated_content)
            .split('\n')
            .map(|line| format!("/// {line}"))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            r"
        /// # {translated_title}
        ///
        {md_lines}
        ///
        /// src: https://leetcode-cn.com/problems/{title_slug}/
        ///
        /// difficulty: `{difficulty}`
    "
        )
    }

    async fn write_to(&mut self, project_dir: PathBuf) -> Result<PathBuf, anyhow::Error> {
        let is_class = self.generate_test_code()?;
        let file_path = project_dir.join(format!("src/{}.rs", self.title));
        let file = File::create(&file_path).await?;
        let mut buf_writer = BufWriter::new(file);

        let import_code = "use leetcode_tool::prelude::*;";
        let doc_code = self.get_doc_code();
        let struct_code = if is_class { "" } else { "pub struct Solution;" };
        let test_code = self.test_code.take().unwrap_or_default();
        let snippet_code = &self.snippet.code;

        buf_writer
            .write(
                format!("{import_code}\n{doc_code}\n{struct_code}\n{START_LINE}\n{snippet_code}\n{END_LINE}\n{test_code}")
                .as_bytes(),
            )
            .await?;

        buf_writer.flush().await?;

        let mut lib_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(project_dir.join("src/lib.rs"))
            .await?;
        lib_file
            .write(format!("\npub mod {};", self.title).as_bytes())
            .await?;
        lib_file.flush().await?;

        if let Err(err) = cargo_fmt(project_dir).await {
            log::warn!("`cargo fmt` process failed to execute: {:?}", err);
        };
        Ok(file_path)
    }
}

pub async fn write_template(
    question: &Question,
    project_dir: PathBuf,
) -> Result<PathBuf> {
    let mut wt = WriteTemplate::new(question)?;
    let pb = wt.write_to(project_dir).await?;
    Ok(pb)
}

async fn cargo_fmt(project_dir: PathBuf) -> Result<std::process::ExitStatus> {
    let res = Command::new("cargo")
        .arg("fmt")
        .current_dir(project_dir)
        .status()
        .await?;
    Ok(res)
}

fn format_val(val: &serde_json::Value, meta_type: &MetaDataType) -> Result<String> {
    let v = match meta_type {
        MetaDataType::List(sub_meta_type) => {
            let array = match val {
                serde_json::Value::Array(a) => a,
                _ => {
                    bail!("format_val parse error: {} {:?}", val, meta_type);
                }
            };
            format!(
                "vec![{}]",
                array
                    .iter()
                    .filter_map(|v| match format_val(v, sub_meta_type) {
                        Ok(o) => Some(o),
                        Err(err) => {
                            log::warn!("format_val: {:?}", err);
                            None
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            )
        }
        MetaDataType::ListNode => {
            format!("ListNode::from_jsonstr(\"{}\")", val)
        }
        MetaDataType::TreeNode => {
            format!("TreeNode::from_jsonstr(\"{}\")", val)
        }
        MetaDataType::Character => format!("'{}'", val),
        MetaDataType::String => format!("{}.to_owned()", val),
        MetaDataType::Unknow(t) => bail!("Unknow MetaType {}", t),
        _ => val.to_string(),
    };
    Ok(v)
}

fn into_test_cases_iter<'a>(
    mut iter: impl Iterator<Item = serde_json::Value> + 'a,
    len: usize,
) -> impl Iterator<Item = Vec<serde_json::Value>> + 'a {
    (0..).map_while(move |_| {
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            match iter.next() {
                Some(v) => list.push(v),
                None => return None,
            }
        }
        Some(list)
    })
}

fn get_class_output(question: &Question) -> Result<(&str, &str)> {
    let (method_str, params_str) = {
        let test_cases_str = if let Some(s) = question.example_testcases.as_ref() {
            s
        } else {
            &question.sample_test_case
        };
        parse_class_test_cases2(test_cases_str).unwrap()
    };

    Ok((method_str, params_str))
}

fn get_first_value(json: &str) -> Option<serde_json::Value> {
    let params: serde_json::Value = serde_json::from_str(json).ok()?;
    match params {
        serde_json::Value::Array(a) => a.into_iter().next(),
        _ => None,
    }
}

fn format_params<'a, 'b>(
    vals: impl Iterator<Item = &'a serde_json::Value>,
    param_types: impl Iterator<Item = &'b MetaDataType>,
) -> String {
    vals.zip(param_types)
        .filter_map(|(val, param_type)| match format_val(val, param_type) {
            Ok(o) => Some(o),
            Err(err) => {
                log::warn!("format_params: {:?}", err);
                None
            }
        })
        .collect::<Vec<String>>()
        .join(",")
}

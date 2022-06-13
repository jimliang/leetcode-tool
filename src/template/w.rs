use std::path::PathBuf;

use anyhow::bail;
use async_std::{fs::File, io::BufWriter, io::WriteExt};

use crate::{domain::{Question, CodeSnippet}, meta::MetaData, util::parse_struct_name};

use super::{END_LINE, START_LINE};

// struct TestCode<'a> {

// }

// impl TestCode<'_> {
//     fn parse(question: &Question) {

//     }
// }

fn get_test_code(question: &Question, snippet: &CodeSnippet) -> String {

    let meta: MetaData = serde_json::from_str(&question.meta_data).unwrap();

    match meta {
        MetaData::Base { .. } => todo!(),
        MetaData::Class { classname, constructor, methods, r#return } => {
            let struct_name = parse_struct_name(&snippet.code).unwrap_or("UnknowStruct");
            let import_code = r"use crate::util::fs::{TestObject, assert_object};
            use serde_json::Value;";

            // let a = r"aa
            // aaa
            // ";

            let test_code = format!(r"
            impl TestObject for {} \{
                fn call(&mut self, method: &str, params: &Vec<Value>) -> Option<Value> \{
                    match method \{
                        {}
                        _ => \{\},
                    }
                    None
                }
            }
            ", struct_name, "");
        },
    }
    todo!()
}

pub async fn write_template(
    question: &Question,
    project_dir: PathBuf,
) -> crate::errors::Result<()> {
    let snippet = match question.code_snippets.iter().find(|c| c.lang == "Rust") {
        Some(snippet) => snippet,
        None => bail!("Fail to get Rust code Snippet"),
    };
    let test_code = "";

    let title = question.title_slug.replace('-', "_");
    let file_path = project_dir.join(format!("{}.rs", title));
    let mut file = File::create(file_path).await?;
    let mut buf_writer = BufWriter::new(file);

    buf_writer.write(b"pub struct Solution;\n").await.unwrap();
    buf_writer.write(START_LINE.as_bytes()).await.unwrap();
    buf_writer.write(b"\n").await.unwrap();
    buf_writer.write(snippet.code.as_bytes()).await.unwrap();
    buf_writer.write(END_LINE.as_bytes()).await.unwrap();
    buf_writer.write(test_code.as_bytes()).await.unwrap();

    Ok(())
}

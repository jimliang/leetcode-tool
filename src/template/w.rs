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

fn get_test_code(question: &Question, snippet: &CodeSnippet) -> (String, String) {

    let meta: MetaData = serde_json::from_str(&question.meta_data).unwrap();

    match meta {
        MetaData::Base { .. } => todo!(),
        MetaData::Class { classname, constructor, methods, r#return } => {
            let struct_name = parse_struct_name(&snippet.code).unwrap_or("UnknowStruct");
            let import_code = r"use crate::util::fs::{TestObject, assert_object};
            use serde_json::Value;";

            let test_code = format!(r"
            impl TestObject for {} {{
                fn call(&mut self, method: &str, params: &Vec<Value>) -> Option<Value> {{
                    match method {{
                        {}
                        _ => {{}},
                    }}
                    None
                }}
            }}
            ", struct_name, "");

            (import_code.to_owned(), test_code)
        },
    }
}

pub async fn write_template(
    question: &Question,
    project_dir: PathBuf,
) -> crate::errors::Result<()> {
    let snippet = match question.code_snippets.iter().find(|c| c.lang == "Rust") {
        Some(snippet) => snippet,
        None => bail!("Fail to get Rust code Snippet"),
    };
    let (import_code, test_code) = get_test_code(question, snippet);

    let title = question.title_slug.replace('-', "_");
    let file_path = project_dir.join(format!("{}.rs", title));
    let mut file = File::create(file_path).await?;
    let mut buf_writer = BufWriter::new(file);

    // buf_writer.write(b"pub struct Solution;\n").await.unwrap();
    // buf_writer.write(START_LINE.as_bytes()).await.unwrap();
    // buf_writer.write(b"\n").await.unwrap();
    // buf_writer.write(snippet.code.as_bytes()).await.unwrap();
    // buf_writer.write(END_LINE.as_bytes()).await.unwrap();
    // buf_writer.write(test_code.as_bytes()).await.unwrap();

    buf_writer.write(format!(r"
        {import_code}

        pub struct Solution;

        {START_LINE}

        {code}

        {END_LINE}

        {test_code}

    ", code = snippet.code).as_bytes()).await?;

    buf_writer.flush().await?;

    Ok(())
}

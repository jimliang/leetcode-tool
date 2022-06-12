use std::path::PathBuf;

use anyhow::bail;
use async_std::{fs::File, io::BufWriter, io::WriteExt};

use crate::domain::Question;

use super::{END_LINE, START_LINE};

// struct TestCode<'a> {

// }

// impl TestCode<'_> {
//     fn parse(question: &Question) {

//     }
// }

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

use std::time::Duration;

use crate::fetch::fetch_question;
use crate::leetcode::{check_submissions, submit, CheckSubmissionsResponse, SubmitResponse};
use crate::template::{END_LINE, START_LINE};
use anyhow::{bail, Ok, Result};
use async_std::prelude::*;
use async_std::process::Command;
use async_std::task::sleep;
use async_std::{fs::File, io::BufReader, path::Path};
use regex::Regex;

pub async fn submit_code(title_slug: &str, cookie: &str) -> Result<()> {
    let title = title_slug.replace('-', "_");
    let file = format!("src/{title}.rs");
    let (title_slug, code) = read_content(&file).await?;

    let question = fetch_question(&title_slug).await?;

    let submit_resp = submit(&question, &code, cookie).await?;

    let submission_id = match submit_resp {
        SubmitResponse::SUCCESS { submission_id } => submission_id,
        SubmitResponse::ERROR { error } => return Err(anyhow::anyhow!(error)),
    };

    for _ in 0..20 {
        let resp = check_submissions(submission_id).await?;
        match resp {
            CheckSubmissionsResponse::STARTED | CheckSubmissionsResponse::PENDING => {
                sleep(Duration::from_secs(1)).await;
            }
            CheckSubmissionsResponse::SUCCESS {
                status_msg,
                submission_id,
                status_runtime,
                status_memory,
                ..
            } => {
                if &status_msg == "Wrong Answer" {
                    bail!("{:?}", status_msg);
                } else {
                    // success

                    Command::new("git").args(&["add", &file]).status().await?;

                    Command::new("git")
                        .args(&["commit", "-m", &format!("leetcode({title_slug}): {submission_id}, ({status_runtime}, {status_memory})")])
                        .status()
                        .await?;

                    println!("success({title_slug}): {submission_id}, ({status_runtime}, {status_memory})");
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn read_content<P: AsRef<Path>>(file: P) -> Result<(String, String)> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"leetcode-cn.com/problems/(\S+)/").unwrap();
    }
    let f = File::open(file).await?;
    let mut buffer_reader = BufReader::new(f);

    let mut title_slug = String::new();
    let mut start = false;
    let mut rust_code = String::new();

    let mut buf = String::new();

    loop {
        let len = buffer_reader.read_line(&mut buf).await?;

        if len == 0 {
            break;
        }

        if buf.starts_with("/// src: https://leetcode-cn.com/") {
            let mut iter = RE.captures_iter(&buf);
            title_slug = iter.next().unwrap().get(1).unwrap().as_str().to_owned();
        } else if buf.starts_with(START_LINE) {
            start = true;
        } else if buf.starts_with(END_LINE) {
            start = false;
        } else if start {
            rust_code.push_str(&buf);
            rust_code.push('\n');
        }

        buf.clear();
    }

    Ok((title_slug, rust_code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_content() {
        async_std::task::block_on(async {
            let c = read_content("src/random_pick_with_blacklist.rs")
                .await
                .unwrap();
            println!("{:?}", c);
        })
    }
}

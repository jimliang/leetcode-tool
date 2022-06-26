use std::collections::HashMap;
use std::time::Duration;

use crate::fetch::{graphql, GraphqlBody};
use crate::template::{END_LINE, START_LINE};
use anyhow::{Ok, bail};
use async_std::prelude::*;
use async_std::process::Command;
use async_std::task::sleep;
use async_std::{fs::File, io::BufReader, path::Path};
use regex::Regex;

pub async fn submit_code(title_slug: &str) -> Result<(), anyhow::Error> {
    let title = title_slug.replace('-', "_");
    let file = format!("src/{title}.rs");
    let (title_slug, code) = read_content(&file).await?;

    // submit
    let submit_id = "";

    for _ in 0..20 {
        let CheckSubmissionsResponse {
            state,
            status_msg,
            submission_id,
            status_memory,
            status_runtime,
        } = check_submissions(submit_id).await?;
        if state == "SUCCESS" {

            if status_msg == Some("Wrong Answer".to_owned()) {
                return bail!("{:?}", status_msg.unwrap());
            } else {
                // success

                Command::new("git")
                    .args(&["add", &file])
                    .status()
                    .await?;

                Command::new("git")
                    .args(&["commit", "-m", &format!("\"leetcode({title_slug}): {submission_id}, ({status_runtime}, {status_memory})\"")])
                    .status()
                    .await?;
            }
        } else {
            sleep(Duration::from_secs(1)).await;
        }
    }

    Ok(())
}

// pub async fn query_submissions(title_slug: &str) -> Result<_, anyhow::Error> {
//   let mut variables = HashMap::new();
//   variables.insert("questionSlug".to_owned(), title_slug);
//   variables.insert("offset".to_owned(), "0");
//   variables.insert("limit".to_owned(), "40");

//   graphql(GraphqlBody {
//     operation_name: "submissions".to_owned(),
//     variables,
//     query:
//   }).await
// }



#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CheckSubmissionsResponse {
    /// PENDING, SUCCESS
    state: String,
    status_msg: Option<String>,
    submission_id: String,
    status_runtime: String,
    status_memory: String,
}

pub async fn check_submissions(submit_id: &str) -> Result<CheckSubmissionsResponse, anyhow::Error> {
    let builder = surf::get(format!(
        "https://leetcode.cn/submissions/detail/{submit_id}/check/"
    ));

    let res = builder.recv_string().await.unwrap();

    println!("res {:?}", res);

    Ok(serde_json::from_str(&res).unwrap())
}

async fn read_content<P: AsRef<Path>>(file: P) -> Result<(String, String), anyhow::Error> {
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

    #[test]
    fn test_check() {
        async_std::task::block_on(async {
            let c = check_submissions("329320745").await.unwrap();
            println!("{:?}", c);
        })
    }
}

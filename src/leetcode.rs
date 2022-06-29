use std::collections::HashMap;

use anyhow::Result;

use crate::domain::Question;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphqlBody<'a> {
    #[serde(rename = "operationName")]
    pub operation_name: &'a str,
    pub variables: HashMap<&'a str, &'a str>,
    pub query: &'a str,
}

pub async fn graphql(body: &GraphqlBody<'_>) -> Result<surf::Response> {
    let builder = surf::post("https://leetcode-cn.com/graphql/");

    let res = builder.body_json(&body).unwrap().await.unwrap();

    Ok(res)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SubmitResponse {
    SUCCESS { submission_id: usize },
    ERROR { error: String },
}

pub async fn submit(question: &Question, code: &str) -> Result<SubmitResponse> {
    // let _ = match question.code_snippets.iter().find(|c| c.lang == "Rust") {
    //     Some(snippet) => snippet,
    //     None => bail!("Fail to get Rust code Snippet"),
    // };

    let title_slug = &question.title_slug;
    let url = format!("https://leetcode.cn/problems/{title_slug}/submit/");

    let res = surf::post(url)
        .body_json(&serde_json::json!({
            "lang": "rust",
            "questionSlug": question.title_slug,
            "question_id": question.question_id,
            "test_judger": "",
            "test_mode": false,
            "typed_code": code,
        }))
        .unwrap()
        .header("cookie", std::env::var("COOKIE").unwrap())
        .header(
            "Referer",
            format!("https://leetcode.cn/problems/{title_slug}/"),
        )
        .recv_string()
        .await
        .unwrap();

    log::trace!("res {:?}", res);

    Ok(serde_json::from_str(&res).unwrap())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "state")]
pub enum CheckSubmissionsResponse {
    STARTED,
    PENDING,
    SUCCESS {
        status_code: usize,
        status_msg: String,
        submission_id: String,
        status_runtime: String,
        status_memory: String,
        memory_percentile: f32,
        runtime_percentile: f32,
        total_testcases: usize,
        task_name: String,
    },
}

pub async fn check_submissions(submit_id: usize) -> Result<CheckSubmissionsResponse> {
    let builder = surf::get(format!(
        "https://leetcode.cn/submissions/detail/{submit_id}/check/"
    ));

    let res = builder.recv_string().await.unwrap();

    log::trace!("res {:?}", res);

    Ok(serde_json::from_str(&res).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check() {
        async_std::task::block_on(async {
            let c = check_submissions(329320745).await.unwrap();
            println!("{:?}", c);
        })
    }
}

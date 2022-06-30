use anyhow::{Ok, Result};

use crate::domain::Question;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphqlBody<'a> {
    #[serde(rename = "operationName")]
    pub operation_name: Option<&'a str>,
    pub variables: serde_json::Value,
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestionRecord {
    date: String,
    question: Option<QuestionRecordQuestion>,
    userStatus: QuestionRecordStatus,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestionRecordQuestion {
    questionFrontendId: String,
    title: String,
    titleSlug: String,
    translatedTitle: String,
    lastSubmission: Option<String>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum QuestionRecordStatus {
    #[serde(rename = "NOT_START")]
    NotStart,
    #[serde(rename = "FINISH")]
    Finish,
}

pub async fn check_submissions(submit_id: usize) -> Result<CheckSubmissionsResponse> {
    let builder = surf::get(format!(
        "https://leetcode.cn/submissions/detail/{submit_id}/check/"
    ));

    let res = builder.recv_string().await.unwrap();

    log::trace!("res {:?}", res);

    Ok(serde_json::from_str(&res).unwrap())
}

pub async fn daily_question_records(month: usize, year: usize) -> Result<Vec<QuestionRecord>> {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct DailyQuestionRecordsWrapper {
        dailyQuestionRecords: Vec<QuestionRecord>,
    }

    let mut resp = graphql(&GraphqlBody {
        query: "\n    query dailyQuestionRecords($year: Int!, $month: Int!) {\n  dailyQuestionRecords(year: $year, month: $month) {\n    date\n    userStatus\n    question {\n      questionFrontendId\n      title\n      titleSlug\n      translatedTitle\n    }\n  }\n}\n    ",
        variables: serde_json::json!({
            "month": month,
            "year": year,
        }),
        operation_name: None,
    }).await?;

    let data: Response<DailyQuestionRecordsWrapper> = resp.body_json().await.unwrap();

    Ok(data.data.dailyQuestionRecords)
}

pub async fn question_of_today() -> Result<Vec<QuestionRecord>> {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct ResponseWrapper {
        todayRecord: Vec<QuestionRecord>,
    }

    let mut resp = graphql(&GraphqlBody {
        query: "\n    query questionOfToday {\n  todayRecord {\n    date\n    userStatus\n    question {\n      questionId\n      frontendQuestionId: questionFrontendId\n      difficulty\n      title\n      titleCn: translatedTitle\n      titleSlug\n      paidOnly: isPaidOnly\n      freqBar\n      isFavor\n      acRate\n      status\n      solutionNum\n      hasVideoSolution\n      topicTags {\n        name\n        nameTranslated: translatedName\n        id\n      }\n      extra {\n        topCompanyTags {\n          imgUrl\n          slug\n          numSubscribed\n        }\n      }\n    }\n    lastSubmission {\n      id\n    }\n  }\n}\n    ",
        variables: serde_json::json!({}),
        operation_name: None,
    }).await?;

    let data: Response<ResponseWrapper> = resp.body_json().await.unwrap();

    Ok(data.data.todayRecord)
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

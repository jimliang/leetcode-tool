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

pub async fn submit(question: &Question, code: &str, cookie: &str) -> Result<SubmitResponse> {
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
        // .header("cookie", std::env::var("COOKIE").unwrap())
        .header("cookie", cookie)
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
        memory_percentile: Option<f32>,
        runtime_percentile: Option<f32>,
        // total_correct: Option<usize>,
        // total_testcases: Option<usize>,
        // task_name: String,
        compile_error: Option<String>,
        full_compile_error: Option<String>,
        last_testcase: Option<String>,
        expected_output: Option<String>,
        code_output: Option<String>,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestionRecord {
    pub date: String,
    pub question: Option<QuestionRecordQuestion2>,
    pub userStatus: Option<QuestionRecordStatus>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestionRecordQuestion {
    pub questionFrontendId: String,
    pub title: String,
    pub titleSlug: String,
    pub translatedTitle: String,
    pub lastSubmission: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestionRecordQuestion2 {
    pub questionId: String,
    pub frontendQuestionId: String,
    pub title: String,
    pub titleCn: String,
    pub titleSlug: String,
    pub lastSubmission: Option<String>,
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
    #[allow(non_snake_case)]
    struct ResponseWrapper {
        todayRecord: Vec<QuestionRecord>,
    }

    let mut resp = graphql(&GraphqlBody {
        query: "\n    query questionOfToday {\n  todayRecord {\n    date\n    userStatus\n    question {\n      questionId\n      frontendQuestionId: questionFrontendId\n      difficulty\n      title\n      titleCn: translatedTitle\n      titleSlug\n      paidOnly: isPaidOnly\n      freqBar\n      isFavor\n      acRate\n      status\n      solutionNum\n      hasVideoSolution\n      topicTags {\n        name\n        nameTranslated: translatedName\n        id\n      }\n      extra {\n        topCompanyTags {\n          imgUrl\n          slug\n          numSubscribed\n        }\n      }\n    }\n    lastSubmission {\n      id\n    }\n  }\n}\n    ",
        variables: serde_json::json!({}),
        operation_name: None,
    }).await?;

    let res = resp.body_string().await.unwrap();

    log::trace!("res {res}");

    let data: Response<ResponseWrapper> = serde_json::from_str(&res).unwrap();

    Ok(data.data.todayRecord)
}

pub async fn random_question() -> Result<String> {
    let mut res = graphql(&GraphqlBody {
        operation_name: None,
        variables: serde_json::json!({
            "categorySlug": "",
            "filters": {},
        }),
        query: include_str!("gql/random.gql"),
    })
    .await?;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    #[allow(non_snake_case)]
    struct ResponseWrapper {
        problemsetRandomFilteredQuestion: String,
    }

    let res = res.body_string().await.unwrap();

    // println!("{:?}", res_string);

    let data: Response<ResponseWrapper> = serde_json::from_str(&res).unwrap();

    Ok(data.data.problemsetRandomFilteredQuestion)
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

    #[test]
    fn test_parse() {
        let s = "{\"status_code\": 20, \"lang\": \"rust\", \"run_success\": false, \"compile_error\": \"Line 35, Char 35: use of unstable library feature 'int_abs_diff' (solution.rs)\", \"full_compile_error\": \"Line 35, Char 35: use of unstable library feature 'int_abs_diff' (solution.rs)\\n   |\\n35 |             let min_val = list[0].abs_diff(list[1]);\\n   |                                   ^^^^^^^^\\n   |\\n   = note: see issue #89492 <https://github.com/rust-lang/rust/issues/89492> for more information\\nFor more information about this error, try `rustc --explain E0658`.\\nerror: could not compile `prog` due to previous error\\nmv: cannot stat '/leetcode/rust_compile/target/release/prog': No such file or directory\", \"status_runtime\": \"N/A\", \"memory\": 0, \"question_id\": \"1306\", \"task_finish_time\": 1656920071929, \"elapsed_time\": 0, \"task_name\": \"judger.judgetask.Judge\", \"finished\": true, \"status_msg\": \"Compile Error\", \"state\": \"SUCCESS\", \"fast_submit\": false, \"total_correct\": null, \"total_testcases\": null, \"submission_id\": \"332499557\", \"runtime_percentile\": null, \"status_memory\": \"N/A\", \"memory_percentile\": null, \"pretty_lang\": \"Rust\"}";

        let ss: CheckSubmissionsResponse = serde_json::from_str(s).unwrap();

        println!("{:?}", ss);
    }

    #[test]
    fn test_today() {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Trace)
            .try_init()
            .unwrap();
        async_std::task::block_on(async {
            let c = question_of_today().await.unwrap();
            println!("{:?}", c);
        })
    }
    #[test]
    fn test_random() {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Trace)
            .try_init()
            .unwrap();
        async_std::task::block_on(async {
            let c = random_question().await.unwrap();
            println!("{}", c);
        })
    }
}

use std::collections::HashMap;

use crate::{domain::Question, errors::Result};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct QuestionWrapper {
    question: Question,
}

pub async fn graphql<T: serde::de::DeserializeOwned>(body: GraphqlBody) -> Result<T> {
    // let headers = serde_json::json!({
    //   "accept": "*/*",
    //   "accept-language": "zh-CN",
    //   "content-type": "application/json",
    //   "sec-fetch-dest": "empty",
    //   "sec-fetch-mode": "cors",
    //   "sec-fetch-site": "same-origin",
    //   "x-definition-name": "question",
    //   "x-operation-name": "questionData",
    //   "x-timezone": "Asia/Shanghai",
    //   "referrer": "https://leetcode-cn.com/",
    // });
    let mut builder = surf::post("https://leetcode-cn.com/graphql/");

    // if let serde_json::Value::Object(map) = headers {
    //     for (k, v) in map {
    //         builder = builder.header(k.as_str(), v.to_string());
    //     }
    // }

    let res = builder.body_json(&body).unwrap().await;

    // println!("--> res {:?}", res);

    match res {
        Ok(mut resp) => {
            let r: Response<T> = resp.body_json().await.expect("resp parse error");
            Ok(r.data)
            // let r = resp.body_string().await;
            // println!("error: {:?}", r);
            // Err(crate::errors::Error::Empty)
        }
        Err(err) => panic!("http graphql error: {:?}", err),
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphqlBody {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: HashMap<String, String>,
    query: String,
}

pub async fn fetch_question(title_slug: String) -> Result<Question> {
    let mut variables = HashMap::new();
    variables.insert("titleSlug".to_owned(), title_slug);
    let qu: QuestionWrapper = graphql(GraphqlBody {
        operation_name: "questionData".to_owned(),
        variables,
        query: "query questionData($titleSlug: String!) {\n question(titleSlug: $titleSlug) {\n questionId\n questionFrontendId\n boundTopicId\n title\n titleSlug\n content\n translatedTitle\n translatedContent\n isPaidOnly\n difficulty\n likes\n dislikes\n isLiked\n similarQuestions\n contributors {\n username\n profileUrl\n avatarUrl\n __typename\n }\n langToValidPlayground\n topicTags {\n name\n slug\n translatedName\n __typename\n }\n companyTagStats\n codeSnippets {\n lang\n langSlug\n code\n __typename\n }\n stats\n hints\n solution {\n id\n canSeeDetail\n __typename\n }\n status\n sampleTestCase\n metaData\n judgerAvailable\n judgeType\n mysqlSchemas\n enableRunCode\n envInfo\n book {\n id\n bookName\n pressName\n source\n shortDescription\n fullDescription\n bookImgUrl\n pressImgUrl\n productUrl\n __typename\n }\n isSubscribed\n isDailyQuestion\n dailyRecordStatus\n editorType\n ugcQuestionId\n style\n exampleTestcases\n __typename\n }\n}\n".to_owned()
    }).await?;

    Ok(qu.question)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fetch_question() {
        async_std::task::block_on(async {
            let question = fetch_question("find-and-replace-pattern".to_owned()).await;
            println!("question: {:?}", question.unwrap());
        })
    }
}

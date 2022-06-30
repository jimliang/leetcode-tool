use std::{collections::HashMap, env::current_dir};

use anyhow::Ok;
use async_std::{
    fs::{create_dir_all, File},
    io::WriteExt,
    path::PathBuf,
};

use crate::{
    domain::Question,
    errors::Result,
    leetcode::{graphql, GraphqlBody, Response},
};

pub async fn get_backup_file(title_slug: &str) -> Result<PathBuf> {
    let title_slug = title_slug.to_owned();
    let p = async_std::task::spawn_blocking(move || {
        let mut current_dir = current_dir().unwrap();

        current_dir.push(format!(".backup/{title_slug}.json"));

        current_dir
    })
    .await;

    Ok(p.into())
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuestionWrapper {
    pub question: Question,
}

pub async fn fetch_question(title_slug: &str) -> Result<Question> {
    let cache_file = get_backup_file(title_slug).await?;

    if cache_file.exists().await {
        // FIXME: sync api
        let reader = std::fs::File::open(cache_file)?;
        let res: Response<QuestionWrapper> = serde_json::from_reader(reader)?;
        return Ok(res.data.question);
    };

    let mut res = graphql(&GraphqlBody {
        operation_name: Some("questionData"),
        variables: serde_json::json!({
            "titleSlug": title_slug
        }),
        query: "query questionData($titleSlug: String!) {\n question(titleSlug: $titleSlug) {\n questionId\n questionFrontendId\n boundTopicId\n title\n titleSlug\n content\n translatedTitle\n translatedContent\n isPaidOnly\n difficulty\n likes\n dislikes\n isLiked\n similarQuestions\n contributors {\n username\n profileUrl\n avatarUrl\n __typename\n }\n langToValidPlayground\n topicTags {\n name\n slug\n translatedName\n __typename\n }\n companyTagStats\n codeSnippets {\n lang\n langSlug\n code\n __typename\n }\n stats\n hints\n solution {\n id\n canSeeDetail\n __typename\n }\n status\n sampleTestCase\n metaData\n judgerAvailable\n judgeType\n mysqlSchemas\n enableRunCode\n envInfo\n book {\n id\n bookName\n pressName\n source\n shortDescription\n fullDescription\n bookImgUrl\n pressImgUrl\n productUrl\n __typename\n }\n isSubscribed\n isDailyQuestion\n dailyRecordStatus\n editorType\n ugcQuestionId\n style\n exampleTestcases\n __typename\n }\n}\n"
    }).await?;

    let body_bytes = res.body_bytes().await.unwrap();

    create_dir_all(cache_file.parent().unwrap()).await?;
    let mut f = File::create(cache_file).await?;
    let _ = f.write(&body_bytes).await?;

    let qu: Response<QuestionWrapper> = serde_json::from_slice(&body_bytes).unwrap();

    Ok(qu.data.question)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fetch_question() {
        async_std::task::block_on(async {
            let question = fetch_question("find-and-replace-pattern").await;
            println!("question: {:?}", question.unwrap());
        })
    }
}

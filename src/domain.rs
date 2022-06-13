#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
pub struct Question {
    #[serde(rename = "questionId")]
    pub question_id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<CodeSnippet>,
    #[serde(rename = "translatedTitle")]
    pub translated_title: String,
    #[serde(rename = "translatedContent")]
    pub translated_content: String,
    #[serde(rename = "hints")]
    pub hints: Vec<String>,
    #[serde(rename = "metaData")]
    pub meta_data: String,
    #[serde(rename = "exampleTestcases")]
    pub example_testcases: Option<String>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodeSnippet {
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
    // pub __typename: String,
}

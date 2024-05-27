use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ContentResponse {
    pub data: QuestionData,
}

#[derive(Deserialize, Debug)]
pub struct QuestionData {
    pub question: Question,
}

#[derive(Deserialize, Debug)]
pub struct Question {
    pub content: String,
    #[serde(rename = "dataSchemas")]
    pub data_schemas: Vec<String>,

    #[serde(rename = "mysqlSchemas")]
    pub mysql_shemas: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct EditorResponnse {
    pub data: EditorData,
}

#[derive(Deserialize, Debug)]
pub struct EditorData {
    question: EditorQuestion,
}

#[derive(Deserialize, Debug)]
pub struct EditorQuestion {
    #[serde(rename = "questionId")]
    pub question_id: String,

    #[serde(rename = "questionFrontendId")]
    pub question_frontend_id: String,

    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<CodeSnippet>,

    #[serde(rename = "envInfo")]
    pub env_info: String,

    #[serde(rename = "enableRunCode")]
    pub enable_run_code: bool,

    #[serde(rename = "hasFrontendPreview")]
    pub has_frontend_preview: bool,

    #[serde(rename = "frontendPreviews")]
    pub frontend_previews: String,
}

#[derive(Deserialize, Debug)]
pub struct CodeSnippet {
    pub lang: String,

    #[serde(rename = "langSlug")]
    pub lang_slug: String,

    pub code: String,
}

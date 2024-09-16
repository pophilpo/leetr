use serde::Deserialize;
// Response types for 'questionContent' query
#[derive(Deserialize, Debug)]
pub struct QuestionContentResponse {
    data: QuestionContentData,
}

#[derive(Deserialize, Debug)]
pub struct QuestionContentData {
    question: Option<QuestionContent>,
}

#[derive(Deserialize, Debug)]
pub struct QuestionContent {
    content: Option<String>,
}

// Response types for 'questionEditorData' query
#[derive(Deserialize, Debug)]
pub struct QuestionEditorDataResponse {
    data: QuestionEditorData,
}

#[derive(Deserialize, Debug)]
pub struct QuestionEditorData {
    question: Option<QuestionEditor>,
}

#[derive(Deserialize, Debug)]
pub struct QuestionEditor {
    #[serde(rename = "codeSnippets")]
    code_snippets: Vec<CodeSnippet>,
    #[serde(rename = "exampleTestcaseList")]
    example_testcase_list: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CodeSnippet {
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}

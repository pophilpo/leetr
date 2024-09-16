use crate::errors::ContentResponseError;
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

impl QuestionEditorDataResponse {
    pub fn get_code_snippet(&self, lang: &str) -> Result<String, ContentResponseError> {
        let code_snippet = self
            .data
            .question
            .as_ref()
            .ok_or(ContentResponseError::MissingQuestionDataError)?
            .code_snippets
            .iter()
            .find(|snippet| snippet.lang_slug == lang)
            .ok_or(ContentResponseError::LangCodeSnippetNotFoundError)?
            .code
            .clone();

        Ok(code_snippet)
    }
}
#[derive(Deserialize, Debug)]
pub struct ConsolePanelConfigResponse {
    pub data: ConsolePanelConfigData,
}

#[derive(Deserialize, Debug)]
pub struct ConsolePanelConfigData {
    pub question: Option<ConsolePanelConfigQuestion>,
}

#[derive(Deserialize, Debug)]
pub struct ConsolePanelConfigQuestion {
    #[serde(rename = "exampleTestcaseList")]
    pub example_testcase_list: Vec<String>,
}

impl ConsolePanelConfigResponse {
    // Helper method to get example test cases
    pub fn get_example_testcases(&self) -> Result<Vec<String>, ContentResponseError> {
        let test_cases = self
            .data
            .question
            .as_ref()
            .ok_or(ContentResponseError::MissingQuestionDataError)?
            .example_testcase_list
            .clone();
        Ok(test_cases)
    }
}

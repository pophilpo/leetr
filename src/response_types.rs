use serde::{de::Error, Deserialize};

use crate::errors::GetResponseError;

#[derive(Debug)]
pub enum Response {
    ContentResponse(ContentResponse),
    EditorResponse(EditorResponse),
}
impl Response {
    pub async fn from_response(response: reqwest::Response) -> Result<Self, GetResponseError> {
        let body = response.text().await?;

        if let Ok(content_response) = serde_json::from_str::<ContentResponse>(&body) {
            return Ok(Response::ContentResponse(content_response));
        }

        if let Ok(editor_response) = serde_json::from_str::<EditorResponse>(&body) {
            return Ok(Response::EditorResponse(editor_response));
        }

        Err(GetResponseError::ParseError(serde_json::Error::custom(
            "Failed to deserialize response",
        )))
    }

    pub fn get_content(&self, lang: String) -> Option<String> {
        // Get markdown or code text based on the response type

        match self {
            Response::ContentResponse(content_response) => {
                Some(content_response.data.question.content.clone())
            }
            Response::EditorResponse(editor_response) => {
                let code_snippets = &editor_response.data.question.code_snippets;
                code_snippets
                    .iter()
                    .find(|snippet| snippet.lang == lang)
                    .map(|snippet| snippet.code.clone())
            }
        }
    }
}

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
pub struct EditorResponse {
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

use reqwest::blocking::Response as ReqwestResponse;
use serde::{de::Error, Deserialize};

use crate::errors::GetResponseError;

#[derive(Debug)]
pub enum Response {
    Content(ContentResponse),
    Editor(EditorResponse),
    ProblemSet(ProblemSetResponse),
}

pub trait ResponseHandler {
    fn parse_response(response: ReqwestResponse) -> Result<Response, GetResponseError>;
}

impl ResponseHandler for ContentResponse {
    fn parse_response(response: ReqwestResponse) -> Result<Response, GetResponseError> {
        let body = response.text()?;

        if let Ok(content_response) = serde_json::from_str::<ContentResponse>(&body) {
            Ok(Response::Content(content_response))
        } else {
            Err(GetResponseError::ParseError(serde_json::Error::custom("Failed to deserialize response")))
        }
    }
}

impl ResponseHandler for EditorResponse {
    fn parse_response(response: ReqwestResponse) -> Result<Response, GetResponseError> {
        let body = response.text()?;

        if let Ok(editor_response) = serde_json::from_str::<EditorResponse>(&body) {
            Ok(Response::Editor(editor_response))
        } else {
            Err(GetResponseError::ParseError(serde_json::Error::custom("Failed to deserialize response")))
        }
    }
}


impl ResponseHandler for ProblemSetResponse {
    fn parse_response(response: ReqwestResponse) -> Result<Response, GetResponseError> {
        let body = response.text()?;

        if let Ok(problem_set_response) = serde_json::from_str::<ProblemSetResponse>(&body) {
            Ok(Response::ProblemSet(problem_set_response))
        } else {
            Err(GetResponseError::ParseError(serde_json::Error::custom("Failed to deserialize response")))
        }
    }
}


impl Response {
    pub fn get_content(&self, lang: String) -> Option<String> {
        // Get markdown or code text based on the response type

        match self {
            Response::Content(content_response) => {
                Some(content_response.data.question.content.clone())
            }
            Response::Editor(editor_response) => {
                let code_snippets = &editor_response.data.question.code_snippets;
                code_snippets
                    .iter()
                    .find(|snippet| snippet.lang == lang)
                    .map(|snippet| snippet.code.clone())
            }
            Response::ProblemSet(problem_set) => Some(
                problem_set
                    .data
                    .problem_set_question_list
                    .questions
                    .iter()
                    .map(|question| question.title_slug.as_str())
                    .collect::<Vec<&str>>()
                    .join("\n"),
            ),
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
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<CodeSnippet>,
}

#[derive(Deserialize, Debug)]
pub struct CodeSnippet {
    pub lang: String,
    pub code: String,
}

#[derive(Deserialize, Debug)]
pub struct ProblemSetResponse {
    data: ProblemSetData,
}

#[derive(Deserialize, Debug)]
pub struct ProblemSetData {
    #[serde(rename = "problemsetQuestionList")]
    problem_set_question_list: ProblemSetQuestionList,
}

#[derive(Deserialize, Debug)]
pub struct ProblemSetQuestionList {
    questions: Vec<ProblemSetQuestion>,
}

#[derive(Deserialize, Debug)]
pub struct ProblemSetQuestion {
    #[serde(rename = "titleSlug")]
    title_slug: String,
}

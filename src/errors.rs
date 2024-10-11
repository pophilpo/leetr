use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LeetCodeApiError {
    #[error("LeetCodeApiError [GenerateHeader]: {0}")]
    GenerateHeaderError(#[from] InvalidHeaderValue),

    #[error("LeetCodeApiError [ReqwestError]: {0}")]
    ReqwestError(#[from] ReqwestError),
}

#[derive(Error, Debug)]
pub enum ContentResponseError {
    #[error("ContentResponseError [MissingQuestionData]")]
    MissingQuestionDataError,

    #[error("ContentResponseError [LangCodeSnippetNotFound]")]
    LangCodeSnippetNotFoundError,
}

#[derive(Error, Debug)]
pub enum ProjectGeneratorError {
    #[error("ProjectGeneratorError [Parse]")]
    ParseError(#[from] serde_json::error::Error),

    #[error("ProjectGeneratorError [Process]")]
    ProcessError(#[from] std::io::Error),
}

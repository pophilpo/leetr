use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenerateMarkdownError {
    #[error("Generate Markdown Error [Io]: {0}")]
    Io(#[from] io::Error),
}

#[derive(Error, Debug)]
pub enum GetResponseError {
    #[error("GetResponseError [Request]: {0}")]
    RequestError(#[from] ReqwestError),

    #[error("GetResponse Error [Parse]: {0}")]
    ParseError(#[from] SerdeError),
}

#[derive(Error, Debug)]
pub enum ProjectGeneratorError {
    #[error(transparent)]
    GenerateMarkdown(#[from] GenerateMarkdownError),

    #[error(transparent)]
    GetResponse(#[from] GetResponseError),

    #[error("ProjectGeneratorError [Command]: {0}")]
    Command(#[from] io::Error),

    #[error("ProjectGeneratorError [Title Extraction]: {0}")]
    TitleExtractionError(String),
}

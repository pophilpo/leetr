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
pub enum GetResponse {
    #[error("GetResponse Error [Request]: {0}")]
    RequestError(#[from] ReqwestError),

    #[error("GetResponse Error [Parse]: {0}")]
    ParseError(#[from] SerdeError),
}

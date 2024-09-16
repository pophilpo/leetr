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

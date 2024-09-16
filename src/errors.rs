use quote;
use reqwest::header::InvalidHeaderValue;
use reqwest::Error as ReqwestError;
use syn::Error as SynError;
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
pub enum TestGenerationError {
    // Error when the test case format is invalid (e.g., missing input or output)
    #[error("TestGenerationError [InvalidTestCaseFormat]: {0}")]
    InvalidTestCaseFormatError(String),

    // Error when no functions are found in the parsed Rust code
    #[error("TestGenerationError [NoFunctionsFound]")]
    NoFunctionsFoundError,

    // Error during the parsing of the Rust code snippet using `syn`
    #[error("TestGenerationError [SynError]: {0}")]
    SynError(#[from] syn::Error),

    // Error when the number of input arguments in test cases doesn't match the function signature
    #[error("TestGenerationError [InvalidInputFormat]: {0}")]
    InvalidInputFormatError(String),

    // Error when parsing input values in the test case generation fails
    #[error("TestGenerationError [ParseError]")]
    ParseError,
}

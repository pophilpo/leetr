use std::io;

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use syn::Error as SynError;
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
    #[error("🚫 Directory '{0}' already exists, please use another name.")]
    DirectoryExists(String),

    #[error(transparent)]
    GenerateMarkdown(#[from] GenerateMarkdownError),

    #[error(transparent)]
    GetResponse(#[from] GetResponseError),

    #[error("ProjectGeneratorError [Command]: {0}")]
    Command(#[from] io::Error),

    #[error("ProjectGeneratorError [Title Extraction]: {0}")]
    TitleExtractionError(String),

    #[error("🚫 Language '{0}' is not supported yet, please select one from [{1}].")]
    LanguageSupportIsNotAvailable(String, String),
}

#[derive(Error, Debug)]
pub enum ExampleParsingError {
    #[error("Could not find the first example to parse")]
    CouldNotFindExample,

    #[error("Could not find Constraints section to stop the parsing")]
    CouldNotFindConstraints,
}

#[derive(Error, Debug)]
pub enum CodeParserError {
    #[error("Could not read file: {0}")]
    CouldNotReadFile(#[from] SynError),
}
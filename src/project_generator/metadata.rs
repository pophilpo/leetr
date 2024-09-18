use serde::Deserialize;
use serde_json::Value;

use log::{debug, error, info, warn};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Metadata {
    Class(ClassMetadata),
    Function(FunctionMetadata),
}

#[derive(Deserialize, Debug)]
pub struct FunctionMetadata {
    pub name: Option<String>, // Handle functions without names
    pub params: Vec<Param>,
    #[serde(rename = "return")]
    pub return_type: ReturnType, // Handle optional return types
    #[serde(flatten)] // Allow extra fields like "manual"
    pub extra: Option<Value>,
}

#[derive(Deserialize, Debug)]
pub struct ClassMetadata {
    pub classname: String,
    pub constructor: Option<Constructor>,
    pub methods: Vec<Method>,
    #[serde(flatten)] // Handle extra fields like maxbytesperline and systemdesign
    pub extra: Option<Value>,
}

#[derive(Deserialize, Debug)]
pub struct Param {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(Deserialize, Debug)]
pub struct ReturnType {
    #[serde(rename = "type")]
    pub return_type: String,
    #[serde(flatten)] // Allow extra fields inside the return type, like "size"
    pub extra: Option<Value>,
}

#[derive(Deserialize, Debug)]
pub struct Constructor {
    pub params: Vec<Param>,
}

#[derive(Deserialize, Debug)]
pub struct Method {
    pub name: String,
    pub params: Vec<Param>,
    #[serde(rename = "return")]
    pub return_type: ReturnType,
}

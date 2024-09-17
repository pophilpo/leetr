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
struct FunctionMetadata {
    name: Option<String>, // Handle functions without names
    params: Vec<Param>,
    #[serde(rename = "return")]
    return_type: Option<ReturnType>, // Handle optional return types
    #[serde(flatten)] // Allow extra fields like "manual"
    extra: Option<Value>,
}

#[derive(Deserialize, Debug)]
struct ClassMetadata {
    classname: String,
    constructor: Option<Constructor>,
    methods: Vec<Method>,
    #[serde(flatten)] // Handle extra fields like maxbytesperline and systemdesign
    extra: Option<Value>,
}

#[derive(Deserialize, Debug)]
struct Param {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
}

#[derive(Deserialize, Debug)]
struct ReturnType {
    #[serde(rename = "type")]
    return_type: String,
    #[serde(flatten)] // Allow extra fields inside the return type, like "size"
    extra: Option<Value>,
}

#[derive(Deserialize, Debug)]
struct Constructor {
    params: Vec<Param>,
}

#[derive(Deserialize, Debug)]
struct Method {
    name: String,
    params: Vec<Param>,
    #[serde(rename = "return")]
    return_type: ReturnType,
}

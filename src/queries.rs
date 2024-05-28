use reqwest::Client;
use serde::Serialize;
use serde_json;

use crate::errors::GetResponseError;
use crate::response_types::ContentResponse;
use crate::response_types::Response;

const QUESTION_CONTENT_QUERY: &str = r#"
    query questionContent($titleSlug: String!) {
        question(titleSlug: $titleSlug) {
            content
            mysqlSchemas
            dataSchemas
        }
    }
    "#;

const QUESTION_EDITOR_DATA_QUERY: &str = r#"
    query questionEditorData($titleSlug: String!) {
        question(titleSlug: $titleSlug) {
            questionId
            questionFrontendId
            codeSnippets {
                lang
                langSlug
                code
            }
        envInfo
        enableRunCode
        hasFrontendPreview
        frontendPreviews
  }
}
    "#;

#[derive(Serialize, Debug)]
pub struct GraphQLPayload {
    query: String,
    variables: serde_json::Value,
}

impl GraphQLPayload {
    pub fn content_query(title: String) -> Self {
        let variables = serde_json::json!({
            "titleSlug": title
        });

        GraphQLPayload {
            query: QUESTION_CONTENT_QUERY.to_string(),
            variables,
        }
    }

    pub fn editor_data_query(title: String) -> Self {
        let variables = serde_json::json!({
            "titleSlug": title
        });

        GraphQLPayload {
            query: QUESTION_EDITOR_DATA_QUERY.to_string(),
            variables,
        }
    }

    pub async fn get_response(&self) -> Result<Response, GetResponseError> {
        let client = Client::new();
        let response = client
            .post("https://leetcode.com/graphql")
            .header("Content-Type", "application/json")
            .json(&self)
            .send()
            .await?;

        Response::from_response(response).await
    }
}

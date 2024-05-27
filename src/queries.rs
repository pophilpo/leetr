use serde_json;
use reqwest::Client;
use serde::Serialize;

use crate::response_types::ContentResponse;
use crate::errors::GetResponseError;

const QUESTION_TITLE_QUERY: &str = r#"
    query consolePanelConfig($titleSlug: String!) {
        question(titleSlug: $titleSlug) {
            questionId
            questionFrontendId
            questionTitle
            enableDebugger
            enableRunCode
            enableSubmit
            enableTestMode
            exampleTestcaseList
            metaData
        }
    }
    "#;

const QUESTION_CONTENT_QUERY: &str = r#"
    query questionContent($titleSlug: String!) {
        question(titleSlug: $titleSlug) {
            content
            mysqlSchemas
            dataSchemas
        }
    }
    "#;

const CONSOLE_PANEL_CONFIG_QUERY: &str = r#"
    query consolePanelConfig($titleSlug: String!) {
        question(titleSlug: $titleSlug) {
            questionId
            questionFrontendId
            questionTitle
            enableDebugger
            enableRunCode
            enableSubmit
            enableTestMode
            exampleTestcaseList
            metaData
        }
    }
    "#;

#[derive(Serialize, Debug)]
pub struct GraphQLPayload {
    query: String,
    variables: serde_json::Value,
}

impl GraphQLPayload {
    pub fn config_query(title: String) -> Self {
        let variables = serde_json::json!({
            "titleSlug": title
        });

        GraphQLPayload {
            query: CONSOLE_PANEL_CONFIG_QUERY.to_string(),
            variables,
        }
    }

    pub fn content_query(title: String) -> Self {
        let variables = serde_json::json!({
            "titleSlug": title
        });

        GraphQLPayload {
            query: QUESTION_CONTENT_QUERY.to_string(),
            variables,
        }
    }

    pub fn title_query(title: String) -> Self {
        let variables = serde_json::json!({
            "titleSlug": title
        });

        GraphQLPayload {
            query: QUESTION_TITLE_QUERY.to_string(),
            variables,
        }
    }

    pub async fn get_response(&self) -> Result<ContentResponse, GetResponseError> {

        let client = Client::new();
        let response = client.post("https://leetcode.cfffffom/graphql").header("Content-Type", "application/json").json(&self).send().await?;
        Ok(response.json::<ContentResponse>().await?)

    }
}

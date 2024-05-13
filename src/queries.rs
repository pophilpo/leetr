use serde::{Deserialize, Serialize};
use serde_json;

use crate::response_types::ContentResponse;

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

#[derive(Serialize)]
pub struct GraphQLPayload {
    query: String,
    variables: serde_json::Value,
    endponit: String,
}

impl GraphQLPayload {
    pub fn config_query(title: String) -> Self {
        let variables = serde_json::json!({
            "titleSlug": title
        });

        GraphQLPayload {
            query: CONSOLE_PANEL_CONFIG_QUERY.to_string(),
            variables,
            endponit: String::from("https://leetcode.com/graphql"),
        }
    }

    pub fn content_query(title: String) -> Self {
        let variables = serde_json::json!({
            "titleSlug": title
        });

        GraphQLPayload {
            query: QUESTION_CONTENT_QUERY.to_string(),
            variables,
            endponit: String::From("https://leetcode.com/graphql"),
        }
    }

    pub fn title_query(title: String) -> Self {
        let variables = serde_json::json!({
            "titleSlug": title
        });

        GraphQLPayload {
            query: QUESTION_TITLE_QUERY.to_string(),
            variables,
            endponit: String::from("https://leetcode.com/graphql"),
        }
    }

    pub async fn get_response(&self, client: &reqwest::Client) -> Result<ContentResponse, Box<dyn std::error::Error>> {

        let response = client.post(&self.endponit).json(&self.query).send().await?;

        Ok(response.json::<ContentResponse>().await?)

    }
}

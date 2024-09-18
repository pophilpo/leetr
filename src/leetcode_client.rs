use crate::errors::LeetCodeApiError;
use crate::response_types::{
    ConsolePanelConfigResponse, ProblemSetResponse, QuestionContentResponse,
    QuestionEditorDataResponse,
};
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderValue},
};
use serde_json::{Map, Value};

use log::{debug, error, info, warn};

const QUESTION_CONTENT_QUERY: &str = include_str!("graphql/question_content.graphql");
const QUESTION_EDITOR_DATA_QUERY: &str = include_str!("graphql/question_editor_data.graphql");
const CONSOLE_PANEL_CONFIG_QUERY: &str = include_str!("graphql/console_panel_config.graphql");
const PROBLEMSET_QUERY: &str = include_str!("graphql/problemset_questionlist_query.graphql");

pub struct LeetCodeClient {
    pub client: Client,
    pub base_url: String,
    pub problem_title: String,
}

impl LeetCodeClient {
    pub fn new(problem_title: &str) -> Result<Self, LeetCodeApiError> {
        let mut headers = HeaderMap::new();
        let header_value = HeaderValue::from_str("leetr")?;
        headers.insert("User-Agent", header_value);

        let content_type = HeaderValue::from_str("application/json")?;
        headers.insert("Content-Type", content_type);

        let client = ClientBuilder::new().default_headers(headers).build()?;
        let base_url = String::from("https://leetcode.com/graphql");
        let problem_title = problem_title.to_owned();

        Ok(Self {
            client,
            base_url,
            problem_title,
        })
    }

    pub fn get_problem_set(&self) -> Result<ProblemSetResponse, LeetCodeApiError> {
        let variables = serde_json::json!({
            "categorySlug": "all-code-essentials",
            "limit": 10000,
            "filters": Map::<String, Value>::new(),
        });

        let request_body = serde_json::json!({
            "variables": variables,
            "query": PROBLEMSET_QUERY,
        });

        let response = self
            .client
            .post(&self.base_url)
            .json(&request_body)
            .send()?;

        let response_data: ProblemSetResponse = response.json()?;
        Ok(response_data)
    }

    pub fn get_problem_description(&self) -> Result<QuestionContentResponse, LeetCodeApiError> {
        let request_body = serde_json::json!({
            "variables": { "titleSlug": self.problem_title },
            "query": QUESTION_CONTENT_QUERY,
        });

        let response = self
            .client
            .post(&self.base_url)
            .json(&request_body)
            .send()?;
        let response_data: QuestionContentResponse = response.json()?;

        Ok(response_data)
    }

    pub fn get_editor_data(&self) -> Result<QuestionEditorDataResponse, LeetCodeApiError> {
        let request_body = serde_json::json!({
            "variables": { "titleSlug": self.problem_title },
            "query": QUESTION_EDITOR_DATA_QUERY,
        });
        let response = self
            .client
            .post(&self.base_url)
            .json(&request_body)
            .send()?;
        let response_data: QuestionEditorDataResponse = response.json()?;

        Ok(response_data)
    }

    pub fn get_console_panel_conifg(&self) -> Result<ConsolePanelConfigResponse, LeetCodeApiError> {
        let request_body = serde_json::json!({
            "variables": { "titleSlug": self.problem_title },
            "query": CONSOLE_PANEL_CONFIG_QUERY,
        });
        let response = self
            .client
            .post(&self.base_url)
            .json(&request_body)
            .send()?;

        let response_data: ConsolePanelConfigResponse = response.json()?;

        Ok(response_data)
    }
}

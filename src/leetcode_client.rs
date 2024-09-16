use crate::errors::LeetCodeApiError;
use crate::response_types::{QuestionContentResponse, QuestionEditorDataResponse};
use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderValue},
};

const QUESTION_CONTENT_QUERY: &str = include_str!("graphql/question_content.graphql");
const QUESTION_EDITOR_DATA_QUERY: &str = include_str!("graphql/question_editor_data.graphql");

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
        let client = ClientBuilder::new().default_headers(headers).build()?;
        let base_url = String::from("https://leetcode.com/graphql");
        let problem_title = problem_title.to_owned();

        Ok(Self {
            client,
            base_url,
            problem_title,
        })
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
}

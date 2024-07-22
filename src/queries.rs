use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::errors::GetResponseError;
use crate::response_types::{Response, ResponseHandler};

const PROBLEM_SET_QUERY: &str = r#"
    query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
        problemsetQuestionList: questionList(
            categorySlug: $categorySlug
            limit: $limit
            skip: $skip
            filters: $filters
  ) {
    questions: data {
      titleSlug
    }
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

    #[allow(dead_code)]
    pub fn problem_set_query() -> Self {
        let variables = serde_json::json!({
            "categorySlug": "all-code-essentials",
            "limit": 10000,
            "filters": Map::<String, Value>::new(),
        });

        Self {
            query: PROBLEM_SET_QUERY.to_string(),
            variables,
        }
    }

    pub fn get_response<T>(&self) -> Result<Response, GetResponseError>
    where
        T: ResponseHandler,
    {
        let client = Client::new();
        let response = client
            .post("https://leetcode.com/graphql")
            .header("Content-Type", "application/json")
            .json(&self)
            .send()?;

        T::parse_response(response)
    }
}

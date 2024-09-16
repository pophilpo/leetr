use reqwest::{Client, ClientBuilder, HeaderMap};

const QUESTION_CONTENT_QUERY: &str = include_str!("graphql/question_content.graphql");
const QUESTION_EDITOR_DATA_QUERY: &str = include_str!("graphql/question_editor_data.graphql");

pub struct LeetCodeClient {
    client: Client,
    base_url: String,
}

impl LeetCodeClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", "leetr");
        let client = ClientBuilder::new().default_headers(headers);
        let base_url = String::from("https://leetcode.com");

        Self { client, base_url }
    }
}

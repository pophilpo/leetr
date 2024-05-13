use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ContentResponse {
    pub data: QuestionData,
}

#[derive(Deserialize, Debug)]
pub struct QuestionData {
    pub question: Question,
}

#[derive(Deserialize, Debug)]
pub struct Question {
    pub content: String,
    #[serde(rename = "dataSchemas")]
    pub data_schemas: Vec<String>,

    #[serde(rename = "mysqlSchemas")]
    pub mysql_shemas: Vec<String>,
}

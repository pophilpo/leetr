mod client;
mod queries;
mod response_types;
mod html;
mod argument_parser;

use clap::Parser;


#[tokio::main]
async fn main() {

    let cli = argument_parser::Cli::parse();
    let title = cli.url.trim_end_matches('/').rsplit('/').nth(1).unwrap().to_string();
    let client = client::generate_client().unwrap();
    let query = queries::GraphQLPayload::content_query(title.clone());
    let content = query.get_response(&client).await.unwrap();
    html::generate_markdown(title, content).unwrap();

}

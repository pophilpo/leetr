mod argument_parser;
mod config;
mod errors;
mod html;
mod logger;
mod project_generator;
mod project_templates;
mod queries;
mod response_types;

use clap::Parser;
use std::process;

use config::Config;
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();
    let config = match Config::new() {
        Ok(project_config) => {
            // TODO: Dynamic path
            info!("Using config ~/.config/leetr/leetr.toml");
            project_config
        }
        Err(e) => {
            // TODO: Switch to default config
            error!("{}", e);
            process::exit(1);
        }
    };

    let cli = argument_parser::Cli::parse();
    let title = cli
        .url
        .trim_end_matches('/')
        .rsplit('/')
        .nth(1)
        .unwrap()
        .to_string();

    let query = queries::GraphQLPayload::editor_data_query(title);

    let response = query.get_response();

    Ok(())
}

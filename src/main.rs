mod queries;
mod response_types;
mod html;
mod argument_parser;
mod project_generator;
mod config;
mod project_templates;
mod errors;
mod logger;

use clap::Parser;
use std::process;

use config::Config;
use log::{info, error};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    logger::init();
    let config = match Config::new() {
        Ok(project_config) => {
            info!("Using config ~/.config/leetr/leetr.toml");
            project_config},
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    };


    let cli = argument_parser::Cli::parse();
    let title = cli.url.trim_end_matches('/').rsplit('/').nth(1).unwrap().to_string();

    let generator = project_generator::Generator::new(config, title);

    generator.generate_project().await?;
    Ok(())

}

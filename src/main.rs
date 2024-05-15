mod queries;
mod response_types;
mod html;
mod argument_parser;
mod project_generator;
mod config;
mod project_templates;
mod errors;

use clap::Parser;
use errors::GetResponse;


#[tokio::main]
async fn main() -> Result<(), GetResponse> {

    let config = config::Config::new().unwrap();
    let cli = argument_parser::Cli::parse();
    let title = cli.url.trim_end_matches('/').rsplit('/').nth(1).unwrap().to_string();

    let generator = project_generator::Generator::new(config, title);

    generator.generate_project().await?;
    Ok(())

}

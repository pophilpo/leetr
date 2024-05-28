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
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    let cli = argument_parser::Cli::parse();
    let title = cli
        .url
        .trim_end_matches('/')
        .rsplit('/')
        .nth(1)
        .unwrap()
        .to_string();

    let lang = match cli.lang {
        Some(lang) => lang,
        None => String::from("rust"),
    };

    let config = Config::new(lang)?;

    let project_generator = project_generator::Generator::new(config, title);
    project_generator.generate_project().await?;

    Ok(())
}

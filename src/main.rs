mod argument_parser;
mod config;
mod errors;
mod html;
mod logger;
mod project_generator;
mod project_templates;
mod queries;
mod response_types;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    let arguments = argument_parser::parse_args();

    let problem_url = arguments
        .get_one::<String>("problem_url")
        .expect("Is required");

    let title = problem_url
        .trim_end_matches('/')
        .rsplit('/')
        .nth(1)
        .unwrap()
        .to_string();

    let dir = arguments.get_one::<String>("directory").cloned();

    let lang = arguments
        .get_one::<String>("language")
        .expect("Has a default value")
        .clone();

    let config = Config::new(lang)?;

    let project_generator = project_generator::Generator::new(config, title, dir);
    project_generator.generate_project().await?;

    Ok(())
}

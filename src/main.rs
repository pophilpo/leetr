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
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::init();

    let arguments = argument_parser::parse_args();

    let problem_url = arguments
        .get_one::<String>("problem_url")
        .cloned()
        .expect("Is required");

    let title = argument_parser::get_title(problem_url);

    let dir = arguments.get_one::<String>("directory").cloned();

    let lang = arguments
        .get_one::<String>("language")
        .cloned()
        .expect("Has a default value");

    let config = Config::new(lang)?;

    match title {
        Ok(title) => {
            info!("Using {} as problem title", title);
            let project_generator = project_generator::Generator::new(config, title, dir);
            project_generator.generate_project().await?;

            Ok(())
        }
        Err(e) => {
            error!("{}", e);
            Ok(())
        }
    }
}

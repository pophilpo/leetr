mod argument_parser;
mod errors;
mod html;
mod logger;
mod project_generator;
mod project_templates;
mod queries;
mod response_types;

use log::error;

use std::env;

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

    let lang = match arguments.value_source("language") {
        Some(_) => arguments
            .get_one::<String>("language")
            .cloned()
            .expect("Is checked for presence"),
        None => match env::var("LEETR_DEFAULT_LANGUAGE") {
            Ok(value) => value,
            Err(_) => String::from("rust"),
        },
    };

    match title {
        Ok(title) => {
            let project_generator = project_generator::Generator::new(lang, title, dir);
            project_generator.generate_project().await?;

            Ok(())
        }
        Err(e) => {
            error!("{}", e);
            Ok(())
        }
    }
}

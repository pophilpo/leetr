use std::env;
use std::process;

use log::error;

mod argument_parser;
mod example_parser;
mod errors;
mod html;
mod logger;
mod project_generator;
mod project_templates;
mod queries;
mod response_types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

        None => env::var("LEETR_DEFAULT_LANGUAGE").unwrap_or_else(|_| String::from("rust")),
    };

    let title = match title {
        Ok(t) => t,
        Err(e) => {
            error!("{}", e);
            return Ok(());
        }
    };

    let generator = project_generator::Generator::new(lang, title, dir);

    match generator {
        Ok(generator) => {
            generator.generate_project()?;
            Ok(())
        }
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    }
}

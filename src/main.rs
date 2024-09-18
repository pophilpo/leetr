mod argument_parser;
mod errors;
mod leetcode_client;
mod logger;
mod project_generator;
mod response_types;

use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info, warn};

use project_generator::example_types::Example;
use project_generator::metadata::Metadata;
use project_generator::rust_generator::RustProjectGenerator;
use project_generator::traits::ProjectGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::setup_logger().unwrap();

    let client = leetcode_client::LeetCodeClient::new("triangle").unwrap();

    let code = client.get_editor_data()?;
    let code = code.get_code_snippet("rust")?;

    let console_data = client.get_console_panel_conifg()?;

    let example_string = console_data.get_example_testcases()?;
    let metadata = console_data.data.question.unwrap().metadata;

    info!("{:?}", metadata);
    let generator = RustProjectGenerator::new(code, example_string.clone(), metadata);

    let metadata = generator.parse_metadata()?;

    let example = Example::new(example_string[0].clone(), metadata)?;

    info!("{:?}", example);

    Ok(())
}

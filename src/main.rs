mod argument_parser;
mod errors;
mod leetcode_client;
mod logger;
mod project_generator;
mod response_types;

use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info, warn};

use std::process::exit;

use project_generator::example_types::Example;
use project_generator::metadata::Metadata;
use project_generator::rust_generator::RustProjectGenerator;
use project_generator::traits::ProjectGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::setup_logger().unwrap();

    let problem_title = String::from("binary-tree-inorder-traversal");
    let client = leetcode_client::LeetCodeClient::new(&problem_title).unwrap();

    let code = client.get_editor_data()?;
    let code = code.get_code_snippet("rust")?;

    info!("\n{}", code);

    let console_data = client.get_console_panel_conifg()?;
    let problem_description = client.get_problem_description()?;

    let html = problem_description.data.question;

    info!("Content html:\n{}", html.content);

    let outputs = html.extract_example_outputs();

    let example_string = console_data.get_example_testcases()?;
    let metadata = console_data.data.question.unwrap().metadata;

    let generator =
        RustProjectGenerator::new(code, example_string.clone(), metadata, problem_title);

    generator.generate_project(None, html.content);
    exit(1);

    let metadata = generator.parse_metadata()?;

    let example = Example::new(example_string, outputs, metadata)?;

    let output = generator.get_complete_code(example);

    info!("\n{}", output);

    Ok(())
}

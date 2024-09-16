mod argument_parser;
mod errors;
mod leetcode_client;
mod logger;
mod project_generator;
mod response_types;

use log::{debug, error, info, warn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logger::setup_logger().unwrap();

    let client = leetcode_client::LeetCodeClient::new("two-sum").unwrap();
    let response = client.get_editor_data().unwrap();
    info!("Got response for editor");
    let code = response.get_code_snippet("rust").unwrap();

    let response = client.get_console_panel_conifg().unwrap();
    let examples = response.get_example_testcases().unwrap();
    debug!("examples: {:?}", examples);
    Ok(())
}

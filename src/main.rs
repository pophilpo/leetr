mod argument_parser;
mod errors;
mod leetcode_client;
mod logger;
mod response_types;

use log::{debug, error, info, warn};

fn main() {
    logger::setup_logger().unwrap();

    info!("Setting up client");
    let client = leetcode_client::LeetCodeClient::new("two-sum").unwrap();

    info!("Sending request");
    let response = client.get_editor_data().unwrap();
    info!("Got response");
    debug!("{:?}", response);
}

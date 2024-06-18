const HEADER: &str = "application/json";

use std::error::Error;
use crate::rust::RustSingleCycle;
use reqwest::{self, blocking::Client, header::ACCEPT};

pub fn api_request_single_rust_cycle(
    rust_version: &str
) -> Result<RustSingleCycle, Box<dyn Error>> {
    let request_string: String = format!("https://endoflife.date/api/rust/{}.json", rust_version);
    // build client request
    let client = Client::new();
    // our body request
    let body = client.get(&request_string).header(ACCEPT, HEADER).send()?.bytes()?.to_vec();
    
    // Convert bytes to string safely
    let response = String::from_utf8(body)?;
    
    // Deserialize the response
    let deserialized_response = serde_json::from_str::<RustSingleCycle>(&response)?;
    
    Ok(deserialized_response)
}

const HEADER: &str = "application/json";

use std::error::Error;
use crate::rust::RustSingleCircle;
use reqwest::{self, blocking::Client, header::ACCEPT};


pub fn api_request_single_rust_circle(
    rust_version: &str
) -> Result<RustSingleCircle, Box<dyn Error>> {
    let request_string: String = format!("https://endoflife.date/api/rust/{}.json", rust_version);
    let client = Client::new();
    let body = client.get(request_string).header(ACCEPT, HEADER).send()?.bytes()?.to_vec();
    let response = unsafe { String::from_utf8_unchecked(body) };
    let deserialized_response = serde_json::from_str::<RustSingleCircle>(&response)?;
    Ok(deserialized_response)
}

pub fn api_request_all_rust_circles() -> Result<Vec<RustSingleCircle>, Box<dyn Error>> {
    let request_string: &str = "https://endoflife.date/api/rust.json";

    // Builds the client request
    let client = Client::new();

    // Our response body
    let body = client.get(request_string).header(ACCEPT, HEADER).send()?.bytes()?.to_vec();

    // Our response string
    let response = unsafe { String::from_utf8_unchecked(body) };
    let deserialized_response: Vec<RustSingleCircle> = serde_json::from_str(&response)?;
    Ok(deserialized_response)
}

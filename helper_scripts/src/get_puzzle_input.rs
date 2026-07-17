use reqwest::header::{HeaderMap, COOKIE};
use std::convert::TryInto;

// Fetches page HTML over HTTP
// Fails early on non-2xx responses
pub fn fetch_input_string(url: &str, data: &mut String) -> bool {
    let session_cookie = "session=53616c7465645f5f2cc297800af9826cb9b336137eef85fcb29acc1be035f7185fd3135a6fb4d17b06ff74baa497a3969ccd0cf62c996f7b04c78993d0e43f38"
        .try_into()
        .expect("Invalid session cookie");

    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(COOKIE, session_cookie);

    let client = reqwest::blocking::Client::new();

    // For real sites, you'll usually want to set a User-Agent
    let result = client.get(url)
        .headers(headers)
        .send();


    // 1. Unpack the network Result. 'response' is now a reqwest::blocking::Response
    let response = match result {
        Ok(res) => res,
        Err(error) => {
            *data = error.to_string();
            return false;
        }
    };

    // 2. Call error_for_status() directly on 'response'. 
    // This method returns a Result, so we match on its output.
    let valid_response = match response.error_for_status() {
        Ok(res) => res,
        Err(error) => {
            *data = error.to_string();
            return false;
        }
    };

    // 3. Read the text body safely from 'valid_response'
    match valid_response.text() {
        Ok(value) => {
            *data = value;
            return true;
        }
        Err(error) => {
            *data = error.to_string();
            return false;
        }
    }
}
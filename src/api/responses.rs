use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Parses the most basic response information from a HTTP request to PAN-OS
/// Accepts a trait to allow injection of test response types.
fn response_from_request<T: ReturnsText>(response: T) -> Result<Response, Box<dyn Error>> {
    let text = response.text();
    let response_struct: Response = from_str(&text)?;
    Ok(response_struct)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Response {
    #[serde(rename="@status")]
    status: String
}


// Test Struct - allows for mocking responses
struct TestResponse {
    text: String
}

// Implemented by TestResponse AND the actual `reqwest::blocking::Response` struct
trait ReturnsText {
    fn text(self) -> String;
}

impl ReturnsText for TestResponse {
    fn text(self) -> String {
        return self.text
    }
}

impl ReturnsText for reqwest::blocking::Response {
    fn text(self) -> String {
        self.text().unwrap()
    }
}

#[test]
fn test_response_from_request() {
    let test_response = TestResponse {
        text: "<response status = 'success'><result><key>abcd1234</key></result></response>".to_string()
    };

    match response_from_request(test_response) {
        Ok(response) => {
            assert_eq!(response.status, "success")
        }
        Err(_) => {assert!(false)}
    }
}
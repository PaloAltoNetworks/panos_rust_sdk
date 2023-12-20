use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::error::Error;


/// Returns the result of a keygen operation as a `KeyGenResponse` type.
pub fn keygen_response_from_request<T: ReturnsText>(response: T) -> Result<KeyGenResponse, Box<dyn Error>> {
    let text = response.text();
    let response_struct: KeyGenResponse = from_str(&text)?;
    Ok(response_struct)
}

/// Basic PAN-OS API Response
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Response {
    #[serde(rename="@status")]
    pub status: String
}

/// Keygen result
///     - key: The generated API Key
#[derive(Serialize, Deserialize)]
pub struct KeyGenResult {
    pub key: String,
}

/// Keygen response
#[derive(Serialize, Deserialize)]
pub struct KeyGenResponse {
    pub result: KeyGenResult,

    #[serde(flatten)]
    pub response: Response
}


// Test Struct - allows for mocking responses
struct TestResponse {
    text: String
}

// Implemented by TestResponse AND the actual `reqwest::blocking::Response` struct
pub trait ReturnsText {
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
fn test_keygen_response_from_request() {
    let test_response = TestResponse {
        text: "<response status = 'success'><result><key>abcd1234</key></result></response>".to_string()
    };

    match keygen_response_from_request(test_response) {
        Ok(response) => {
            assert_eq!(response.result.key, "abcd1234");
            assert_eq!(response.response.status, "success");
        }
        Err(e) => {
            panic!("{}", e.to_string());
        }
    }
}
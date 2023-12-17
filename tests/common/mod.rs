use std::env;
use panos_rust_sdk::api::ConnectionBuilder;

/// Sets up a common ConnectionBuilder object, used in integration tests,
/// based on environment variables.
#[allow(dead_code)]
pub fn setup_connection_builder() -> Result<ConnectionBuilder, env::VarError> {
    let password = match env::var("PASSWORD") {
        Ok(password) => password,
        Err(e) => return Err(e)
    };
    let username = match env::var("USERNAME") {
        Ok(username) => username,
        Err(e) => return Err(e)
    };
    let url = match env::var("URL") {
        Ok(url) => url,
        Err(e) => return Err(e)
    };

    Ok(ConnectionBuilder::new(
        username,
        password,
        url
    ).accept_invalid_certificates())

}
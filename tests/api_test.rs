use panos_rust_sdk;

mod common;

#[test]
fn it_connects() {
    panos_rust_sdk::api::connect();
}
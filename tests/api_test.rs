use crate::common::setup_connection_builder;

mod common;

#[test]
fn it_connects() {
    match setup_connection_builder() {
        Ok(connection_builder) => {
            // Try the connection function
            let connection = connection_builder.build();
            assert!(connection.is_ok())
        },
        // Test passes if no test fixture due to missing envvars.
        Err(_) => assert!(true),
    }
}
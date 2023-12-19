//! Implements the low-level HTTPS connectivity to a PAN-OS API, and manages things like;
//!     - Authentication
//!     - Interfaces for standard API functions (op/keygen/config)

mod responses;

use reqwest::{Client, Proxy};

// API Query Param types
const PARAM_TYPE_KEYGEN: (&str, &str) = ("type", "keygen");

/// Connection manages the actual HTTPS connection to the PAN-OS API
/// It's recommended to use `ConnectionBuilder` to setup its parameters
/// You can use this struct directly if you wish to pass complex settings to `reqwest::Client`.
pub struct Connection {
    url: String,
    api_key: String,
    client: Client,
}

impl Connection {
    pub fn builder() -> ConnectionBuilder {
        ConnectionBuilder::default()
    }
}

/// Paramaters for intializing connections to PAN-OS devices
/// ConnectionBuilder generates Connection objects through the use of the `new(...)` method.
#[derive(Default)]
pub struct ConnectionBuilder {
    /// Username
    /// Password
    /// URL, including schema; `https://127.0.0.1`
    /// Proxy (Optional); Configurable `reqwest::Proxy` object, allowing for the use of a http proxy when connecting
    ///     to PAN-OS.
    /// accept_invalid_certificates (default: true): Accept bad or insecure certificates. Think carefully before
    ///     enabling this one!
    username: String,
    password: String,
    url: String,

    proxy: Option<Proxy>,
    accept_invalid_certificates: bool
}

/// Builds Connection objects.
impl ConnectionBuilder {
    /// Creates a new `ConnectionBuilder` Instance.
    /// Requires the following arguments:
    ///
    ///  - Username<String>: The PAN-OS Username
    ///  - Password<String>: The PAN-OS Password
    ///  - url<String>: The HTTP URL for the PAN-OS Device to connect to.
    pub fn new(username: String, password: String, url: String) -> ConnectionBuilder {
        ConnectionBuilder {
            username,
            password,
            url,
            proxy: None,
            accept_invalid_certificates: false
        }
    }

    /// Set the HTTP Proxy for the client, if there is one
    pub fn proxy(mut self, proxy: Proxy) -> ConnectionBuilder {
        self.proxy = Some(proxy);
        self
    }

    /// Enables accepting invalid certificates - use with caution!
    pub fn accept_invalid_certificates(mut self) -> ConnectionBuilder {
        self.accept_invalid_certificates = true;
        self
    }

    /// Create a new `Connection` object using the parameters from the builder.
    /// This function will generate an API key in PAN-OS to simplify later calls, and to init the `Connection` object.
    pub fn build(self) {
        let mut client_builder = reqwest::blocking::Client::builder();

        if let Some(proxy) = self.proxy {
            client_builder = client_builder.proxy(proxy);
        }

        if self.accept_invalid_certificates {
            client_builder = client_builder.danger_accept_invalid_certs(true);
        }

        // Set rustls as the default TLS handler to ensure Client cert store is used
        client_builder = client_builder.use_rustls_tls();

        let client = client_builder.build().unwrap();

        let params = [
            ("user", self.username.as_str()),
            ("password", self.password.as_str()),
            PARAM_TYPE_KEYGEN
        ];

        let url = reqwest::Url::parse_with_params(
            format!("{}/api", self.url).as_str(),
            &params,
        ).unwrap();

        let response = client.get(url).send().unwrap();
        println!("{:#?}", response.text());
    }
}

#[test]
fn test_connection_builder_set_proxy() {
    let mut connection_builder = ConnectionBuilder::new(
        String::from("admin"),
        String::from("password"),
        String::from("https://127.0.0.1"),
    );
    connection_builder = connection_builder.proxy(
        Proxy::https("https://127.0.0.1").unwrap()
    );
    assert!(connection_builder.proxy.is_some());
}
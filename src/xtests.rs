use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_skip::is_default;
use std::collections::BTreeMap as Map;

/// The test in the "x-tests" extension.
/// Implementation is based on 8c84cc6 of <https://github.com/davidkpiano/openapi-test>
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Test {
    /// The description of the test, used for reporting the test results.
    pub description: String,
    #[serde(skip_serializing_if = "is_default", default)]
    /// Whether to add the --token JWT to the header and authorize the request (default: false).
    pub auth: bool,
    #[serde(skip_serializing_if = "is_default", default)]
    /// If true, this test will be skipped. (default: false).
    pub skip: bool,
    #[serde(skip_serializing_if = "is_default", default)]
    /// (optional) Supplies dynamic parameter values (e.g., "name": ... would supply the value for /{name} in the path). Only needed for tests with URL params.
    pub params: Map<String, String>,
    #[serde(skip_serializing_if = "is_default", default)]
    /// An object that contains:
    /// query
    pub request: Request,
    #[serde(skip_serializing_if = "is_default", default)]
    /// An object that contains:
    /// status
    pub response: Response,

    #[serde(skip_serializing_if = "is_default", default)]
    pub required: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Request {
    #[serde(skip_serializing_if = "is_default", default)]
    pub body: Value,
    #[serde(skip_serializing_if = "is_default", default)]
    pub cookie: String,
    #[serde(skip_serializing_if = "is_default", default)]
    pub headers: Map<String, String>,
    #[serde(skip_serializing_if = "is_default", default)]
    /// (optional) - A mapping of query parameter keys to their values (e.g., "format": "json" will append ?format=json to the URL)
    pub query: Map<String, String>,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Response {
    #[serde(skip_serializing_if = "is_default", default)]
    pub cookie: Map<String, String>,
    #[serde(skip_serializing_if = "is_default", default)]
    pub headers: Map<String, String>,
    #[serde(skip_serializing_if = "is_default", default)]
    /// A status code (e.g., 200) or array of status codes (e.g., [200, 301]) that the expected response should match
    pub status: Vec<u16>,
}

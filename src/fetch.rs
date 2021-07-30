use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;

use crate::apipath::ApiPath;
use crate::generator::{Map, Operation, OperationInfo, Parameter, RefOr, Responses};
use crate::oasgen::Oas3Builder;
use crate::xtests::Test;

impl Oas3Builder {
    pub fn fetch<O: JsonSchema + Serialize, E: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
    ) {
        self.fetch_with_tests::<O, E>(web_path, document_name, operation_description, &[]);
    }

    /// This variant accepts a test spec
    /// ```
    /// use oas_gen::xtests::{Test,Response};
    /// let mut params = std::collections::BTreeMap::default();
    /// params.insert("key".to_owned(), "8472".to_owned());
    /// let test_ok = Test {
    ///     description: "Fetch a document by key.".to_owned(),
    ///     response: Response {
    ///         status: vec![200],
    ///         ..Response::default()
    ///     },
    ///     params,
    ///     ..Test::default()
    /// };
    /// assert_eq!(r#"{"description":"Fetch a document by key.","params":{"key":"8472"},"response":{"status":[200]}}"#, serde_json::to_string(&test_ok).unwrap());
    /// let tests = vec![test_ok];
    /// ```
    ///
    /// # Panics
    ///
    /// Will panic if json serialization of `tests` fail
    pub fn fetch_with_tests<O: JsonSchema + Serialize, E: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
        tests: &[Test],
    ) {
        let operation_id = format!("fetch{}", document_name);
        let method = http::Method::GET;

        let mut resps = Responses::default();

        let status = "201".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses::<E>(&mut resps);

        let mut parameters: Vec<RefOr<Parameter>> = vec![];
        self.add_path_params(web_path.clone(), &mut parameters);

        let mut extensions: Map<String, Value> = Map::default();
        if !tests.is_empty() {
            extensions.insert("x-tests".to_owned(), serde_json::to_value(tests).unwrap());
        }

        self.generator.add_operation(OperationInfo {
            path: web_path.to_string(),
            method,
            operation: Operation {
                operation_id: Some(operation_id),
                description: operation_description,
                responses: resps,
                request_body: None,
                parameters,
                extensions,
                ..Operation::default()
            },
        });
    }
}

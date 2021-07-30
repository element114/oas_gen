use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;

use crate::apipath::ApiPath;
use crate::generator::{Map, Operation, OperationInfo, Parameter, RefOr, Responses};
use crate::oasgen::Oas3Builder;
use crate::xtests::Test;

impl Oas3Builder {
    pub fn any<I: JsonSchema + Serialize, O: JsonSchema + Serialize, E: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        method: http::Method,
        document_name: String,
        operation_name: &str,
        operation_description: Option<String>,
    ) {
        self.any_with_tests::<I, O, E>(
            web_path,
            method,
            document_name,
            operation_name,
            operation_description,
            &[],
        );
    }

    /// # Panics
    ///
    /// Will panic if json serialization of `tests` fail
    pub fn any_with_tests<
        I: JsonSchema + Serialize,
        O: JsonSchema + Serialize,
        E: JsonSchema + Serialize,
    >(
        &mut self,
        web_path: &ApiPath,
        method: http::Method,
        document_name: String,
        operation_name: &str,
        operation_description: Option<String>,
        tests: &[Test],
    ) {
        let operation_id = format!("{}{}", operation_name, document_name);

        let mut resps = Responses::default();

        let status = "200".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses::<E>(&mut resps);

        let request_body = self.create_request_body::<I>();

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
                request_body,
                parameters,
                extensions,
                ..Operation::default()
            },
        });
    }
}

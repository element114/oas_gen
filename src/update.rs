use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;

use crate::apipath::ApiPath;
use crate::oasgen::Oas3Builder;
use crate::okapi3::{Map, Operation, OperationInfo, Parameter, RefOr, Responses};
use crate::xtests::Test;

impl Oas3Builder {
    pub fn update<
        I: JsonSchema + Serialize,
        O: JsonSchema + Serialize,
        E: JsonSchema + Serialize,
    >(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
    ) {
        self.update_with_tests::<I, O, E>(web_path, document_name, operation_description, &[])
    }

    pub fn update_with_tests<
        I: JsonSchema + Serialize,
        O: JsonSchema + Serialize,
        E: JsonSchema + Serialize,
    >(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
        tests: &[Test],
    ) {
        let operation_id = format!("update{}", document_name);
        let method = http::Method::PATCH;

        let mut resps = Responses::default();

        let status = "201".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses::<E>(&mut resps);

        let request_body = self.create_request_body::<I>();

        let mut parameters: Vec<RefOr<Parameter>> = vec![];
        self.add_path_params(web_path.clone(), &mut parameters);

        let mut extensions: Map<String, Value> = Map::default();
        extensions.insert("x-tests".to_owned(), serde_json::to_value(tests).unwrap());

        self.generator.add_operation(OperationInfo {
            path: web_path.to_string(),
            method,
            operation: Operation {
                operation_id: Some(operation_id),
                description: operation_description,
                responses: resps,
                request_body: Some(request_body.into()),
                parameters,
                extensions,
                ..Operation::default()
            },
        })
    }
}
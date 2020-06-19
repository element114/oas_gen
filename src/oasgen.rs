use contracts::pre;
use heck::CamelCase;
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;

use crate::apipath::ApiPath;
use crate::okapi3::{
    Info, MediaType, OpenApi, OpenApiGenerator, Parameter, ParameterValue, RefOr, RequestBody,
    Response, Responses,
};

#[derive(Debug, Clone)]
pub struct Oas3Builder {
    pub(crate) generator: OpenApiGenerator,
}
impl Default for Oas3Builder {
    fn default() -> Self {
        Oas3Builder::new()
    }
}
impl Oas3Builder {
    #[must_use]
    pub fn new() -> Self {
        let mut sts = SchemaSettings::openapi3();
        sts.option_add_null_type = false;
        sts.option_nullable = false;
        Oas3Builder {
            generator: OpenApiGenerator::new(SchemaGenerator::new(sts)),
        }
    }

    #[must_use]
    pub fn build(self, version: String) -> OpenApi {
        let mut openapi = self.generator.into_openapi();
        openapi.info = Info {
            version,
            ..openapi.info
        };
        // openapi.security = Vec<SecurityRequirement>
        openapi
    }

    pub(crate) fn create_response<O: JsonSchema + Serialize>(
        &mut self,
        description: String,
    ) -> Response {
        let content_type = "application/json; charset=utf-8".to_owned();
        let schema = self.generator.schema_generator.subschema_for::<O>().into();
        let media = MediaType {
            schema: Some(schema),
            ..MediaType::default()
        };
        let mut resp = Response::default();
        resp.description = description;
        resp.content.insert(content_type, media);
        resp
    }

    pub(crate) fn create_request_body<I: JsonSchema + Serialize>(&mut self) -> RequestBody {
        let content_type = "application/json; charset=utf-8".to_owned();
        let schema = self.generator.schema_generator.subschema_for::<I>().into();
        let media = MediaType {
            schema: Some(schema),
            ..MediaType::default()
        };
        let mut request_body = RequestBody::default();
        request_body.content.insert(content_type, media);
        request_body.required = true;
        request_body
    }

    pub(crate) fn add_error_responses<E: Serialize + JsonSchema>(
        &mut self,
        responses: &mut Responses,
    ) {
        let status = "400".to_owned();
        let resp = self.create_response::<E>("Bad Request".to_owned());
        responses.responses.insert(status, resp.into());

        let status = "401".to_owned();
        let resp = self.create_response::<E>("Unauthorized".to_owned());
        responses.responses.insert(status, resp.into());

        #[cfg(feature = "teapot")]
        {
            let status = "418".to_owned();
            let resp = self.create_response::<E>("I'm a teapot".to_owned());
            responses.responses.insert(status, resp.into());
        }

        let status = "500".to_owned();
        let resp = self.create_response::<E>("Internal Server Error".to_owned());
        responses.responses.insert(status, resp.into());
    }

    #[pre(param_name.starts_with('{'))]
    #[pre(param_name.ends_with('}'))]
    #[pre(param_name.len() >2)]
    pub(crate) fn add_path_param(
        &mut self,
        param_name: String,
        parameters: &mut Vec<RefOr<Parameter>>,
        description: String,
    ) {
        let param_name = param_name
            .trim_start_matches('{')
            .trim_end_matches('}')
            .to_owned();
        let param_schema = ParameterValue::Schema {
            style: None,
            explode: None,
            allow_reserved: false,
            schema: Box::new(
                self.generator
                    .schema_generator
                    .subschema_for::<String>()
                    .into(),
            ),
            example: Some(Value::String {
                0: "84742".to_owned(),
            }),
            examples: None,
        };
        let param = Parameter {
            name: param_name,
            location: "path".to_owned(),
            description: Some(description),
            required: true,
            deprecated: false,
            allow_empty_value: false,
            value: param_schema,
            extensions: std::collections::BTreeMap::new(),
        };
        parameters.push(param.into());
    }

    pub(crate) fn add_path_params(
        &mut self,
        api_path: ApiPath,
        parameters: &mut Vec<RefOr<Parameter>>,
    ) {
        if api_path.token.is_none() {
            if let Some((last, elements)) = api_path.ids.split_last() {
                for api_id in elements {
                    let description =
                        format!("{}({})", api_id.document.to_camel_case(), api_id.key);
                    self.add_path_param(api_id.key.clone(), parameters, description);
                }
                let description = format!(
                    "The {} document is identified by the {} key at the end of this url.",
                    last.document.to_camel_case(),
                    last.key
                );
                self.add_path_param(last.key.clone(), parameters, description);
            }
        } else {
            for api_id in api_path.ids {
                let description = format!("{}({})", api_id.document.to_camel_case(), api_id.key);
                self.add_path_param(api_id.key, parameters, description);
            }
        }
        for query_param in api_path.query_params {
            parameters.push(query_param.into());
        }
    }
}

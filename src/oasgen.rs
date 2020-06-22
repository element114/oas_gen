use contracts::pre;
use heck::CamelCase;
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::{JsonSchema, Map};
use serde::Serialize;
use serde_json::Value;

use crate::apipath::ApiPath;
use crate::okapi3::{
    Components, Info, MediaType, OpenApi, OpenApiGenerator, Parameter, ParameterValue, RefOr,
    RequestBody, Response, Responses, SecurityScheme, SecuritySchemeData,
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

    /// Create a http bearer Auth security scheme
    #[must_use]
    pub fn create_bearer_scheme() -> SecurityScheme {
        SecurityScheme {
            // schema_type: "http".to_owned(),
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("JWT".to_owned()),
            },
            description: None,
            extensions: Map::default(),
            schema_type: None,
        }
    }

    ///
    /// ```
    /// let mut security_schemes: Map<String, RefOr<SecurityScheme>> = Map::default();
    /// let security_scheme = create_bearer_scheme();
    /// security_schemes.insert("bearerAuth".to_owned(), RefOr::Object(security_scheme));
    /// ```
    #[must_use]
    pub fn build_with_security(
        self,
        version: String,
        security_schemes: Map<String, RefOr<SecurityScheme>>,
    ) -> OpenApi {
        let mut security: Map<String, Vec<String>> = Map::default();
        security.insert("bearerAuth".to_owned(), vec![]);

        let mut openapi = self.build(version);
        let components = Components {
            security_schemes,
            ..openapi.components.unwrap_or_default()
        };
        openapi.components = Some(components);
        openapi.security.push(security);
        openapi
    }

    pub(crate) fn create_response<O: JsonSchema + Serialize>(
        &mut self,
        description: String,
    ) -> Response {
        let content_type = "application/json; charset=utf-8".to_owned();
        let schema: schemars::schema::SchemaObject =
            self.generator.schema_generator.subschema_for::<O>().into();
        // OAS3 requires that if InstanceType::Null then ommit content entirely
        let ommit_content =
            if let Some(schemars::schema::SingleOrVec::Single(some)) = &schema.instance_type {
                schemars::schema::InstanceType::Null.eq(&*some)
            } else {
                false
            };
        let media = MediaType {
            schema: Some(schema),
            ..MediaType::default()
        };
        let mut resp = Response::default();
        resp.description = description;
        if !ommit_content {
            resp.content.insert(content_type, media);
        }
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

#[cfg(test)]
mod tests {
    use super::Oas3Builder;
    use serde_json::json;

    #[test]
    fn test_create_response_empty() {
        let mut oasb = Oas3Builder::default();
        let resp = oasb.create_response::<()>("Empty response is invalid in oas 3.0.".to_owned());

        let got = serde_json::to_value(resp).unwrap();
        let expect = json!({
            "description": "Empty response is invalid in oas 3.0."
        });
        assert_eq!(expect, got);
    }
}

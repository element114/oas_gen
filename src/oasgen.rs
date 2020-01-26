use contracts::pre;
use heck::CamelCase;
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;

use crate::okapi3::*;

#[derive(Debug, Clone, Default)]
pub struct ApiId {
    pub document: String,
    pub key: String,
    // Use new or default please
    nothing: (),
}
impl std::fmt::Display for ApiId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.document, self.key)
    }
}
impl ApiId {
    #[pre(!document.contains('/'))]
    #[pre(key.starts_with('{'))]
    #[pre(key.ends_with('}'))]
    pub fn new(document: &str, key: &str) -> Self {
        ApiId {
            document: document.to_owned(),
            key: key.to_owned(),
            nothing: (),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiPath {
    pub prefix: Option<String>,
    pub ids: Vec<ApiId>,
    pub token: Option<String>,
    query_params: Vec<Parameter>,
}
impl std::fmt::Display for ApiPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tmp = vec![];
        let ids: Vec<String> = self.ids.iter().map(|id| id.to_string()).collect();
        if let Some(pfx) = &self.prefix {
            tmp.push(pfx.clone());
        }
        tmp.extend(ids);
        if let Some(tkn) = &self.token {
            tmp.push(tkn.clone());
        }
        let pth: String = tmp.join("/");

        write!(f, "/{}", pth)
    }
}
impl ApiPath {
    #[pre(!prefix.clone().unwrap_or_default().starts_with('/'))]
    #[pre(!prefix.clone().unwrap_or_default().contains('{'))]
    #[pre(!prefix.clone().unwrap_or_default().contains('}'))]
    #[pre(!token.clone().unwrap_or_default().starts_with('/'))]
    #[pre(!token.clone().unwrap_or_default().contains('{'))]
    #[pre(!token.clone().unwrap_or_default().contains('}'))]
    pub fn new(prefix: Option<String>, ids: Vec<ApiId>, token: Option<String>) -> Self {
        ApiPath {
            prefix,
            ids,
            token,
            query_params: vec![],
        }
    }

    #[pre(!prefix.clone().unwrap_or_default().starts_with('/'))]
    #[pre(!prefix.clone().unwrap_or_default().contains('{'))]
    #[pre(!prefix.clone().unwrap_or_default().contains('}'))]
    #[pre(!token.clone().unwrap_or_default().starts_with('/'))]
    #[pre(!token.clone().unwrap_or_default().contains('{'))]
    #[pre(!token.clone().unwrap_or_default().contains('}'))]
    pub fn with_queries(
        prefix: Option<String>,
        ids: Vec<ApiId>,
        token: Option<String>,
        qpbuilders: Vec<QueryParamBuilder>,
    ) -> Self {
        let query_params = qpbuilders.iter().map(|qpb| qpb.build()).collect();
        ApiPath {
            prefix,
            ids,
            token,
            query_params,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Oas3Builder {
    generator: OpenApiGenerator,
}
impl Default for Oas3Builder {
    fn default() -> Self {
        Oas3Builder::new()
    }
}
impl Oas3Builder {
    pub fn new() -> Self {
        Oas3Builder {
            generator: OpenApiGenerator::new(SchemaGenerator::new(SchemaSettings::openapi3())),
        }
    }

    pub fn build(self, version: String) -> OpenApi {
        let mut openapi = self.generator.into_openapi();
        openapi.info = Info {
            version,
            ..openapi.info
        };
        openapi
    }

    pub fn list<O: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
    ) {
        let operation_id = format!("list{}", document_name);
        let method = http::Method::GET;

        let mut resps = Responses::default();

        let status = "201".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses(&mut resps);

        let mut parameters: Vec<RefOr<Parameter>> = vec![];
        self.add_path_params(web_path.clone(), &mut parameters);

        self.generator.add_operation(OperationInfo {
            path: web_path.to_string(),
            method,
            operation: Operation {
                operation_id: Some(operation_id),
                description: operation_description,
                responses: resps,
                request_body: None,
                parameters,
                ..Default::default()
            },
        })
    }

    pub fn fetch<O: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
    ) {
        let operation_id = format!("fetch{}", document_name);
        let method = http::Method::GET;

        let mut resps = Responses::default();

        let status = "201".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses(&mut resps);

        let mut parameters: Vec<RefOr<Parameter>> = vec![];
        self.add_path_params(web_path.clone(), &mut parameters);

        self.generator.add_operation(OperationInfo {
            path: web_path.to_string(),
            method,
            operation: Operation {
                operation_id: Some(operation_id),
                description: operation_description,
                responses: resps,
                request_body: None,
                parameters,
                ..Default::default()
            },
        })
    }

    pub fn delete<I: JsonSchema + Serialize, O: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
    ) {
        let operation_id = format!("delete{}", document_name);
        let method = http::Method::DELETE;

        let mut resps = Responses::default();

        let status = "202".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses(&mut resps);

        let request_body = self.create_request_body::<I>();

        let mut parameters: Vec<RefOr<Parameter>> = vec![];
        self.add_path_params(web_path.clone(), &mut parameters);

        self.generator.add_operation(OperationInfo {
            path: web_path.to_string(),
            method,
            operation: Operation {
                operation_id: Some(operation_id),
                description: operation_description,
                responses: resps,
                request_body: Some(request_body.into()),
                parameters,
                ..Default::default()
            },
        })
    }

    pub fn create<I: JsonSchema + Serialize, O: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
    ) {
        let operation_id = format!("create{}", document_name);
        let method = http::Method::POST;

        let mut resps = Responses::default();

        let status = "201".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses(&mut resps);

        let request_body = self.create_request_body::<I>();

        let mut parameters: Vec<RefOr<Parameter>> = vec![];
        self.add_path_params(web_path.clone(), &mut parameters);

        self.generator.add_operation(OperationInfo {
            path: web_path.to_string(),
            method,
            operation: Operation {
                operation_id: Some(operation_id),
                description: operation_description,
                responses: resps,
                request_body: Some(request_body.into()),
                parameters,
                ..Default::default()
            },
        })
    }

    pub fn update<I: JsonSchema + Serialize, O: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
    ) {
        let operation_id = format!("update{}", document_name);
        let method = http::Method::PATCH;

        let mut resps = Responses::default();

        let status = "201".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses(&mut resps);

        let request_body = self.create_request_body::<I>();

        let mut parameters: Vec<RefOr<Parameter>> = vec![];
        self.add_path_params(web_path.clone(), &mut parameters);

        self.generator.add_operation(OperationInfo {
            path: web_path.to_string(),
            method,
            operation: Operation {
                operation_id: Some(operation_id),
                description: operation_description,
                responses: resps,
                request_body: Some(request_body.into()),
                parameters,
                ..Default::default()
            },
        })
    }

    pub fn replace<I: JsonSchema + Serialize, O: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        document_name: String,
        operation_description: Option<String>,
    ) {
        let operation_id = format!("replace{}", document_name);
        let method = http::Method::PUT;

        let mut resps = Responses::default();

        let status = "201".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses(&mut resps);

        let request_body = self.create_request_body::<I>();

        let mut parameters: Vec<RefOr<Parameter>> = vec![];
        self.add_path_params(web_path.clone(), &mut parameters);

        self.generator.add_operation(OperationInfo {
            path: web_path.to_string(),
            method,
            operation: Operation {
                operation_id: Some(operation_id),
                description: operation_description,
                responses: resps,
                request_body: Some(request_body.into()),
                parameters,
                ..Default::default()
            },
        })
    }

    pub fn any<I: JsonSchema + Serialize, O: JsonSchema + Serialize>(
        &mut self,
        web_path: &ApiPath,
        method: http::Method,
        document_name: String,
        operation_name: String,
        operation_description: Option<String>,
    ) {
        let operation_id = format!("{}{}", operation_name, document_name);

        let mut resps = Responses::default();

        let status = "200".to_owned();
        let resp = self.create_response::<O>(document_name);
        resps.responses.insert(status, resp.into());

        self.add_error_responses(&mut resps);

        let request_body = self.create_request_body::<I>();

        let mut parameters: Vec<RefOr<Parameter>> = vec![];
        self.add_path_params(web_path.clone(), &mut parameters);

        self.generator.add_operation(OperationInfo {
            path: web_path.to_string(),
            method,
            operation: Operation {
                operation_id: Some(operation_id),
                description: operation_description,
                responses: resps,
                request_body: Some(request_body.into()),
                parameters,
                ..Default::default()
            },
        })
    }

    fn create_response<O: JsonSchema + Serialize>(&mut self, description: String) -> Response {
        let content_type = "application/json; charset=utf-8".to_owned();
        let schema = self.generator.schema_generator.subschema_for::<O>().into();
        let media = MediaType {
            schema: Some(schema),
            ..Default::default()
        };
        let mut resp = Response::default();
        resp.description = description;
        resp.content.insert(content_type, media);
        resp
    }

    fn create_request_body<I: JsonSchema + Serialize>(&mut self) -> RequestBody {
        let content_type = "application/json; charset=utf-8".to_owned();
        let schema = self.generator.schema_generator.subschema_for::<I>().into();
        let media = MediaType {
            schema: Some(schema),
            ..Default::default()
        };
        let mut request_body = RequestBody::default();
        request_body.content.insert(content_type, media);
        request_body.required = true;
        request_body
    }

    fn add_error_responses(&mut self, responses: &mut Responses) {
        let status = "400".to_owned();
        let resp = self.create_response::<Result<String, String>>("Bad Request".to_owned());
        responses.responses.insert(status, resp.into());

        let status = "401".to_owned();
        let resp = self.create_response::<Result<String, String>>("Unauthorized".to_owned());
        responses.responses.insert(status, resp.into());

        #[cfg(feature = "teapot")]
        {
            let status = "418".to_owned();
            let resp = self.create_response::<Result<String, String>>("I'm a teapot".to_owned());
            responses.responses.insert(status, resp.into());
        }

        let status = "500".to_owned();
        let resp =
            self.create_response::<Result<String, String>>("Internal Server Error".to_owned());
        responses.responses.insert(status, resp.into());
    }

    #[pre(param_name.starts_with('{'))]
    #[pre(param_name.ends_with('}'))]
    #[pre(param_name.len() >2)]
    fn add_path_param(
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

    fn add_path_params(&mut self, api_path: ApiPath, parameters: &mut Vec<RefOr<Parameter>>) {
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

#[derive(Debug, Clone)]
pub struct QueryParamBuilder {
    param: Parameter,
    schema_generator: SchemaGenerator,
}
impl QueryParamBuilder {
    pub fn new<T: JsonSchema + Serialize>(name: String, example: Option<T>) -> Self {
        let mut schema_generator = SchemaGenerator::new(SchemaSettings::openapi3());
        let example = if let Some(ex) = example {
            Some(serde_json::to_value(&ex).unwrap_or_default())
        } else {
            None
        };
        let param_name = name;
        let param_schema = ParameterValue::Schema {
            style: None,
            explode: None,
            allow_reserved: false,
            schema: Box::new(schema_generator.subschema_for::<T>().into()),
            example,
            examples: None,
        };
        let param = Parameter {
            name: param_name,
            location: "query".to_owned(),
            description: None,
            required: false,
            deprecated: false,
            allow_empty_value: false,
            value: param_schema,
            extensions: std::collections::BTreeMap::new(),
        };

        QueryParamBuilder {
            param,
            schema_generator,
        }
    }

    pub fn build(&self) -> Parameter {
        self.param.clone()
    }

    pub fn description(&self, description: String) -> Self {
        let mut me = self.clone();
        me.param.description = Some(description);
        me
    }

    pub fn required(&self, required: bool) -> Self {
        let mut me = self.clone();
        me.param.required = required;
        me
    }

    pub fn deprecated(&self, deprecated: bool) -> Self {
        let mut me = self.clone();
        me.param.deprecated = deprecated;
        me
    }

    pub fn allow_empty_value(&self, allow_empty_value: bool) -> Self {
        let mut me = self.clone();
        me.param.allow_empty_value = allow_empty_value;
        me
    }

    pub fn style(&self, style: ParameterStyle) -> Self {
        let mut me = self.clone();
        if let ParameterValue::Schema {
            style: _s,
            explode: e,
            allow_reserved: a,
            schema: sc,
            example: ex,
            examples: exs,
        } = me.param.value
        {
            let ps = ParameterValue::Schema {
                style: Some(style),
                explode: e,
                allow_reserved: a,
                schema: sc,
                example: ex,
                examples: exs,
            };
            me.param.value = ps;
        }
        me
    }

    pub fn explode(&self, explode: bool) -> Self {
        let mut me = self.clone();
        if let ParameterValue::Schema {
            style: s,
            explode: _e,
            allow_reserved: a,
            schema: sc,
            example: ex,
            examples: exs,
        } = me.param.value
        {
            let ps = ParameterValue::Schema {
                style: s,
                explode: Some(explode),
                allow_reserved: a,
                schema: sc,
                example: ex,
                examples: exs,
            };
            me.param.value = ps;
        }
        me
    }

    pub fn allow_reserved(&self, allow_reserved: bool) -> Self {
        let mut me = self.clone();
        if let ParameterValue::Schema {
            style: s,
            explode: e,
            allow_reserved: _a,
            schema: sc,
            example: ex,
            examples: exs,
        } = me.param.value
        {
            let ps = ParameterValue::Schema {
                style: s,
                explode: e,
                allow_reserved,
                schema: sc,
                example: ex,
                examples: exs,
            };
            me.param.value = ps;
        }
        me
    }

    pub fn example(&self, example: Value) -> Self {
        let mut me = self.clone();
        if let ParameterValue::Schema {
            style: s,
            explode: e,
            allow_reserved: a,
            schema: sc,
            example: _ex,
            examples: exs,
        } = me.param.value
        {
            let ps = ParameterValue::Schema {
                style: s,
                explode: e,
                allow_reserved: a,
                schema: sc,
                example: Some(example),
                examples: exs,
            };
            me.param.value = ps;
        }
        me
    }

    pub fn examples(&self, examples: Map<String, Example>) -> Self {
        let mut me = self.clone();
        let examples = if examples.is_empty() {
            None
        } else {
            Some(examples)
        };
        if let ParameterValue::Schema {
            style: s,
            explode: e,
            allow_reserved: a,
            schema: sc,
            example: ex,
            examples: _exs,
        } = me.param.value
        {
            let ps = ParameterValue::Schema {
                style: s,
                explode: e,
                allow_reserved: a,
                schema: sc,
                example: ex,
                examples,
            };
            me.param.value = ps;
        }
        me
    }
}

#[cfg(test)]
mod tests {
    use super::ApiId;
    use super::ApiPath;

    #[test]
    fn test_api_path() {
        let test_path = ApiPath::new(Some("api".to_owned()), vec![], Some("testdoc".to_owned()));
        let test_str = test_path.to_string();
        assert_eq!("/api/testdoc", test_str.as_str());

        let test_path = ApiPath::new(Some("api/testdoc".to_owned()), vec![], None);
        let test_str = test_path.to_string();
        assert_eq!("/api/testdoc", test_str.as_str());

        let test_path = ApiPath::new(
            Some("api".to_owned()),
            vec![ApiId::new("parents", "{pid}")],
            Some("testdoc".to_owned()),
        );
        let test_str = test_path.to_string();
        assert_eq!("/api/parents/{pid}/testdoc", test_str.as_str());
    }
}

use http::Method;
use schemars::gen::{SchemaGenerator, SchemaSettings};
// use schemars::schema::SchemaObject;
pub use okapi::openapi3::{Components, OpenApi, Operation, PathItem, *};

use std::collections::{hash_map::Entry as HashEntry, HashMap};

pub type Map<K, V> = schemars::Map<K, V>;
// pub type SecurityRequirement = Map<String, Vec<String>>;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct OpenApiGenerator {
    pub schema_generator: SchemaGenerator,
    pub operations: HashMap<(String, http::Method), Operation>,
    // passthrough
    components: Components,
    openapi: OpenApi,
}
impl OpenApiGenerator {
    pub fn new(generator: SchemaGenerator) -> Self {
        OpenApiGenerator {
            schema_generator: generator,
            operations: HashMap::default(),
            components: Components::default(),
            openapi: OpenApi::default(),
        }
    }
    pub fn add_operation(&mut self, mut op: OperationInfo) {
        if let Some(op_id) = op.operation.operation_id {
            // TODO do this outside add_operation
            op.operation.operation_id = Some(op_id.trim_start_matches(':').replace("::", "_"));
        }
        match self.operations.entry((op.path, op.method)) {
            HashEntry::Occupied(e) => {
                let (path, method) = e.key();
                panic!(
                    "An OpenAPI operation has already been added for {} {}",
                    method, path
                );
            }
            HashEntry::Vacant(e) => e.insert(op.operation),
        };
    }

    pub fn into_openapi(self) -> OpenApi {
        OpenApi {
            openapi: "3.0.0".to_owned(),
            paths: {
                let mut paths = Map::new();
                for ((path, method), op) in self.operations {
                    let path_item: &mut PathItem = paths.entry(path).or_default();
                    Self::add_operation_to_path_item(path_item, &method, op);
                }
                paths
            },
            components: Some(Components {
                schemas: self
                    .schema_generator
                    .definitions()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone().into()))
                    .collect::<Map<_, _>>(),
                ..self.components
            }),
            ..self.openapi
        }
    }

    fn add_operation_to_path_item(path_item: &mut PathItem, method: &http::Method, op: Operation) {
        // use http::Method::*;
        let option = match *method {
            Method::GET => &mut path_item.get,
            Method::PUT => &mut path_item.put,
            Method::POST => &mut path_item.post,
            Method::DELETE => &mut path_item.delete,
            Method::OPTIONS => &mut path_item.options,
            Method::HEAD => &mut path_item.head,
            Method::PATCH => &mut path_item.patch,
            Method::TRACE => &mut path_item.trace,
            // Connect not available in OpenAPI3. Maybe should set in extensions?
            // &Method::CONNECT => return,
            _ => return,
        };
        assert!(option.is_none());
        option.replace(op);
    }

    fn get_operation_infos_from_path_item(
        path: String,
        path_item: &PathItem,
    ) -> Vec<OperationInfo> {
        let mut res = vec![];
        if path_item.get.is_some() {
            let method = Method::GET;
            let operation = path_item.get.clone().unwrap();
            res.push(OperationInfo {
                path: path.to_string(),
                method,
                operation,
            });
        }
        if path_item.put.is_some() {
            let method = Method::PUT;
            let operation = path_item.put.clone().unwrap();
            res.push(OperationInfo {
                path: path.to_string(),
                method,
                operation,
            });
        }
        if path_item.post.is_some() {
            let method = Method::POST;
            let operation = path_item.post.clone().unwrap();
            res.push(OperationInfo {
                path: path.to_string(),
                method,
                operation,
            });
        }
        if path_item.delete.is_some() {
            let method = Method::DELETE;
            let operation = path_item.delete.clone().unwrap();
            res.push(OperationInfo {
                path: path.to_string(),
                method,
                operation,
            });
        }
        if path_item.options.is_some() {
            let method = Method::OPTIONS;
            let operation = path_item.options.clone().unwrap();
            res.push(OperationInfo {
                path: path.to_string(),
                method,
                operation,
            });
        }
        if path_item.head.is_some() {
            let method = Method::HEAD;
            let operation = path_item.head.clone().unwrap();
            res.push(OperationInfo {
                path: path.to_string(),
                method,
                operation,
            });
        }
        if path_item.patch.is_some() {
            let method = Method::PATCH;
            let operation = path_item.patch.clone().unwrap();
            res.push(OperationInfo {
                path: path.to_string(),
                method,
                operation,
            });
        }
        if path_item.trace.is_some() {
            let method = Method::TRACE;
            let operation = path_item.trace.clone().unwrap();
            res.push(OperationInfo {
                path,
                method,
                operation,
            });
        }
        res
    }
}

#[derive(Debug, Clone)]
pub struct OperationInfo {
    pub path: String,
    pub method: http::Method,
    pub operation: Operation,
}

impl From<OpenApi> for OpenApiGenerator {
    fn from(openapi: OpenApi) -> Self {
        let mut sts = SchemaSettings::openapi3();
        sts.option_add_null_type = false;
        sts.option_nullable = false;
        let generator = SchemaGenerator::new(sts);

        let mut openapigenerator = OpenApiGenerator {
            schema_generator: generator,
            operations: HashMap::default(),
            components: Components::default(),
            openapi: OpenApi::default(),
        };

        for (path, path_item) in openapi.paths.clone() {
            let op_infs = Self::get_operation_infos_from_path_item(path, &path_item);
            if op_infs.len() > 1 {
                println!("{:#?}", op_infs);
            }
            for mut op_inf in op_infs {
                let op = &mut op_inf.operation;

                // Okapi limitation
                op.responses.extensions.clear();
                for refor in &mut op.parameters {
                    if let RefOr::Object(param) = refor {
                        param.extensions.clear();
                    }
                }

                openapigenerator.add_operation(op_inf);
            }
        }

        if let Some(components) = openapi.components.clone() {
            openapigenerator.components = components.clone();
            openapigenerator.components.schemas.clear();
            let defs = openapigenerator.schema_generator.definitions_mut();
            for (name, schema) in components.schemas {
                defs.insert(name, schema.into());
            }
        }

        openapigenerator.openapi = openapi;
        openapigenerator.openapi.paths.clear();
        openapigenerator.openapi.components = None;

        openapigenerator
    }
}

#[cfg(test)]
mod tests {
    use crate::{generator::OpenApi, generator::OpenApiGenerator, ApiId, ApiPath, Oas3Builder};
    use openapiv3::OpenAPI;
    use schemars::JsonSchema;
    use serde::Serialize;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn generator_from_file() {
        let file = File::open("openapi_test.json").unwrap();
        let reader = BufReader::new(file);
        let openapi: OpenApi = serde_json::from_reader(reader).unwrap();
        let generator: OpenApiGenerator = openapi.into();
        let mut builder: Oas3Builder = generator.into();

        #[derive(Serialize, JsonSchema)]
        pub struct AdditionalData {
            pub title: String,
        }

        let additional_path = ApiPath::new(
            Some("api".to_owned()),
            vec![ApiId::new("z_data", "{lid}")],
            Some("data".to_owned()),
        );
        builder.delete_by_key::<AdditionalData, String>(&additional_path, "Data".to_owned(), None);

        let json_str = serde_json::to_string_pretty(&builder.build("1.0.1".to_owned()));
        let json_str = json_str.unwrap_or_default();

        let _openapi_json: OpenAPI =
            serde_json::from_str(&json_str).expect("Could not deserialize input");
        let _res = std::fs::write("openapi_additional_test.json", json_str.clone());
    }

    #[test]
    fn oas30_roundtrip_petstore_expanded() {
        let file = File::open("testdata/oas30_petstore-expanded.json").unwrap();
        let reader = BufReader::new(file);
        let openapi: OpenApi = serde_json::from_reader(reader).unwrap();
        let generator: OpenApiGenerator = openapi.clone().into();
        let builder: Oas3Builder = generator.into();

        let built_spec = &builder.build("1.0.0".to_owned());
        let json_str = serde_json::to_string_pretty(built_spec);
        let json_str = json_str.unwrap_or_default();
        let _openapi_json: OpenAPI =
            serde_json::from_str(&json_str).expect("Could not deserialize input");

        let built_openapi: OpenApi = serde_json::from_str(&json_str).unwrap();
        similar_asserts::assert_eq!(&openapi, &built_openapi);
    }
}

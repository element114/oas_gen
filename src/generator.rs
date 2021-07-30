use http::Method;
use schemars::gen::SchemaGenerator;
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
}
impl OpenApiGenerator {
    pub fn new(generator: SchemaGenerator) -> Self {
        OpenApiGenerator {
            schema_generator: generator,
            operations: HashMap::default(),
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
                    Self::set_operation(path_item, &method, op);
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
                ..Components::default()
            }),
            ..OpenApi::default()
        }
    }

    fn set_operation(path_item: &mut PathItem, method: &http::Method, op: Operation) {
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
}

pub struct OperationInfo {
    pub path: String,
    pub method: http::Method,
    pub operation: Operation,
}

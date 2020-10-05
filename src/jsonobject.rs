use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[schemars(transparent)]
pub struct JsonObject {
    #[serde(flatten)]
    map: serde_json::Map<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::JsonObject;
    use schemars::schema_for;

    #[test]
    fn test_json_object_schema() {
        let schema = schema_for!(JsonObject);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        assert_eq!(
            serde_json::json!({
                "$schema": "http://json-schema.org/draft-07/schema#",
                "title": "Map_of_AnyValue",
                "additionalProperties": true,
                "type": "object"
            }),
            serde_json::json!(&schema)
        );
    }
}

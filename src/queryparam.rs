use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;

use crate::generator::{Example, Map, Parameter, ParameterStyle, ParameterValue};

#[derive(Debug, Clone)]
pub struct QueryParamBuilder {
    param: Parameter,
    schema_generator: SchemaGenerator,
}
impl QueryParamBuilder {
    #[must_use]
    pub fn new<T: JsonSchema + Serialize>(name: String, example: Option<T>) -> Self {
        let mut schema_generator = SchemaGenerator::new(SchemaSettings::openapi3());
        let example = example.map(|ex| serde_json::to_value(&ex).unwrap_or_default());
        let param_name = name;
        let param_schema = ParameterValue::Schema {
            style: None,
            explode: None,
            allow_reserved: false,
            schema: schema_generator.subschema_for::<T>().into(),
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

    #[must_use]
    pub fn build(&self) -> Parameter {
        self.param.clone()
    }

    #[must_use]
    pub fn description(&self, description: String) -> Self {
        let mut me = self.clone();
        me.param.description = Some(description);
        me
    }

    #[must_use]
    pub fn required(&self, required: bool) -> Self {
        let mut me = self.clone();
        me.param.required = required;
        me
    }

    #[must_use]
    pub fn deprecated(&self, deprecated: bool) -> Self {
        let mut me = self.clone();
        me.param.deprecated = deprecated;
        me
    }

    #[must_use]
    pub fn allow_empty_value(&self, allow_empty_value: bool) -> Self {
        let mut me = self.clone();
        me.param.allow_empty_value = allow_empty_value;
        me
    }

    #[must_use]
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

    #[must_use]
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

    #[must_use]
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

    #[must_use]
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

    #[must_use]
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

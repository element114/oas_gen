pub mod openapi3;

#[cfg(test)]
mod tests {
    use crate::openapi3::*;
    use schemars::{JsonSchema};
    use serde::Serialize;
    use openapiv3::OpenAPI;

    #[test]
    fn it_works() {
        let mut oasb = Oas3Builder::default();

        #[derive(Serialize, JsonSchema)]
        pub struct CollectionWrapper<T> {
            collection: Vec<T>
        }

        #[derive(Serialize, JsonSchema)]
        pub struct TestEvent {
            pub title: String,
        }

        #[derive(Serialize, JsonSchema)]
        pub struct TestEventForm {
            pub title: String,
        }

        // list events
        oasb.list::<CollectionWrapper<TestEvent>>(
            "/api/organizers/{oid}/events".to_owned(),
            "Events".to_owned(),
            Some("{oid}".to_owned()),
        );

        // fetch event
        oasb.fetch::<TestEvent>(
            "/api/organizers/{oid}/events/{eid}".to_owned(),
            "Events".to_owned(),
            Some("{oid}".to_owned()),
            "{eid}".to_owned(),
        );

        // create event
        oasb.create::<TestEvent, TestEvent>(
            "/api/organizers/{oid}/events".to_owned(),
            "Events".to_owned(),
            Some("{oid}".to_owned()),
        );

        // update event
        oasb.update::<TestEvent, TestEvent>(
            "/api/organizers/{oid}/events/{eid}".to_owned(),
            "Events".to_owned(),
            Some("{oid}".to_owned()),
            "{eid}".to_owned(),
        );
        
        // replace event
        oasb.replace::<TestEvent, TestEvent>(
            "/api/organizers/{oid}/events/{eid}".to_owned(),
            "Events".to_owned(),
            Some("{oid}".to_owned()),
            "{eid}".to_owned(),
        );

        // delete event
        oasb.delete::<TestEvent>(
            "/api/organizers/{oid}/events/{eid}".to_owned(),
            "Events".to_owned(),
            Some("{oid}".to_owned()),
            "{eid}".to_owned(),
        );

        let json_str = serde_json::to_string_pretty(&oasb.generator.into_openapi());
        let json_str = json_str.unwrap_or_default();

        let _openapi_json: OpenAPI = serde_json::from_str(&json_str).expect("Could not deserialize input");

        let _res = std::fs::write("openapi_test.json", json_str.clone());

        // assert_eq!("", json_str);
    }
}

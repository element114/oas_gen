pub mod openapi3;

#[cfg(test)]
mod tests {
    use crate::openapi3::*;
    use openapiv3::OpenAPI;
    use schemars::JsonSchema;
    use serde::Serialize;

    #[test]
    fn it_works() {
        let mut oasb = Oas3Builder::default();

        #[derive(Serialize, JsonSchema)]
        pub struct CollectionWrapper<T> {
            collection: Vec<T>,
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
        let list_path = ApiPath::new(
            Some("api".to_owned()),
            vec!(ApiId::new("organizers","{oid}")),
            Some("events".to_owned()),
        );
        oasb.list::<CollectionWrapper<TestEvent>>(
            &list_path,
            "Events".to_owned(),
        );

        // fetch event
        let fetch_path = ApiPath::new(
            Some("api".to_owned()),
            vec!(
                ApiId::new("organizers","{oid}"),
                ApiId::new("events","{eid}"),
            ),
            None,
        );
        oasb.fetch::<TestEvent>(
            &fetch_path,
            "Events".to_owned(),
        );

        // create event
        oasb.create::<TestEvent, TestEvent>(
            &list_path,
            "Events".to_owned(),
        );

        // update event
        oasb.update::<TestEvent, TestEvent>(
            &fetch_path,
            "Events".to_owned(),
        );

        // replace event
        oasb.replace::<TestEvent, TestEvent>(
            &fetch_path,
            "Events".to_owned(),
        );

        // delete event
        oasb.delete::<TestEvent>(
            &fetch_path,
            "Events".to_owned(),
        );

        let json_str = serde_json::to_string_pretty(&oasb.generator.into_openapi());
        let json_str = json_str.unwrap_or_default();

        let _openapi_json: OpenAPI =
            serde_json::from_str(&json_str).expect("Could not deserialize input");

        let _res = std::fs::write("openapi_test.json", json_str.clone());

        // assert_eq!("", json_str);
    }
}

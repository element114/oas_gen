#![forbid(unsafe_code)]

mod oasgen;
mod okapi3;

pub use oasgen::*;

#[cfg(test)]
mod tests {
    use crate::*;
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

        let limit_param = QueryParamBuilder::new::<u64>("limit".to_owned(), Some(std::u64::MAX));
        let categories_param = QueryParamBuilder::new::<Vec<String>>(
            "categories".to_owned(),
            Some(vec![
                "Financial Education".to_owned(),
                "Work safety training".to_owned(),
            ]),
        );
        let categories_param = categories_param.explode(false);
        let qpbs = vec![limit_param, categories_param];
        // list events
        let list_path = ApiPath::with_queries(
            Some("api".to_owned()),
            vec![ApiId::new("organizers", "{oid}")],
            Some("events".to_owned()),
            qpbs,
        );
        oasb.list::<CollectionWrapper<TestEvent>>(&list_path, "Events".to_owned(), None);

        // fetch event
        let fetch_path = ApiPath::new(
            Some("api".to_owned()),
            vec![
                ApiId::new("organizers", "{oid}"),
                ApiId::new("events", "{eid}"),
            ],
            None,
        );
        oasb.fetch::<TestEvent>(&fetch_path, "Events".to_owned(), None);

        // create event
        let create_path = ApiPath::new(
            Some("api".to_owned()),
            vec![ApiId::new("organizers", "{oid}")],
            Some("events".to_owned()),
        );
        oasb.create::<TestEventForm, TestEvent>(&create_path, "Events".to_owned(), None);

        // update event
        oasb.update::<TestEventForm, TestEvent>(&fetch_path, "Events".to_owned(), None);

        // replace event
        oasb.replace::<TestEventForm, TestEvent>(&fetch_path, "Events".to_owned(), None);

        // delete event
        oasb.delete::<TestEvent>(&fetch_path, "Events".to_owned(), None);

        // any operation: find event by title
        let title_param = QueryParamBuilder::new::<String>("title".to_owned(), Some("Hackaton 2020 01 23".to_owned()));
        let qpbs = vec![title_param];
        let find_path = ApiPath::with_queries(
            Some("api".to_owned()),
            vec![ApiId::new("organizers", "{oid}")],
            Some("events/find".to_owned()),
            qpbs,
        );
        oasb.any::<(), TestEvent>(
            &find_path,
            http::Method::GET,
            "Events".to_owned(),
            "Find".to_owned(),
            Some("Find and event by it's title".to_owned()),
        );

        let json_str = serde_json::to_string_pretty(&oasb.build());
        let json_str = json_str.unwrap_or_default();

        let _openapi_json: OpenAPI =
            serde_json::from_str(&json_str).expect("Could not deserialize input");

        let _res = std::fs::write("openapi_test.json", json_str.clone());

        // assert_eq!("", json_str);
    }
}

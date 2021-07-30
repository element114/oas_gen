#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(non_fmt_panic)]

mod any;
mod apipath;
mod create;
mod delete;
mod fetch;
mod list;
mod queryparam;
mod replace;
mod update;

pub mod jsonobject;
mod oasgen;
// mod okapi3;
mod generator;
pub mod xtests;

pub use any::*;
pub use create::*;
pub use delete::*;
pub use fetch::*;
pub use list::*;
pub use replace::*;
pub use update::*;

pub use apipath::*;
pub use oasgen::*;
// pub use okapi3::{RefOr, SecurityScheme};
pub use queryparam::*;

#[cfg(test)]
mod tests {
    use crate::{ApiId, ApiPath, Oas3Builder, QueryParamBuilder};
    use openapiv3::OpenAPI;
    use schemars::JsonSchema;
    use serde::Serialize;

    #[test]
    fn it_works() {
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

        let mut oasb = Oas3Builder::default();

        let limit_param = QueryParamBuilder::new::<u64>("limit".to_owned(), Some(u64::max_value()));
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
        oasb.list::<CollectionWrapper<TestEvent>, String>(&list_path, "Events".to_owned(), None);

        // fetch event
        let fetch_path = ApiPath::new(
            Some("api".to_owned()),
            vec![
                ApiId::new("organizers", "{oid}"),
                ApiId::new("events", "{eid}"),
            ],
            None,
        );
        oasb.fetch::<TestEvent, String>(&fetch_path, "Events".to_owned(), None);

        // create event
        let create_path = ApiPath::new(
            Some("api".to_owned()),
            vec![ApiId::new("organizers", "{oid}")],
            Some("events".to_owned()),
        );
        oasb.create::<TestEventForm, TestEvent, String>(&create_path, "Events".to_owned(), None);

        // update event
        oasb.update::<TestEventForm, TestEvent, String>(&fetch_path, "Events".to_owned(), None);

        // replace event
        oasb.replace::<TestEventForm, TestEvent, String>(&fetch_path, "Events".to_owned(), None);

        // delete event
        oasb.delete::<TestEventForm, TestEvent, String>(&fetch_path, "Events".to_owned(), None);

        // delete event
        oasb.delete_by_key::<TestEvent, String>(&create_path, "Events".to_owned(), None);

        // any operation: find event by title
        let title_param = QueryParamBuilder::new::<String>(
            "title".to_owned(),
            Some("Hackaton 2020 01 23".to_owned()),
        );
        let qpbs = vec![title_param];
        let find_path = ApiPath::with_queries(
            Some("api".to_owned()),
            vec![ApiId::new("organizers", "{oid}")],
            Some("events/find".to_owned()),
            qpbs,
        );
        oasb.any::<(), TestEvent, String>(
            &find_path,
            http::Method::GET,
            "Events".to_owned(),
            "Find",
            Some("Find and event by it's title".to_owned()),
        );

        let json_str = serde_json::to_string_pretty(&oasb.build("1.0.0".to_owned()));
        let json_str = json_str.unwrap_or_default();

        let _openapi_json: OpenAPI =
            serde_json::from_str(&json_str).expect("Could not deserialize input");

        let _res = std::fs::write("openapi_test.json", json_str);

        // assert_eq!("", json_str);
    }
}

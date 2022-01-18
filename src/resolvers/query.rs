use juniper::{FieldResult, graphql_object};
use crate::models::notebook::Notebook;

pub struct Query;

#[graphql_object()]
impl Query {
    pub fn apiVersion() -> &'static str {
        "1.0"
    }
}

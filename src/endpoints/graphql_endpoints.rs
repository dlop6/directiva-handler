use juniper::{EmptyMutation, RootNode};
use crate::endpoints::handlers::graphql::directiva::queries::DirectivaQuery;


pub struct Query;

#[juniper::graphql_object]
impl Query {
    fn directiva(&self) -> DirectivaQuery {
        DirectivaQuery
    }


}

pub type Schema = RootNode<'static, Query, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new())
}
use juniper::{EmptyMutation, EmptySubscription, RootNode}; 
use crate::endpoints::handlers::graphql::{
    directiva::queries::DirectivaQuery,
    moras::queries::MoraQuery,
    pagos::queries::PagoQuery,
    prestamos::queries::PrestamoQuery
};

pub struct Query;

#[juniper::graphql_object]
impl Query {
    async fn directiva(&self) -> DirectivaQuery {
        DirectivaQuery
    }

    async fn moras(&self) -> MoraQuery {
        MoraQuery
    }

    async fn pagos(&self) -> PagoQuery {
        PagoQuery
    }

    async fn prestamos(&self) -> PrestamoQuery {
        PrestamoQuery
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}
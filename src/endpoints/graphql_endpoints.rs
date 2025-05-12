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
    fn directiva(&self) -> DirectivaQuery {
        DirectivaQuery
    }

    fn moras(&self) -> MoraQuery {
        MoraQuery
    }

    fn pagos(&self) -> PagoQuery {
        PagoQuery
    }

    fn prestamos(&self) -> PrestamoQuery {
        PrestamoQuery
    }
}

// Corrige el Schema añadiendo EmptySubscription
pub type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(
        Query {}, 
        EmptyMutation::new(), 
        EmptySubscription::new()
    )
}
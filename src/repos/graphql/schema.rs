use juniper::RootNode;
use crate::models::{Estados, Loan};

// db context
pub struct Context {
    pub pool: sqlx::PgPool,
}

impl juniper::Context for Context {}

// main schema
pub type Schema = RootNode<'static, Query, juniper::EmptyMutation<Context>>;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn moras(&self) -> moras::MoraQuery {
        moras::MoraQuery
    }

    fn pagos(&self) -> pagos::PagoQuery {
        pagos::PagoQuery
    }

    fn prestamos(&self) -> prestamos::PrestamoQuery {
        prestamos::PrestamoQuery
    }
}
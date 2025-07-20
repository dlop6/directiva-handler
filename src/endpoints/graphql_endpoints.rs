use crate::endpoints::handlers::graphql::{
    directiva::queries::DirectivaQuery,
    moras::queries::MoraQuery,
    pagos::queries::PagoQuery,
    prestamos::queries::PrestamoQuery,
    graphql_context::Context,
};
use deadpool_postgres::Pool;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use actix_web::web;

/// Tipo del schema, ahora con nuestro Contexto
pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn directiva(context: &Context) -> DirectivaQuery {
        DirectivaQuery { pg_client: context.pg_client.clone() }
    }

    async fn moras(context: &Context) -> MoraQuery {
        MoraQuery { pg_client: context.pg_client.clone() }
    }

    async fn pagos(context: &Context) -> PagoQuery {
        PagoQuery { pg_client: context.pg_client.clone() }
    }

    async fn prestamos(context: &Context) -> PrestamoQuery {
        PrestamoQuery { pg_client: context.pg_client.clone() }
    }
}

/// Crea el schema, inyectando el Contexto
pub fn create_schema(pg_pool: Pool) -> Schema {
    Schema::new_with_ctx(Query {}, EmptyMutation::new(), EmptySubscription::new(), Context { pg_client: pg_pool })
}

/// Configura las rutas y el schema en Actix
pub fn configure(cfg: &mut web::ServiceConfig, pg_pool: Pool) {
    let schema = std::sync::Arc::new(create_schema(pg_pool));
    cfg.app_data(web::Data::new(schema.clone()));
    cfg.service(
        web::resource("/graphql")
            .route(web::post().to(graphql_handler))
    )
    .service(
        web::resource("/graphiql")
            .route(web::get().to(graphiql)
        )
    ;
}

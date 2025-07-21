use crate::endpoints::handlers::graphql::{
    directiva::queries::DirectivaQuery,
    moras::queries::MoraQuery,
    pagos::queries::PagoQuery,
    prestamos::queries::PrestamoQuery,
};
use crate::endpoints::graphql_context::Context;
use deadpool_postgres::Pool;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use actix_web::{web, HttpResponse, Result};

/// Tipo del schema, ahora sin contexto embebido
pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn directiva(_context: &Context) -> DirectivaQuery {
        DirectivaQuery
    }

    async fn moras(_context: &Context) -> MoraQuery {
        MoraQuery
    }

    async fn pagos(_context: &Context) -> PagoQuery {
        PagoQuery
    }

    async fn prestamos(_context: &Context) -> PrestamoQuery {
        PrestamoQuery
    }
}

/// Crea el schema, inyectando el Contexto
pub fn create_schema(_pg_pool: Pool) -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}

/// Handler para las queries GraphQL
pub async fn graphql_handler(
    schema: web::Data<Schema>,
    data: web::Json<juniper::http::GraphQLRequest>,
    pg_pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let ctx = Context { pg_client: pg_pool.as_ref().clone() };
    let res = data.execute(&schema, &ctx).await;
    Ok(HttpResponse::Ok().json(res))
}

/// Handler para GraphiQL UI
pub async fn graphiql() -> Result<HttpResponse> {
    let html = juniper::http::graphiql::graphiql_source("/graphql", None);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
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
            .route(web::get().to(graphiql))
    );
}

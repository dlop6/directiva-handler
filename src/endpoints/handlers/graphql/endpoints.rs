use actix_web::{web, Result, HttpResponse};
use juniper::http::{GraphQLRequest, playground::playground_source};
use super::super::configs::schema::{DirectivaContext, create_schema, QueryRoot, MutationRoot};

/// Handler para queries GraphQL siguiendo patrón general-handler exacto
pub async fn graphql_handler(
    schema: web::Data<juniper::RootNode<'static, QueryRoot, MutationRoot, juniper::EmptySubscription<DirectivaContext>>>,
    data: web::Json<GraphQLRequest>,
    context: web::Data<DirectivaContext>,
) -> Result<HttpResponse> {
    let res = data.execute(&schema, &context).await;
    Ok(HttpResponse::Ok().json(res))
}

/// Handler para GraphQL Playground UI
pub async fn playground_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(playground_source("/graphql", None)))
}

/// Configuración de rutas GraphQL siguiendo patrón general-handler
pub fn config(cfg: &mut web::ServiceConfig) {
    let schema = create_schema();
    
    cfg.app_data(web::Data::new(schema))
        .route("/graphql", web::post().to(graphql_handler))
        .route("/playground", web::get().to(playground_handler));
}

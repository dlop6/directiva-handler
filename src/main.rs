use actix_web::{web, App, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use std::sync::Arc;
use endpoints::graphql_endpoints::{Schema, create_schema};

mod endpoints;
mod models;

async fn graphql_handler(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<juniper::http::GraphQLRequest>,
) -> HttpResponse {
    let response = data.execute(&schema, &()).await; // Añade .await aquí
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_handler))
            .route("/graphiql", web::get().to(|| async {
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(graphiql_source("/graphql", None))
            }))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
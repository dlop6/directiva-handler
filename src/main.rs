// src/main.rs

mod config;
mod endpoints;
mod models;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use config::Config;
use dotenv::dotenv;
use env_logger::Env;
use juniper::http::graphiql::graphiql_source;
use std::sync::Arc;

use endpoints::graphql_endpoints::{create_schema, Schema};

/// Handler para todas las peticiones GraphQL.
async fn graphql_handler(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<juniper::http::GraphQLRequest>,
) -> HttpResponse {
    let response = data.execute(&schema, &()).await;
    HttpResponse::Ok().json(response)
}

/// Entry point de la aplicación.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Cargar variables de entorno desde .env si existe
    let _ = dotenv();

    // Inicializar logger usando RUST_LOG si está en el entorno
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Cargar configuración
    let cfg = Config::init().expect("Falló al cargar la configuración");

    println!("🔗 Conectando a Postgres en: {}", cfg.database_url);
    println!("🔗 Conectando a Redis en: {}", cfg.redis_url);
    println!("🚀 Servidor corriendo en: http://{}", cfg.server_addr());

    // Crear el schema de GraphQL
    let schema = Arc::new(create_schema());

    // Levantar el servidor HTTP/HTTPS
    HttpServer::new(move || {
        // Configuración de CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            // Middleware de CORS
            .wrap(cors)
            // Compartir el schema con todos los handlers
            .app_data(web::Data::new(schema.clone()))
            // Rutas GraphQL y GraphiQL
            .route("/graphql", web::post().to(graphql_handler))
            .route("/graphiql", web::get().to(|| async {
                HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(graphiql_source("/graphql", None))
            }))
    })
    .bind(cfg.server_addr())?
    .run()
    .await
}

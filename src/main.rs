use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use directiva_handler::config::Env;
use directiva_handler::configs::{get_pool_connection, DirectivaContext};
use directiva_handler::endpoints::handlers::{create_schema, graphql_config, rest_config};
use std::sync::Arc;

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(directiva_handler::models::general::GeneralInfo {
        api_version: "1.0.0".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config: Env = Env::env_init();
    println!("Server Running in: {}:{}", config.host, config.port);

    // Inicializar pool de conexiones Redis siguiendo patrón general-handler
    let redis_pool = Arc::new(get_pool_connection());
    let context = web::Data::new(DirectivaContext::new(Arc::clone(&redis_pool)));
    let schema = web::Data::new(create_schema());

    HttpServer::new(move || {
        let cors = if cfg!(debug_assertions) {
            Cors::permissive()
        } else {
            Cors::permissive() // Same for both for now
        };

        App::new()
            .wrap(cors)
            .app_data(context.clone())
            .app_data(schema.clone())
            .route("/health", web::get().to(health))
            .configure(rest_config)
            .configure(graphql_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}

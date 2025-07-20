// src/main.rs

mod config;
mod endpoints;
mod models;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, middleware::{Logger, DefaultHeaders}};
use config::Config;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod};
use dotenv::dotenv;
use env_logger::Env;
use juniper::http::graphiql::graphiql_source;
use std::sync::Arc;
use tokio_postgres::NoTls;

use endpoints::graphql_endpoints::{create_schema, Schema};
use endpoints::graphql_context::Context;

/// Handler para la interfaz interactiva GraphiQL (solo en desarrollo)
async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

/// Handler para todas las peticiones GraphQL.
async fn graphql_handler(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<juniper::http::GraphQLRequest>,
    pg_pool: web::Data<Pool>,
) -> HttpResponse {
    let ctx = Context {
        pg_client: pg_pool.get_ref().clone(),
    };
    let response = data.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1) Cargar .env y configurar logger
    let _ = dotenv();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // 2) Cargar configuración
    let cfg = Config::init().expect("Falló al cargar la configuración");
    println!("🚀 Ambiente:     {}", cfg.environment);
    println!("🔗 Postgres URL: {}", cfg.database_url);
    println!("🔗 Redis URL:    {}", cfg.redis_url);
    println!("🌐 CORS Origin:  {}", cfg.cors_origin());
    println!("🚀 Server on:    http://{}", cfg.server_addr());

    // 3) Crear pool de Postgres
    let mut pg_cfg = deadpool_postgres::Config::new();
    pg_cfg.url = Some(cfg.database_url.clone());
    pg_cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pg_client: Pool =
        pg_cfg.create_pool(NoTls).expect("No se pudo crear el pool de Postgres");

    // 4) Crear schema de GraphQL con contexto
    let schema = Arc::new(create_schema(pg_client.clone()));

    // 5) Arrancar servidor HTTP
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.cors_origin())
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .max_age(3600);

        let mut app = App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(pg_client.clone()))
            .service(web::resource("/graphql").route(web::post().to(graphql_handler)));

        // Solo habilitar GraphiQL en desarrollo
        if !cfg.is_production() {
            app = app.service(web::resource("/graphiql").route(web::get().to(graphiql)));
        }

        // En producción, agregar headers de seguridad
        if cfg.is_production() {
            app = app.wrap(
                DefaultHeaders::new()
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
            );
        }

        app
    })
    .bind(cfg.server_addr())?
    .run()
    .await
}

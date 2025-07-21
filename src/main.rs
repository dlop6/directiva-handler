// src/main.rs

mod config;
mod endpoints;
mod models;
mod repos;

use actix_web::{web, App, HttpServer, middleware::{Logger, DefaultHeaders}};
use actix_cors::Cors;
use config::Config;
use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod};
use dotenv::dotenv;
use env_logger::Env;
use std::sync::Arc;
use tokio_postgres::NoTls;

use endpoints::graphql_endpoints::{create_schema, graphql_handler, graphiql};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // cargar .env y configurar logger
    let _ = dotenv();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // cargar configuración
    let cfg = Config::init().expect("Falló al cargar la configuración");
    println!("🚀 Ambiente:     {}", cfg.environment);
    println!("🔗 Postgres URL: {}", cfg.database_url);
    println!("🔗 Redis URL:    {}", cfg.redis_url);
    println!("🌐 CORS Origin:  {}", cfg.cors_origin());
    println!("🚀 Server on:    http://{}", cfg.server_addr());

    // crear pool de postgres
    let mut pg_cfg = deadpool_postgres::Config::new();
    // para desarrollo, configuramos manualmente
    pg_cfg.user = Some("directiva_user".to_string());
    pg_cfg.password = Some("directiva_pass".to_string());
    pg_cfg.host = Some("localhost".to_string());
    pg_cfg.port = Some(5432);
    pg_cfg.dbname = Some("directiva_db".to_string());
    pg_cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pg_client: Pool =
        pg_cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .expect("No se pudo crear el pool de Postgres");

    // crear schema de graphql
    let schema = Arc::new(create_schema(pg_client.clone()));

    // arrancar servidor http
    let server = HttpServer::new({
        let schema = schema.clone();
        let pg_client = pg_client.clone();
        let cfg = cfg.clone();
        
        move || {
            let cors = Cors::default()
                .allowed_origin(&cfg.cors_origin())
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allowed_headers(vec!["Content-Type", "Authorization"])
                .max_age(3600);

            // middleware de seguridad solo en prod
            let security_headers = if cfg.is_production() {
                DefaultHeaders::new()
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
            } else {
                DefaultHeaders::new() // headers vacíos para dev
            };

            // crear la app
            let mut app = App::new()
                .wrap(Logger::default())
                .wrap(cors)
                .wrap(security_headers)
                .app_data(web::Data::new(schema.clone()))
                .app_data(web::Data::new(pg_client.clone()))
                .service(web::resource("/graphql").route(web::post().to(graphql_handler)));

            // graphiql solo en desarrollo
            if !cfg.is_production() {
                app = app.service(web::resource("/graphiql").route(web::get().to(graphiql)));
            }

            app
        }
    });

    server
        .bind(cfg.server_addr())?
        .run()
        .await
}

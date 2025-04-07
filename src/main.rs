use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // cargar  .env
    dotenv().ok();

    
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    // crear y lanzar el servidor Actix
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hola Mundo" }))
    })
    .bind(format!("0.0.0.0:{}", server_port))?
    .run()
    .await
}
// src/config.rs

use envconfig::Envconfig;
use dotenv::dotenv;
use std::net::SocketAddr;

/// Configuración de la aplicación cargada desde variables de entorno.
#[derive(Envconfig, Debug)]
pub struct Config {
    /// URL de conexión a PostgreSQL, e.g. "postgres://user:pass@host:5432/dbname"
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    /// URL de conexión a Redis, e.g. "redis://:pass@host:6379/"
    #[envconfig(from = "REDIS_URL")]
    pub redis_url: String,

    /// Puerto donde corre el servidor HTTP, e.g. "8080"
    #[envconfig(from = "SERVER_PORT", default = "8080")]
    pub server_port: u16,

    /// Ambiente de ejecución (development, staging, production)
    #[envconfig(from = "RUST_ENV", default = "development")]
    pub environment: String,

    /// Dominio base para CORS (ej: dev.directiva.cooperativa-isp.cc)
    #[envconfig(from = "ALLOWED_ORIGIN", default = "http://localhost:3000")]
    pub allowed_origin: String,
}

impl Config {
    ///  cargar de .env
    pub fn init() -> Result<Self, envconfig::Error> {
        // Cargar variables desde un archivo .env si existe
        let _ = dotenv();
        //  las variables de entorno
        Config::init_from_env()
    }

    ///dirección SocketAddr para bind del servidor.
    pub fn server_addr(&self) -> SocketAddr {
        // En producción bindeamos a 0.0.0.0 para permitir conexiones externas
        // En desarrollo solo a localhost
        let host = if self.is_production() {
            "0.0.0.0"
        } else {
            "127.0.0.1"
        };
        format!("{}:{}", host, self.server_port)
            .parse()
            .expect("Invalid server address")
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    pub fn cors_origin(&self) -> String {
        if self.is_production() {
            format!("https://{}", self.allowed_origin)
        } else {
            "http://localhost:3000".to_string()
        }
    }
}

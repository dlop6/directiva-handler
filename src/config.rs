// src/config.rs

use envconfig::Envconfig;
use std::env;
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
}

impl Config {
    /// Carga la configuración leyendo primero un posible `.env`, luego las variables de entorno.
    pub fn init() -> Result<Self, envconfig::Error> {
        // Si tienes un archivo .env en tu proyecto, lo cargamos aquí:
        let _ = dotenv::dotenv();

        // Inicializamos desde las variables de entorno
        Config::init_from_env()
    }

    /// Devuelve la dirección SocketAddr para bind del servidor.
    pub fn server_addr(&self) -> SocketAddr {
        let host = "0.0.0.0";
        let port = self.server_port;
        format!("{host}:{port}")
            .parse()
            .expect("SERVER_PORT debe ser un número válido entre 0 y 65535")
    }
}

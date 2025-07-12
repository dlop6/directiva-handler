

use envconfig::Envconfig;
use std::env;
use std::net::SocketAddr;

#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    #[envconfig(from = "REDIS_URL")]
    pub redis_url: String,

    #[envconfig(from = "SERVER_PORT", default = "8080")]
    pub server_port: u16,
}

impl Config {
    pub fn init() -> Result<Self, envconfig::Error> {

        let _ = dotenv::dotenv();

        Config::init_from_env()
    }

    pub fn server_addr(&self) -> SocketAddr {
        let host = "0.0.0.0";
        let port = self.server_port;
        format!("{host}:{port}")
            .parse()
            .expect("SERVER_PORT debe ser un número válido entre 0 y 65535")
    }
}

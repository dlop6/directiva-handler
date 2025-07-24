use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct Env {
    #[envconfig(from = "HOST", default = "localhost")]
    pub host: String,

    #[envconfig(from = "PORT", default = "8080")]
    pub port: String,

    #[envconfig(from = "REDIS_URL", default = "redis://127.0.0.1/")]
    pub redis_url: String,
}

impl Env {
    pub fn env_init() -> Env {
        Env::init_from_env().unwrap()
    }
}

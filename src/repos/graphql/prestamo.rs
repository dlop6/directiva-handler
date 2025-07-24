use r2d2::Pool;
use redis::Client;
use std::sync::Arc;

pub struct PrestamoRepo {
    pool: Arc<Pool<Client>>,
}

impl PrestamoRepo {
    pub fn new(pool: Arc<Pool<Client>>) -> Self {
        Self { pool }
    }

    pub fn get_all_prestamos(&self) -> Vec<String> {
        // TODO: Implementar consulta real a Redis con claves SHA256
        vec!["prestamo_dummy".to_string()]
    }

    pub fn get_prestamo_by_id(&self, _id: &str) -> Option<String> {
        // TODO: Implementar consulta real a Redis
        Some("prestamo_dummy".to_string())
    }
}

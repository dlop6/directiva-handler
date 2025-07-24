use r2d2::Pool;
use redis::Client;
use std::sync::Arc;

/// Repositorio para operaciones GraphQL de moras
pub struct MoraRepo {
    pool: Arc<Pool<Client>>,
}

impl MoraRepo {
    pub fn new(pool: Arc<Pool<Client>>) -> Self {
        Self { pool }
    }
    
    // TODO: Implementar operaciones reales con Redis
    pub fn get_all_moras(&self) -> Vec<String> {
        vec!["mora_dummy".to_string()]
    }
    
    pub fn get_moras_by_prestamo_id(&self, _prestamo_id: &str) -> Vec<String> {
        vec!["mora_dummy".to_string()]
    }
}

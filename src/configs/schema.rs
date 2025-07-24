use r2d2::Pool;
use redis::Client;
use std::sync::Arc;

/// Contexto GraphQL siguiendo patrón general-handler exacto
#[derive(Clone)]
pub struct DirectivaContext {
    pub pool: Arc<Pool<Client>>,
}

impl DirectivaContext {
    /// Crear nueva instancia del contexto
    pub fn new(pool: Arc<Pool<Client>>) -> Self {
        Self { pool }
    }
    
    /// Factory method para repositorio de préstamos
    pub fn prestamo_repo(&self) -> crate::repos::graphql::prestamo::PrestamoRepo {
        crate::repos::graphql::prestamo::PrestamoRepo::new(Arc::clone(&self.pool))
    }
    
    /// Factory method para repositorio de moras
    pub fn mora_repo(&self) -> crate::repos::graphql::mora::MoraRepo {
        crate::repos::graphql::mora::MoraRepo::new(Arc::clone(&self.pool))
    }
    
    /// Factory method para repositorio de usuarios/directiva
    pub fn usuario_repo(&self) -> crate::repos::rest::UsuarioRepo {
        crate::repos::rest::UsuarioRepo::new(Arc::clone(&self.pool))
    }
}

impl juniper::Context for DirectivaContext {}

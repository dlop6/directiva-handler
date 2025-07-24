use actix_web::web;
use juniper::{EmptySubscription, RootNode};
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
    
    /// Factory method para repositorio de pagos/moras
    pub fn mora_repo(&self) -> crate::repos::graphql::mora::MoraRepo {
        crate::repos::graphql::mora::MoraRepo::new(Arc::clone(&self.pool))
    }
    
    /// Factory method para repositorio de usuarios/directiva
    pub fn usuario_repo(&self) -> crate::repos::rest::auth::UsuarioRepo {
        crate::repos::rest::auth::UsuarioRepo::new(Arc::clone(&self.pool))
    }
}

impl juniper::Context for DirectivaContext {}

/// Query root para GraphQL - implementa las consultas de directivos
#[derive(Default)]
pub struct QueryRoot;

#[juniper::graphql_object(Context = DirectivaContext)]
impl QueryRoot {
    /// Obtener todos los préstamos supervisados por la directiva
    fn prestamos(context: &DirectivaContext) -> Vec<String> {
        context.prestamo_repo().get_all_prestamos()
    }
    
    /// Obtener préstamo específico por ID
    fn prestamo(context: &DirectivaContext, id: String) -> Option<String> {
        context.prestamo_repo().get_prestamo_by_id(&id)
    }
    
    /// Obtener todas las moras para supervisión
    fn moras(context: &DirectivaContext) -> Vec<String> {
        context.mora_repo().get_all_moras()
    }
    
    /// Información de la API
    fn api_info() -> String {
        "Directiva Handler API - v1.0.0".to_string()
    }
}

/// Mutation root placeholder - siguiendo patrón general-handler
#[derive(Default)]
pub struct MutationRoot;

#[juniper::graphql_object(Context = DirectivaContext)]
impl MutationRoot {
    /// Placeholder para futuras mutaciones
    fn placeholder() -> bool {
        true
    }
}

/// Crear schema GraphQL genérico siguiendo patrón general-handler
pub fn create_schema() -> RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<DirectivaContext>> {
    RootNode::new_with_info(
        QueryRoot::default(),
        MutationRoot::default(), 
        EmptySubscription::new(),
        (),
        (),
        (),
    )
}

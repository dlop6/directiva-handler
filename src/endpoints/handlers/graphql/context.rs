use actix_web::web;
use r2d2::Pool;
use redis::Client;
use std::sync::Arc;

/// Contexto GraphQL para directiva-handler siguiendo patrón exacto de general-handler
#[derive(Clone)]
pub struct Context {
    pub pool: Arc<Pool<Client>>,
}

impl Context {
    pub fn new(pool: Arc<Pool<Client>>) -> Self {
        Self { pool }
    }
}

impl juniper::Context for Context {}

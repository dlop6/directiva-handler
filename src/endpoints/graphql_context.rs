use deadpool_postgres::Pool;

/// Contexto que pasaremos a todos los resolvers de Juniper.
pub struct Context {
    /// Pool de clientes Postgres
    pub pg_client: Pool,
    
}

impl juniper::Context for Context {}

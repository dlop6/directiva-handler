use r2d2::Pool;
use redis::Client;
use std::time::Duration;

/// Crea y configura el pool de conexiones Redis siguiendo patrón general-handler exacto
pub fn get_pool_connection() -> Pool<Client> {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    
    let client = Client::open(redis_url)
        .unwrap_or_else(|_| panic!("Error: No se pudo conectar a Redis"));
    
    Pool::builder()
        .connection_timeout(Duration::from_secs(5))
        .build(client)
        .unwrap_or_else(|_| panic!("Error: No se pudo crear el pool de Redis"))
}

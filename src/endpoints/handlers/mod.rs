pub mod configs;
pub mod graphql; 
pub mod rest;

// Importar desde configs de handlers (no desde raíz)
pub use configs::{get_pool_connection, DirectivaContext, create_schema, QueryRoot, MutationRoot};
pub use graphql::{Context, graphql_handler, playground_handler, graphql_config};
pub use rest::{user_login, user_sign_up, rest_config, LoginRequest, SignUpRequest, AuthResponse};
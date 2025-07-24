// Repositorios Redis siguiendo patrón general-handler exacto
pub mod graphql;
pub mod rest;

pub use graphql::{PrestamoRepo, MoraRepo};
pub use rest::UsuarioRepo;

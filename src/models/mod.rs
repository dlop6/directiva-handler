// Nuevos modelos siguiendo patrón general-handler
pub mod general;
pub mod auth;
pub mod graphql;
pub mod om_mappers;

// Modelos existentes (mantenidos por compatibilidad)
pub mod codeudor;
pub mod cuota;
pub mod estados;
pub mod moras;
pub mod pagare;
pub mod prestamo;
pub mod prestamo_detalle;
pub mod roles;
pub mod tipo_cuota;
pub mod usuarios;
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use crate::models::estados::Estados;

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Pagare {
    pub id: i32,  // Campo requerido
    pub prestamo_id: i32,
    pub pagare: String,
    pub estado: Estados,  // Corregido de Estado a Estados
    pub comentarios_rechazo: Option<String>,
}
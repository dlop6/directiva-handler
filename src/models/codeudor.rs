use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Codeudor {
    pub nombre: String,
    pub correo: String,
    pub dpi: String,
    pub nit: String,
    pub direccion: String,
    pub telefono: String,
}
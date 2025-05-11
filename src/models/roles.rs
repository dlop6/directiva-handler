use juniper::{GraphQLObject, GraphQLInputObject};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Rol {
    pub nombre: String,
    pub tasa_interes: f64,
}
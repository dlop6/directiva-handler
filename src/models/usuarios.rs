use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct User {
    pub id: i32,
    pub nombre_completo: String,
    pub roles: Vec<Rol>,
    pub total_aporte: f64,  

}
#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Rol {
    pub nombre: String,
    pub tasa_interes: f64, 
}
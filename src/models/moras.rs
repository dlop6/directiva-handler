use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use crate::models::estados::Estados;

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Mora {
    pub nombre_usuario: String,
    pub moras_cuota: Vec<CuotaMora>,
    pub moras_prestamo: Vec<PrestamoCuotaMora>,
}

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct CuotaMora {
    pub mes_cuota: String,
    pub monto: f64,
    pub estado: Estados,  // Campo requerido
}

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct PrestamoCuotaMora {
    pub nombre_prestamo: String,
    pub mes_cuota: String,
    pub monto: f64,
    pub estado: Estados,  // Campo requerido
}
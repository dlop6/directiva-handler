use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Cuota {
    pub monto_cuota: f64,
    pub fecha_vencimiento: String,
    pub monto_pagado: f64,
    pub multa: f64,
}
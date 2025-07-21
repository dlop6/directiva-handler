use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Cuota {
    pub id: i32,
    pub usuario_id: i32,
    pub tipo_cuota_id: i32,
    pub monto_cuota: f64,
    pub fecha_vencimiento: String,
    pub monto_pagado: f64,
    pub multa: f64,
    pub fecha_creacion: String,
}

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct PagoCompleto {
    pub id: i32,
    pub usuario_id: i32,
    pub nombre_usuario: String,
    pub tipo_cuota: String,
    pub monto_cuota: f64,
    pub fecha_vencimiento: String,
    pub monto_pagado: f64,
    pub multa: f64,
    pub fecha_creacion: String,
    pub estado_pago: String,
}
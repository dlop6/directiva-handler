use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct PrestamoDetalle {
    pub prestamo_id: i32,
    pub numero_cuota: i32,
    pub monto_cuota: f64,
    pub fecha_vencimiento: NaiveDate,
    pub monto_pagado: f64,
    pub multa: f64,
}
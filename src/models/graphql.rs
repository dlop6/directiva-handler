use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

// Modelos para GraphQL con derivación de Juniper
#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Prestamo {
    pub id: i32,
    pub solicitante_id: i32,
    pub nombre: String,
    pub monto_total: f64,
    pub monto_cancelado: f64,
    pub motivo: String,
    pub tasa_interes: f64,
    pub fecha_solicitud: String,
    pub plazo_meses: i32,
    pub meses_cancelados: i32,
    pub estado: String,
}

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

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Usuario {
    pub id: i32,
    pub nombre: String,
    pub email: String,
    pub rol: String,
}

#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Mora {
    pub id: i32,
    pub usuario_id: i32,
    pub monto_mora: f64,
    pub fecha_mora: String,
    pub estado: String,
}

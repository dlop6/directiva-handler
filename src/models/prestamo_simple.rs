use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// Modelo de Préstamo simplificado para arquitectura Redis
#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct PrestamoSimple {
    pub id: String,
    pub monto: f64,
    pub plazo_meses: i32,
    pub tasa_interes: f64,
    pub estado: String,
    pub fecha_creacion: NaiveDateTime,
}

/// Modelo de Pago para arquitectura Redis
#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Pago {
    pub id: String,
    pub prestamo_id: String,
    pub monto: f64,
    pub fecha_pago: NaiveDateTime,
    pub metodo_pago: String,
}

// Re-export del modelo original para compatibilidad
pub use super::prestamo::Prestamo;

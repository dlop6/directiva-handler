use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

use crate::models::{codeudor::Codeudor, prestamo_detalle::PrestamoDetalle, pagare::Pagare};


#[derive(Clone, Serialize, Deserialize, GraphQLObject, Debug)]
pub struct Prestamo {
    pub solicitante_id: i32,
    pub nombre: String,
    pub monto_total: f64,
    pub monto_cancelado: f64,
    pub motivo: String,
    pub tasa_interes: f64,
    pub fecha_solicitud: String,  // Usando chrono para fechas
    pub plazo_meses: i32,
    pub meses_cancelados: i32,
    pub estado: super::estados::Estados,
    
    pub codeudores: Option<Vec<Codeudor>>,
    pub mensualidad_prestamo: Option<Vec<PrestamoDetalle>>,
    pub pagare: Option<Vec<Pagare>>,
}
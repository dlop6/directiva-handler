use juniper::FieldResult;
use crate::models::{prestamo::Prestamo, estados::Estados};

pub struct PrestamoQuery;

#[juniper::graphql_object]
impl PrestamoQuery {
    fn obtener_prestamos_vigentes(&self, _usuario_id: Option<i32>) -> FieldResult<Vec<Prestamo>> {
        // Lógica dummy temporal
        Ok(vec![Prestamo {
            solicitante_id: 1,
            nombre: "Préstamo Ejemplo".to_string(),
            monto_total: 10000.0,
            monto_cancelado: 2000.0,
            motivo: "Emergencia".to_string(),
            tasa_interes: 5.0,
            fecha_solicitud: "2023-01-01".to_string(), 
            plazo_meses: 12,
            meses_cancelados: 2,
            estado: Estados::Vigente,
            codeudores: None,
            mensualidad_prestamo: None,
            pagare: None,
        }])
    }
}
use juniper::FieldResult;
use crate::models::Cuota;
use crate::endpoints::handlers::configs::dummy_data;

pub struct PagoQuery;

#[juniper::graphql_object]
impl PagoQuery {
    fn obtener_pagos_pendientes(&self, usuario_id: i32) -> FieldResult<Vec<Cuota>> {
        Ok(dummy_data::get_dummy_cuotas()
            .into_iter()
            .filter(|c| c.monto_pagado == 0.0)
            .collect())
    }

    fn obtener_pagos_pagados(&self, usuario_id: i32) -> FieldResult<Vec<Cuota>> {
        Ok(dummy_data::get_dummy_cuotas()
            .into_iter()
            .filter(|c| c.monto_pagado > 0.0)
            .collect())
    }
}
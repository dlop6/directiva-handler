use juniper::FieldResult;
use crate::models::{Cuota, Estados};

pub struct PagoQuery;

#[juniper::graphql_object]
impl PagoQuery {
    
    fn obtener_pagos_pendientes(usuario_id: i32) -> FieldResult<Vec<Cuota>> {
        Ok(get_dummy_cuotas()
            .into_iter()
            .filter(|c| c.monto_pagado == 0.0) // Filtra cuotas no pagadas
            .collect()
        )
    }

    
    fn obtener_pagos_pagados(&self, usuario_id: i32) -> FieldResult<Vec<Cuota>> {
        // Filtrar cuotas con estado Pagada
        todo!()
    }
}
use juniper::FieldResult;
use crate::models::{
    prestamo::Prestamo,
    estados::Estados,
    codeudor::Codeudor,
    prestamo_detalle::PrestamoDetalle,
    pagare::Pagare
};
use crate::endpoints::handlers::configs::dummy_data;

pub struct PrestamoQuery;

#[juniper::graphql_object]
impl PrestamoQuery {
    fn obtener_todos_prestamos(&self) -> FieldResult<Vec<Prestamo>> {
        Ok(dummy_data::get_dummy_prestamos())
    }

    fn obtener_prestamos_vigentes(&self) -> FieldResult<Vec<Prestamo>> {
        Ok(dummy_data::get_dummy_prestamos()
            .into_iter()
            .filter(|p| p.estado == Estados::Vigente)
            .collect())
    }

    fn obtener_prestamos_completados(&self) -> FieldResult<Vec<Prestamo>> {
        Ok(dummy_data::get_dummy_prestamos()
            .into_iter()
            .filter(|p| p.estado == Estados::Completado)
            .collect())
    }

    fn obtener_prestamos_por_usuario(
        &self, 
        usuario_id: i32,
        estado: Option<Estados>
    ) -> FieldResult<Vec<Prestamo>> {
        let prestamos = dummy_data::get_dummy_prestamos()
            .into_iter()
            .filter(|p| p.solicitante_id == usuario_id)
            .filter(|p| estado.as_ref().map_or(true, |e| p.estado == *e))
            .collect();

        Ok(prestamos)
    }

    fn obtener_prestamo_por_id(&self, id: i32) -> FieldResult<Prestamo> {
        dummy_data::get_dummy_prestamos()
            .into_iter()
            .find(|p| p.id == id)  // Cambiado de solicitante_id a id
            .ok_or("Préstamo no encontrado".into())
    }

    fn obtener_detalle_prestamo(&self, prestamo_id: i32) -> FieldResult<Vec<PrestamoDetalle>> {
        Ok(vec![
            PrestamoDetalle {
                prestamo_id,
                numero_cuota: 1,
                monto_cuota: 850.0,
                fecha_vencimiento: "2023-04-01".to_string(),
                monto_pagado: 850.0,
                multa: 0.0,
            },
            PrestamoDetalle {
                prestamo_id,
                numero_cuota: 2,
                monto_cuota: 850.0,
                fecha_vencimiento: "2023-05-01".to_string(),
                monto_pagado: 0.0,
                multa: 0.0,
            }
        ])
    }

    fn obtener_codeudores(&self, _prestamo_id: i32) -> FieldResult<Vec<Codeudor>> {
    Ok(vec![
        Codeudor {
            nombre: "Juan Pérez".to_string(),
            correo: "juan@example.com".to_string(),
            dpi: "1234567890101".to_string(),
            nit: "1234567".to_string(),
            direccion: "Calle 123, Ciudad".to_string(),
            telefono: "5555-5555".to_string(),
        }
    ])
}

    fn obtener_pagares(&self, prestamo_id: i32) -> FieldResult<Vec<Pagare>> {
        Ok(vec![
            Pagare {
                id: 1,
                prestamo_id,
                pagare: "Pagaré de ejemplo".to_string(),
                estado: Estados::Vigente,
                comentarios_rechazo: None,
            }
        ])
    }
}
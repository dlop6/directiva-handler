use juniper::FieldResult;
use crate::models::{Loan, Estados};

pub struct PrestamoQuery;

#[juniper::graphql_object]
impl PrestamoQuery {
    fn obtener_prestamos_vigentes(&self, usuario_id: Option<i32>) -> FieldResult<Vec<Loan>> {
        // Filtrar por estado = Vigente
        todo!()
    }

    fn obtener_prestamos_completados(&self, usuario_id: Option<i32>) -> FieldResult<Vec<Loan>> {
        // Filtrar por estado = Completado
        todo!()
    }
}
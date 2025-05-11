use juniper::FieldResult;
use crate::models::moras::Mora;

pub struct MoraQuery;

#[juniper::graphql_object]
impl MoraQuery {
    
    fn obtener_moras_por_usuario(&self, usuario_id: i32) -> FieldResult<Mora> {
        todo!("Conectar con repositorio")
    }

    
    fn obtener_todas_moras(&self) -> FieldResult<Vec<Mora>> {
        todo!("Conectar con repositorio")
    }
}
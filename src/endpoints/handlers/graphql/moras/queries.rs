use juniper::FieldResult;
use crate::models::moras::Mora;
use crate::endpoints::handlers::configs::dummy_data;

pub struct MoraQuery;

#[juniper::graphql_object]
impl MoraQuery {
    fn obtener_moras_por_usuario(&self, usuario_id: i32) -> FieldResult<Mora> {
        // Buscar en los datos dummy por usuario_id
        let moras = dummy_data::get_dummy_moras();
        moras.into_iter()
            .find(|m| {
                let user = dummy_data::get_dummy_users()
                    .into_iter()
                    .find(|u| u.id == usuario_id)
                    .unwrap();
                m.nombre_usuario == user.nombre_completo
            })
            .ok_or("No se encontraron moras para este usuario".into())
    }

    fn obtener_todas_moras(&self) -> FieldResult<Vec<Mora>> {
        Ok(dummy_data::get_dummy_moras())
    }
}
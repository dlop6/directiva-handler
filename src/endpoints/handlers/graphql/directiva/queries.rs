use juniper::FieldResult;
use crate::{models::usuarios::User, endpoints::handlers::configs::dummy_data};

pub struct DirectivaQuery;

#[juniper::graphql_object]
impl DirectivaQuery {

    fn obtener_miembros_directiva(&self) -> FieldResult<Vec<User>> {
        Ok(
            dummy_data::get_dummy_users()
                .into_iter()
                .filter(|u| u.roles.iter().any(|r| r.nombre == "directiva"))
                .collect()
        )
    }

    
    fn obtener_roles_directiva(&self) -> FieldResult<Vec<String>> {
        Ok(vec!["directiva".to_string()])
    }
}
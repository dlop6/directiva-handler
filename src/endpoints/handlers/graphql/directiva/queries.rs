use juniper::FieldResult;
use crate::repos::pg::fetch_usuarios;
use crate::models::usuarios::User;
use crate::endpoints::graphql_context::Context;

pub struct DirectivaQuery;

#[juniper::graphql_object(Context = Context)]
impl DirectivaQuery {
    /// Ahora recibimos `ctx: &Context` junto al `&self`
    async fn obtener_miembros_directiva(
        &self,
        ctx: &Context,
    ) -> FieldResult<Vec<User>> {
        // 1) Sacamos un cliente del pool
        let client = ctx.pg_client.get().await?;
        // 2) Llamamos a tu función de repositorio
        let usuarios = fetch_usuarios(&client).await?;
        // 3) (Opcional) Filtramos solo “directiva” si hacía falta
        Ok(usuarios
            .into_iter()
            .filter(|u| u.roles.iter().any(|r| r.nombre == "directiva"))
            .collect())
    }
}

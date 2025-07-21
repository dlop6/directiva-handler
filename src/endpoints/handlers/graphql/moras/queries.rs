use juniper::FieldResult;
use crate::models::moras::Mora;
use crate::endpoints::graphql_context::Context;
use crate::repos::pg::fetch_moras;

pub struct MoraQuery;

#[juniper::graphql_object(Context = Context)]
impl MoraQuery {
    // obtener moras por nombre de usuario (ya que Mora usa nombre_usuario, no usuario_id)
    async fn obtener_moras_por_usuario(&self, ctx: &Context, nombre_usuario: String) -> FieldResult<Mora> {
        let client = ctx.pg_client.get().await?;
        let moras = fetch_moras(&client).await?;
        moras.into_iter()
            .find(|m| m.nombre_usuario == nombre_usuario)
            .ok_or("No se encontraron moras para este usuario".into())
    }

    async fn obtener_todas_moras(&self, ctx: &Context) -> FieldResult<Vec<Mora>> {
        let client = ctx.pg_client.get().await?;
        Ok(fetch_moras(&client).await?)
    }
}
use juniper::FieldResult;
use crate::models::moras::Mora;
use crate::endpoints::graphql_context::Context;
use crate::repos::pg::fetch_moras;

pub struct MoraQuery;

#[juniper::graphql_object(Context = Context)]
impl MoraQuery {
    async fn obtener_moras_por_usuario(&self, ctx: &Context, usuario_id: i32) -> FieldResult<Mora> {
        let client = ctx.pg_client.get().await?;
        let moras = fetch_moras(&client).await?;
        moras.into_iter()
            .find(|m| m.usuario_id == usuario_id)
            .ok_or("No se encontraron moras para este usuario".into())
    }

    async fn obtener_todas_moras(&self, ctx: &Context) -> FieldResult<Vec<Mora>> {
        let client = ctx.pg_client.get().await?;
        Ok(fetch_moras(&client).await?)
    }
}
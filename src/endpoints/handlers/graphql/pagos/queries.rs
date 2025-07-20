use juniper::FieldResult;
use crate::models::cuota::Cuota;
use crate::endpoints::graphql_context::Context;
use crate::repos::pg::fetch_cuotas;

pub struct PagoQuery;

#[juniper::graphql_object(Context = Context)]
impl PagoQuery {
    async fn obtener_pagos_pendientes(&self, ctx: &Context, usuario_id: i32) -> FieldResult<Vec<Cuota>> {
        let client = ctx.pg_client.get().await?;
        let cuotas = fetch_cuotas(&client).await?;
        Ok(cuotas
            .into_iter()
            .filter(|c| c.usuario_id == usuario_id && c.monto_pagado == 0.0)
            .collect())
    }

    async fn obtener_pagos_pagados(&self, ctx: &Context, usuario_id: i32) -> FieldResult<Vec<Cuota>> {
        let client = ctx.pg_client.get().await?;
        let cuotas = fetch_cuotas(&client).await?;
        Ok(cuotas
            .into_iter()
            .filter(|c| c.usuario_id == usuario_id && c.monto_pagado > 0.0)
            .collect())
    }
}
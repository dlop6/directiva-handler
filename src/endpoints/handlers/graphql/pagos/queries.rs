use juniper::FieldResult;
use crate::models::cuota::{Cuota, PagoCompleto};
use crate::endpoints::graphql_context::Context;
use crate::repos::pg::{fetch_cuotas, fetch_todos_los_pagos};

pub struct PagoQuery;

#[juniper::graphql_object(Context = Context)]
impl PagoQuery {
    /// obtiene todos los pagos de todos los socios - para supervisión de la directiva
    async fn obtener_todos_los_pagos(&self, ctx: &Context) -> FieldResult<Vec<PagoCompleto>> {
        let client = ctx.pg_client.get().await?;
        let pagos = fetch_todos_los_pagos(&client).await?;
        Ok(pagos)
    }

    /// todos los pagos pendientes de todos los socios
    async fn obtener_todos_los_pagos_pendientes(&self, ctx: &Context) -> FieldResult<Vec<PagoCompleto>> {
        let client = ctx.pg_client.get().await?;
        let pagos = fetch_todos_los_pagos(&client).await?;
        Ok(pagos
            .into_iter()
            .filter(|p| p.estado_pago == "Pendiente" || p.estado_pago == "Vencido")
            .collect())
    }

    /// pagos completados de todos los socios
    async fn obtener_todos_los_pagos_completados(&self, ctx: &Context) -> FieldResult<Vec<PagoCompleto>> {
        let client = ctx.pg_client.get().await?;
        let pagos = fetch_todos_los_pagos(&client).await?;
        Ok(pagos
            .into_iter()
            .filter(|p| p.estado_pago == "Pagado")
            .collect())
    }

    /// pagos por socio específico - para revisión individual
    async fn obtener_pagos_por_socio(&self, ctx: &Context, usuario_id: i32) -> FieldResult<Vec<PagoCompleto>> {
        let client = ctx.pg_client.get().await?;
        let pagos = fetch_todos_los_pagos(&client).await?;
        Ok(pagos
            .into_iter()
            .filter(|p| p.usuario_id == usuario_id)
            .collect())
    }

    // mantener las funciones originales para compatibilidad
    async fn obtener_pagos_pendientes(&self, ctx: &Context, usuario_id: i32) -> FieldResult<Vec<Cuota>> {
        let client = ctx.pg_client.get().await?;
        let cuotas = fetch_cuotas(&client).await?;
        Ok(cuotas
            .into_iter()
            .filter(|c| c.usuario_id == usuario_id && c.monto_pagado < c.monto_cuota)
            .collect())
    }

    async fn obtener_pagos_pagados(&self, ctx: &Context, usuario_id: i32) -> FieldResult<Vec<Cuota>> {
        let client = ctx.pg_client.get().await?;
        let cuotas = fetch_cuotas(&client).await?;
        Ok(cuotas
            .into_iter()
            .filter(|c| c.usuario_id == usuario_id && c.monto_pagado >= c.monto_cuota)
            .collect())
    }
}
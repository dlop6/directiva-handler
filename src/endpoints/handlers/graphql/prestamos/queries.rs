use juniper::FieldResult;
use crate::models::{
    prestamo::Prestamo,
    estados::Estados,
    codeudor::Codeudor,
    prestamo_detalle::PrestamoDetalle,
    pagare::Pagare
};
use crate::endpoints::graphql_context::Context;
use crate::repos::pg::{
    fetch_prestamos,
    fetch_prestamo_detalles,
    fetch_codeudores,
    fetch_pagares
};

pub struct PrestamoQuery;

#[juniper::graphql_object(Context = Context)]
impl PrestamoQuery {
    async fn obtener_todos_prestamos(&self, ctx: &Context) -> FieldResult<Vec<Prestamo>> {
        let client = ctx.pg_client.get().await?;
        Ok(fetch_prestamos(&client).await?)
    }

    async fn obtener_prestamos_vigentes(&self, ctx: &Context) -> FieldResult<Vec<Prestamo>> {
        let client = ctx.pg_client.get().await?;
        let prestamos = fetch_prestamos(&client).await?;
        Ok(prestamos.into_iter()
            .filter(|p| p.estado == Estados::Vigente)
            .collect())
    }

    async fn obtener_prestamos_completados(&self, ctx: &Context) -> FieldResult<Vec<Prestamo>> {
        let client = ctx.pg_client.get().await?;
        let prestamos = fetch_prestamos(&client).await?;
        Ok(prestamos.into_iter()
            .filter(|p| p.estado == Estados::Completado)
            .collect())
    }

    async fn obtener_prestamos_por_usuario(
        &self, 
        ctx: &Context,
        usuario_id: i32,
        estado: Option<Estados>
    ) -> FieldResult<Vec<Prestamo>> {
        let client = ctx.pg_client.get().await?;
        let prestamos = fetch_prestamos(&client).await?;
        Ok(prestamos.into_iter()
            .filter(|p| p.solicitante_id == usuario_id)
            .filter(|p| estado.as_ref().map_or(true, |e| p.estado == *e))
            .collect())
    }

    async fn obtener_prestamo_por_id(&self, ctx: &Context, id: i32) -> FieldResult<Prestamo> {
        let client = ctx.pg_client.get().await?;
        let prestamos = fetch_prestamos(&client).await?;
        prestamos.into_iter()
            .find(|p| p.id == id)
            .ok_or("Préstamo no encontrado".into())
    }

    async fn obtener_detalle_prestamo(&self, ctx: &Context, prestamo_id: i32) -> FieldResult<Vec<PrestamoDetalle>> {
        let client = ctx.pg_client.get().await?;
        Ok(fetch_prestamo_detalles(&client, prestamo_id).await?)
    }

    async fn obtener_codeudores(&self, ctx: &Context, prestamo_id: i32) -> FieldResult<Vec<Codeudor>> {
        let client = ctx.pg_client.get().await?;
        Ok(fetch_codeudores(&client, prestamo_id).await?)
    }

    async fn obtener_pagares(&self, ctx: &Context, prestamo_id: i32) -> FieldResult<Vec<Pagare>> {
        let client = ctx.pg_client.get().await?;
        Ok(fetch_pagares(&client, prestamo_id).await?)
    }
}
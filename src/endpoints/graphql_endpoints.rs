use actix_web::{web, Result, HttpResponse};
use juniper::http::{GraphQLRequest, playground::playground_source};
use crate::configs::schema::{DirectivaContext, create_schema};

// GraphQL Query Root siguiendo patrón general-handler exacto
#[derive(Debug, Default)]
pub struct DirectivaQueryRoot;

#[juniper::graphql_object(Context = DirectivaContext)]
impl DirectivaQueryRoot {
    /// Obtener todos los préstamos
    fn prestamos(context: &DirectivaContext) -> Vec<crate::repos::prestamo::Prestamo> {
        context.prestamo_repo().get_all_prestamos()
    }
    
    /// Obtener préstamo por ID
    fn prestamo(context: &DirectivaContext, id: String) -> Option<crate::repos::prestamo::Prestamo> {
        context.prestamo_repo().get_prestamo_by_id(&id)
    }
    
    /// Obtener todos los pagos
    fn pagos(context: &DirectivaContext) -> Vec<crate::repos::prestamo::Pago> {
        context.pago_repo().get_all_pagos()
    }
}

// Mutation Root (placeholder para extensiones futuras)
#[derive(Debug, Default)]
pub struct DirectivaMutationRoot;

#[juniper::graphql_object(Context = DirectivaContext)]
impl DirectivaMutationRoot {
    fn placeholder() -> bool {
        true
    }
}

/// Handler para queries GraphQL
pub async fn graphql_handler(
    schema: web::Data<juniper::RootNode<'static, DirectivaQueryRoot, DirectivaMutationRoot, juniper::DefaultScalarValue>>,
    data: web::Json<GraphQLRequest>,
    context: web::Data<DirectivaContext>,
) -> Result<HttpResponse> {
    let res = data.execute(&schema, &context).await;
    Ok(HttpResponse::Ok().json(res))
}

/// Handler para GraphQL Playground UI
pub async fn playground_handler() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(playground_source("/graphql", None)))
}

/// Configuración de rutas GraphQL siguiendo patrón general-handler
pub fn config(cfg: &mut web::ServiceConfig) {
    let schema = create_schema::<DirectivaQueryRoot, DirectivaMutationRoot>();
    
    cfg.app_data(web::Data::new(schema))
        .route("/graphql", web::post().to(graphql_handler))
        .route("/playground", web::get().to(playground_handler));
}

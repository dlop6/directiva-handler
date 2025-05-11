use juniper::GraphQLEnum;

#[derive(GraphQLEnum, Clone, Debug)]
pub enum Estados {
    Pendiente,
    Rechazado,
    Aprobado,
    Completado,
    Vigente,
}

#[derive(GraphQLEnum, Clone, Debug)]
pub enum TipoCuota {
    Ordinaria,
    Extraordinaria,
}
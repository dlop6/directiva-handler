use juniper::GraphQLEnum;
use serde::{Deserialize, Serialize};

#[derive(GraphQLEnum, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
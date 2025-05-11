
use juniper::GraphQLEnum;

#[derive(GraphQLEnum, Clone, Debug)]
pub enum TipoCuota {
    Ordinaria,
    Extraordinaria,
}
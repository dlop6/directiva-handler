use sqlx::PgPool;
use crate::models::{Loan, Estados};

pub async fn obtener_prestamos_vigentes(pool: &PgPool, usuario_id: Option<i32>) -> Result<Vec<Loan>, sqlx::Error> {
    let query = match usuario_id {
        Some(id) => sqlx::query_as!(
            Loan,
            r#"SELECT * FROM prestamos WHERE estado = $1 AND solicitante_id = $2"#,
            Estados::Vigente.to_string(),
            id
        ),
        None => sqlx::query_as!(
            Loan,
            r#"SELECT * FROM prestamos WHERE estado = $1"#,
            Estados::Vigente.to_string()
        ),
    };
    query.fetch_all(pool).await
}
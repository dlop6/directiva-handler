use sqlx::PgPool;
use crate::models::moras::Mora;

pub async fn obtener_moras_por_usuario(pool: &PgPool, usuario_id: i32) -> Result<Mora, sqlx::Error> {
    sqlx::query_as!(
        Mora,
        r#"SELECT nombre_usuario, moras_cuota, moras_prestamo FROM moras WHERE usuario_id = $1"#,
        usuario_id
    )
    .fetch_one(pool)
    .await
}
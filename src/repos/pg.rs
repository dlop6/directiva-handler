// src/repos/pg.rs

use crate::models::{
    usuarios::User,
    roles::Rol,
    prestamo::Prestamo,
    cuota::Cuota,
    moras::{Mora, CuotaMora, PrestamoCuotaMora},
    estados::Estados,
    codeudor::Codeudor,
    prestamo_detalle::PrestamoDetalle,
    pagare::Pagare
};
use deadpool_postgres::Client;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Error;
use std::str::FromStr;

/// Mapea la fila de Postgres a struct intermedio.
#[derive(PostgresMapper)]
#[pg_mapper(table = "usuarios")]
struct UsuarioRow {
    id_usuario: i32,
    nombre_completo: String,
    // asume que `total_aporte` está en la tabla; si no, quítalo o ajusta:
    total_aporte: f64,
}

/// Obtiene todos los usuarios desde la base de datos.
pub async fn fetch_usuarios(client: &Client) -> Result<Vec<User>, Error> {
    // prepara la consulta
    let stmt = client
        .prepare(
            "SELECT u.id_usuario, u.nombre_completo, u.total_aporte, 
                    array_agg(r.nombre) as roles
             FROM usuarios u
             LEFT JOIN usuarios_roles ur ON u.id_usuario = ur.usuario_id
             LEFT JOIN roles r ON ur.rol_id = r.id_rol
             GROUP BY u.id_usuario, u.nombre_completo, u.total_aporte"
        )
        .await?;
    // ejecuta
    let rows = client.query(&stmt, &[]).await?;
    // mapea a tu modelo de dominio
    let users = rows
        .iter()
        .map(|row| {
            let roles: Vec<String> = row.get("roles");
            User {
                id: row.get("id_usuario"),
                nombre_completo: row.get("nombre_completo"),
                total_aporte: row.get("total_aporte"),
                roles: roles
                    .into_iter()
                    .map(|nombre| Rol { 
                        nombre, 
                        tasa_interes: 0.0  // por ahora hardcodeado, luego se puede obtener de la bd
                    })
                    .collect(),
            }
        })
        .collect();
    Ok(users)
}

/// Implementación para convertir strings a Estados
impl FromStr for Estados {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Vigente" => Ok(Estados::Vigente),
            "Completado" => Ok(Estados::Completado),
            "Pendiente" => Ok(Estados::Pendiente),
            "Rechazado" => Ok(Estados::Rechazado),
            _ => Err(format!("Estado inválido: {}", s)),
        }
    }
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "prestamos")]
struct PrestamoRow {
    id: i32,
    solicitante_id: i32,
    nombre: String,
    monto_total: f64,
    monto_cancelado: f64,
    motivo: String,
    tasa_interes: f64,
    fecha_solicitud: String,
    plazo_meses: i32,
    meses_cancelados: i32,
    estado: String,
}

pub async fn fetch_prestamos(client: &Client) -> Result<Vec<Prestamo>, Error> {
    let stmt = client
        .prepare(
            "SELECT p.*, 
                    array_agg(DISTINCT jsonb_build_object(
                        'nombre', c.nombre,
                        'correo', c.correo,
                        'dpi', c.dpi,
                        'nit', c.nit,
                        'direccion', c.direccion,
                        'telefono', c.telefono
                    )) as codeudores,
                    array_agg(DISTINCT jsonb_build_object(
                        'prestamo_id', pd.prestamo_id,
                        'numero_cuota', pd.numero_cuota,
                        'monto_cuota', pd.monto_cuota,
                        'fecha_vencimiento', pd.fecha_vencimiento,
                        'monto_pagado', pd.monto_pagado,
                        'multa', pd.multa
                    )) as mensualidades,
                    array_agg(DISTINCT jsonb_build_object(
                        'id', pg.id,
                        'prestamo_id', pg.prestamo_id,
                        'pagare', pg.pagare,
                        'estado', pg.estado,
                        'comentarios_rechazo', pg.comentarios_rechazo
                    )) as pagares
             FROM prestamos p
             LEFT JOIN codeudores c ON p.id = c.prestamo_id
             LEFT JOIN prestamos_detalle pd ON p.id = pd.prestamo_id
             LEFT JOIN pagares pg ON p.id = pg.prestamo_id
             GROUP BY p.id"
        )
        .await?;

    let rows = client.query(&stmt, &[]).await?;
    
    let prestamos = rows
        .iter()
        .map(|row| {
            let codeudores: Vec<Codeudor> = serde_json::from_value(row.get("codeudores")).unwrap_or_default();
            let mensualidades: Vec<PrestamoDetalle> = serde_json::from_value(row.get("mensualidades")).unwrap_or_default();
            let pagares: Vec<Pagare> = serde_json::from_value(row.get("pagares")).unwrap_or_default();
            
            Prestamo {
                id: row.get("id"),
                solicitante_id: row.get("solicitante_id"),
                nombre: row.get("nombre"),
                monto_total: row.get("monto_total"),
                monto_cancelado: row.get("monto_cancelado"),
                motivo: row.get("motivo"),
                tasa_interes: row.get("tasa_interes"),
                fecha_solicitud: row.get("fecha_solicitud"),
                plazo_meses: row.get("plazo_meses"),
                meses_cancelados: row.get("meses_cancelados"),
                estado: Estados::from_str(row.get::<&str, &str>("estado")).unwrap(),
                codeudores: Some(codeudores),
                mensualidad_prestamo: Some(mensualidades),
                pagare: Some(pagares),
            }
        })
        .collect();

    Ok(prestamos)
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "cuotas")]
struct CuotaRow {
    id: i32,
    usuario_id: i32,
    tipo_cuota_id: i32,
    monto_cuota: f64,
    fecha_vencimiento: String,
    monto_pagado: f64,
    multa: f64,
    fecha_creacion: String,
}

pub async fn fetch_cuotas(client: &Client) -> Result<Vec<Cuota>, Error> {
    let stmt = client
        .prepare(
            "SELECT c.id, c.usuario_id, c.tipo_cuota_id, c.monto_cuota, 
                    c.fecha_vencimiento::text, c.monto_pagado, c.multa,
                    c.fecha_creacion::text
             FROM cuotas c
             ORDER BY c.fecha_vencimiento DESC"
        )
        .await?;

    let rows = client.query(&stmt, &[]).await?;
    
    let cuotas = rows
        .iter()
        .map(|row| {
            Cuota {
                id: row.get("id"),
                usuario_id: row.get("usuario_id"),
                tipo_cuota_id: row.get("tipo_cuota_id"),
                monto_cuota: row.get("monto_cuota"),
                fecha_vencimiento: row.get("fecha_vencimiento"),
                monto_pagado: row.get("monto_pagado"),
                multa: row.get("multa"),
                fecha_creacion: row.get("fecha_creacion"),
            }
        })
        .collect();

    Ok(cuotas)
}

/// obtiene todos los pagos de todos los socios para la supervisión de la directiva
pub async fn fetch_todos_los_pagos(client: &Client) -> Result<Vec<crate::models::cuota::PagoCompleto>, Error> {
    let stmt = client
        .prepare(
            "SELECT c.id, c.usuario_id, u.nombre_completo as nombre_usuario,
                    tc.nombre as tipo_cuota, c.monto_cuota, 
                    c.fecha_vencimiento::text, c.monto_pagado, c.multa,
                    c.fecha_creacion::text,
                    CASE 
                        WHEN c.monto_pagado >= c.monto_cuota THEN 'Pagado'
                        WHEN c.fecha_vencimiento < CURRENT_DATE AND c.monto_pagado < c.monto_cuota THEN 'Vencido'
                        ELSE 'Pendiente'
                    END as estado_pago
             FROM cuotas c
             INNER JOIN usuarios u ON c.usuario_id = u.id_usuario
             INNER JOIN tipos_cuota tc ON c.tipo_cuota_id = tc.id
             ORDER BY c.fecha_vencimiento DESC, u.nombre_completo"
        )
        .await?;

    let rows = client.query(&stmt, &[]).await?;
    
    let pagos = rows
        .iter()
        .map(|row| {
            crate::models::cuota::PagoCompleto {
                id: row.get("id"),
                usuario_id: row.get("usuario_id"),
                nombre_usuario: row.get("nombre_usuario"),
                tipo_cuota: row.get("tipo_cuota"),
                monto_cuota: row.get("monto_cuota"),
                fecha_vencimiento: row.get("fecha_vencimiento"),
                monto_pagado: row.get("monto_pagado"),
                multa: row.get("multa"),
                fecha_creacion: row.get("fecha_creacion"),
                estado_pago: row.get("estado_pago"),
            }
        })
        .collect();

    Ok(pagos)
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "moras")]
struct MoraRow {
    usuario_id: i32,
    nombre_usuario: String,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "moras_cuota")]
struct CuotaMoraRow {
    mora_id: i32,
    mes_cuota: String,
    monto: f64,
    estado: String,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "moras_prestamo")]
struct PrestamoCuotaMoraRow {
    mora_id: i32,
    prestamo_id: i32,
    mes_cuota: String,
    monto: f64,
    estado: String,
}

pub async fn fetch_moras(client: &Client) -> Result<Vec<Mora>, Error> {
    let stmt = client
        .prepare(
            "SELECT m.*,
                    array_agg(DISTINCT jsonb_build_object(
                        'mes_cuota', mc.mes_cuota,
                        'monto', mc.monto,
                        'estado', mc.estado
                    )) as moras_cuota,
                    array_agg(DISTINCT jsonb_build_object(
                        'prestamo_id', mp.prestamo_id,
                        'mes_cuota', mp.mes_cuota,
                        'monto', mp.monto,
                        'estado', mp.estado
                    )) as moras_prestamo
             FROM moras m
             LEFT JOIN moras_cuota mc ON m.id = mc.mora_id
             LEFT JOIN moras_prestamo mp ON m.id = mp.mora_id
             GROUP BY m.id, m.usuario_id, m.nombre_usuario"
        )
        .await?;

    let rows = client.query(&stmt, &[]).await?;
    
    let moras = rows
        .iter()
        .map(|row| {
            let moras_cuota: Vec<CuotaMora> = serde_json::from_value(row.get("moras_cuota")).unwrap_or_default();
            let moras_prestamo: Vec<PrestamoCuotaMora> = serde_json::from_value(row.get("moras_prestamo")).unwrap_or_default();
            
            Mora {
                nombre_usuario: row.get("nombre_usuario"),
                moras_cuota,
                moras_prestamo,
            }
        })
        .collect();

    Ok(moras)
}

pub async fn fetch_prestamo_detalles(client: &Client, prestamo_id: i32) -> Result<Vec<PrestamoDetalle>, Error> {
    let stmt = client
        .prepare(
            "SELECT * FROM prestamos_detalle WHERE prestamo_id = $1 ORDER BY numero_cuota"
        )
        .await?;

    let rows = client.query(&stmt, &[&prestamo_id]).await?;
    
    let detalles = rows
        .iter()
        .map(|row| {
            PrestamoDetalle {
                prestamo_id: row.get("prestamo_id"),
                numero_cuota: row.get("numero_cuota"),
                monto_cuota: row.get("monto_cuota"),
                fecha_vencimiento: row.get("fecha_vencimiento"),
                monto_pagado: row.get("monto_pagado"),
                multa: row.get("multa"),
            }
        })
        .collect();

    Ok(detalles)
}

pub async fn fetch_codeudores(client: &Client, prestamo_id: i32) -> Result<Vec<Codeudor>, Error> {
    let stmt = client
        .prepare(
            "SELECT * FROM codeudores WHERE prestamo_id = $1"
        )
        .await?;

    let rows = client.query(&stmt, &[&prestamo_id]).await?;
    
    let codeudores = rows
        .iter()
        .map(|row| {
            Codeudor {
                nombre: row.get("nombre"),
                correo: row.get("correo"),
                dpi: row.get("dpi"),
                nit: row.get("nit"),
                direccion: row.get("direccion"),
                telefono: row.get("telefono"),
            }
        })
        .collect();

    Ok(codeudores)
}

pub async fn fetch_pagares(client: &Client, prestamo_id: i32) -> Result<Vec<Pagare>, Error> {
    let stmt = client
        .prepare(
            "SELECT * FROM pagares WHERE prestamo_id = $1"
        )
        .await?;

    let rows = client.query(&stmt, &[&prestamo_id]).await?;
    
    let pagares = rows
        .iter()
        .map(|row| {
            Pagare {
                id: row.get("id"),
                prestamo_id: row.get("prestamo_id"),
                pagare: row.get("pagare"),
                estado: Estados::from_str(row.get::<&str, &str>("estado")).unwrap(),
                comentarios_rechazo: row.get("comentarios_rechazo"),
            }
        })
        .collect();

    Ok(pagares)
}

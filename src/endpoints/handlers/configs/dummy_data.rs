use crate::models::{
    usuarios::User,
    roles::Rol,
    moras::{Mora, CuotaMora, PrestamoCuotaMora},
    cuota::Cuota,
    estados::Estados,
    prestamo::Prestamo
};

pub fn get_dummy_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            nombre_completo: "Ana Pérez".to_string(),
            roles: vec![Rol { nombre: "directiva".to_string(), tasa_interes: 12.5 }],
            total_aporte: 1500.0,
        },
        User {
            id: 2,
            nombre_completo: "Luis García".to_string(),
            roles: vec![Rol { nombre: "socio".to_string(), tasa_interes: 3.5 }],
            total_aporte: 800.0,
        },
        User {
            id: 3,
            nombre_completo: "Carlos Martínez".to_string(),
            roles: vec![Rol { nombre: "socio".to_string(), tasa_interes: 3.5 }],
            total_aporte: 1200.0,
        },
        User {
            id: 4,
            nombre_completo: "María Rodríguez".to_string(),
            roles: vec![
                Rol { nombre: "directiva".to_string(), tasa_interes: 12.5 },
                Rol { nombre: "tesorero".to_string(), tasa_interes: 10.0 }
            ],
            total_aporte: 2000.0,
        }
    ]
}

pub fn get_dummy_cuotas() -> Vec<Cuota> {
    vec![
        Cuota {
            id: 1,
            usuario_id: 1,
            tipo_cuota_id: 1,
            monto_cuota: 100.0,
            fecha_vencimiento: "2025-10-01".to_string(),
            monto_pagado: 0.0,
            multa: 0.0,
            fecha_creacion: "2025-07-01T00:00:00Z".to_string(),
        },
        Cuota {
            id: 2,
            usuario_id: 2,
            tipo_cuota_id: 1,
            monto_cuota: 100.0,
            fecha_vencimiento: "2025-11-01".to_string(),
            monto_pagado: 100.0,
            multa: 0.0,
            fecha_creacion: "2025-07-01T00:00:00Z".to_string(),
        },
        Cuota {
            id: 3,
            usuario_id: 3,
            tipo_cuota_id: 2,
            monto_cuota: 150.0,
            fecha_vencimiento: "2025-12-01".to_string(),
            monto_pagado: 75.0,
            multa: 5.0,
            fecha_creacion: "2025-07-01T00:00:00Z".to_string(),
        },
        Cuota {
            id: 4,
            usuario_id: 4,
            tipo_cuota_id: 1,
            monto_cuota: 200.0,
            fecha_vencimiento: "2026-01-01".to_string(),
            monto_pagado: 0.0,
            multa: 10.0,
            fecha_creacion: "2025-07-01T00:00:00Z".to_string(),
        }
    ]
}

pub fn get_dummy_moras() -> Vec<Mora> {
    vec![
        Mora {
            nombre_usuario: "Usuario Ejemplo".to_string(),
            moras_cuota: vec![
                CuotaMora {
                    mes_cuota: "2025-09".to_string(),
                    monto: 50.0,
                    estado: Estados::Pendiente,
                },
                CuotaMora {
                    mes_cuota: "2025-10".to_string(),
                    monto: 75.0,
                    estado: Estados::Pendiente,
                },
            ],
            moras_prestamo: vec![
                PrestamoCuotaMora {
                    nombre_prestamo: "Préstamo Vivienda".to_string(),
                    mes_cuota: "2025-08".to_string(),
                    monto: 100.0,
                    estado: Estados::Vigente,
                },
            ],
        },
        Mora {
            nombre_usuario: "Carlos Martínez".to_string(),
            moras_cuota: vec![
                CuotaMora {
                    mes_cuota: "2025-11".to_string(),
                    monto: 60.0,
                    estado: Estados::Pendiente,
                },
            ],
            moras_prestamo: vec![],
        }
    ]
}

pub fn get_dummy_prestamos() -> Vec<Prestamo> {
    vec![
        Prestamo {
            id: 1,
            solicitante_id: 1,
            nombre: "Préstamo Personal".to_string(),
            monto_total: 10000.0,
            monto_cancelado: 2000.0,
            motivo: "Emergencia médica".to_string(),
            tasa_interes: 5.0,
            fecha_solicitud: "2023-01-01".to_string(),
            plazo_meses: 12,
            meses_cancelados: 2,
            estado: Estados::Vigente,
            codeudores: None,
            mensualidad_prestamo: None,
            pagare: None,
        },
        Prestamo {
            id: 2,
            solicitante_id: 2,
            nombre: "Préstamo Vivienda".to_string(),
            monto_total: 15000.0,
            monto_cancelado: 5000.0,
            motivo: "Compra de casa".to_string(),
            tasa_interes: 7.5,
            fecha_solicitud: "2023-03-15".to_string(),
            plazo_meses: 24,
            meses_cancelados: 6,
            estado: Estados::Vigente,
            codeudores: None,
            mensualidad_prestamo: None,
            pagare: None,
        },
        Prestamo {
            id: 3,
            solicitante_id: 1,
            nombre: "Préstamo Completado".to_string(),
            monto_total: 5000.0,
            monto_cancelado: 5000.0,
            motivo: "Educación".to_string(),
            tasa_interes: 3.0,
            fecha_solicitud: "2022-06-01".to_string(),
            plazo_meses: 10,
            meses_cancelados: 10,
            estado: Estados::Completado,
            codeudores: None,
            mensualidad_prestamo: None,
            pagare: None,
        },

    ]
}
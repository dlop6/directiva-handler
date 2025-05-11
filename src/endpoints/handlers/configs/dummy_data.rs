use crate::models::usuarios::{User, Rol};
use crate::models::moras::{Mora, CuotaMora, PrestamoCuotaMora};

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
        }
    ]
}

pub fn get_dummy_moras() -> Vec<Mora> {
    vec![
        Mora {
            nombre_usuario: "Ana Pérez".to_string(),
            moras_cuota: vec![
                CuotaMora {
                    mes_cuota: "2025-10".to_string(),
                    monto: 25.0,
                },
            ],
            moras_prestamo: vec![
                PrestamoCuotaMora {
                    nombre_prestamo: "Préstamo Casa".to_string(),
                    mes_cuota: "2025-09".to_string(),
                    monto: 50.0,
                },
            ],
        },
    ]
}

pub fn get_dummy_cuotas() -> Vec<Cuota> {
    vec![
        Cuota {
            monto_couta: 100.0,
            fecha_vencimiento: "2025-10-01".to_string(),
            monto_pagado: 0.0,
            multa: 0.0,
        },
        Cuota {
            monto_couta: 100.0,
            fecha_vencimiento: "2025-11-01".to_string(),
            monto_pagado: 100.0, // Pagada
            multa: 0.0,
        },
    ]
}
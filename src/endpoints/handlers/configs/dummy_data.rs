use crate::models::{
    usuarios::User,
    roles::Rol,
    moras::{Mora, CuotaMora, PrestamoCuotaMora},
    cuota::Cuota,
    estados::Estados
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
        }
    ]
}

pub fn get_dummy_cuotas() -> Vec<Cuota> {
    vec![
        Cuota {
            monto_cuota: 100.0,
            fecha_vencimiento: "2025-10-01".to_string(),
            monto_pagado: 0.0,
            multa: 0.0,
        },
        Cuota {
            monto_cuota: 100.0,
            fecha_vencimiento: "2025-11-01".to_string(),
            monto_pagado: 100.0,
            multa: 0.0,
        },
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
                    estado: Estados::Pendiente,  // Campo agregado
                },
                CuotaMora {
                    mes_cuota: "2025-10".to_string(),
                    monto: 75.0,
                    estado: Estados::Pendiente,  // Campo agregado
                },
            ],
            moras_prestamo: vec![
                PrestamoCuotaMora {
                    nombre_prestamo: "Préstamo Vivienda".to_string(),
                    mes_cuota: "2025-08".to_string(),
                    monto: 100.0,
                    estado: Estados::Vigente,  // Campo agregado
                },
                PrestamoCuotaMora {
                    nombre_prestamo: "Préstamo Vehículo".to_string(),
                    mes_cuota: "2025-09".to_string(),
                    monto: 150.0,
                    estado: Estados::Vigente,  // Campo agregado
                },
            ],
        },
    ]
}
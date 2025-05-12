# Documentación API GraphQL - Cooperativa Backend

## Info General
**URL Base**: `http://localhost:8080/graphql`  
**Playground**: `http://localhost:8080/graphiql`  
**Tecnología**: Actix-Web + Juniper (Rust)

## 🔍 Queries Disponibles

### 1. Consultas de Directiva
```graphql
query {
  directiva {
    obtenerMiembrosDirectiva {
      id
      nombreCompleto
      roles {
        nombre
        tasaInteres
      }
      totalAporte
    }
    obtenerRolesDirectiva
  }
}
```

### 2. Consultas de Moras
```graphql
query {
  moras {
    obtenerTodasMoras {
      nombreUsuario
      morasCuota {
        mesCuota
        monto
        estado
      }
      morasPrestamo {
        nombrePrestamo
        mesCuota
        monto
        estado
      }
    }
    
    obtenerMorasPorUsuario(usuarioId: 1) {
      nombreUsuario
      morasCuota {
        mesCuota
        monto
      }
    }
  }
}
```

### 3. Consultas de Pagos
```graphql
query {
  pagos {
    obtenerPagosPendientes(usuarioId: 1) {
      montoCuota
      fechaVencimiento
      montoPagado
      multa
    }
    
    obtenerPagosPagados(usuarioId: 1) {
      montoCuota
      fechaVencimiento
      montoPagado
    }
  }
}
```

### 4. Consultas de Préstamos
```graphql
query {
  prestamos {
    obtenerPrestamosVigentes(usuarioId: 1) {
      nombre
      montoTotal
      montoCancelado
      estado
      tasaInteres
      plazoMeses
      fechaSolicitud
    }
  }
}
```

##  Datos Dummy Disponibles

### Usuarios de prueba
| ID | Nombre          | Roles               | Aporte |
|----|-----------------|---------------------|--------|
| 1  | Ana Pérez       | directiva           | 1500   |
| 2  | Luis García     | socio               | 800    |
| 3  | Carlos Martínez | socio               | 1200   |
| 4  | María Rodríguez | directiva, tesorero | 2000   |

### Moras disponibles
- Usuario "Ejemplo Usuario":
  - 2 moras de cuota (50 y 75)
  - 1 mora de préstamo (100)
- Usuario "Carlos Martínez":
  - 1 mora de cuota (60)

### Cuotas disponibles
- 2 pendientes (0 pagado)
- 1 pagada (100 pagado)
- 1 parcial (75/150 pagado)

## Cómo Probar

1. Inicie el servidor:
```bash
cargo run
```

2. Abra el playground GraphiQL:
```
http://localhost:8080/graphiql
```

3. Ejecute queries de prueba como:
```graphql
query PruebaCompleta {
  usuarios: directiva {
    obtenerMiembrosDirectiva {
      nombreCompleto
    }
  }
  moras {
    obtenerTodasMoras {
      nombreUsuario
    }
  }
}
```

## Estructura de Datos

### Usuario (`User`)
```graphql
type User {
  id: Int!
  nombreCompleto: String!
  roles: [Rol!]!
  totalAporte: Float!
}
```

### Mora (`Mora`)
```graphql
type Mora {
  nombreUsuario: String!
  morasCuota: [CuotaMora!]!
  morasPrestamo: [PrestamoCuotaMora!]!
}
```






##  Notas para Frontend
1. Todos los campos son sensibles a mayúsculas/minúsculas
2. Los enums disponibles son: `Pendiente`, `Vigente`, `Pagado`, `Rechazado`
3. IDs de usuarios dummy válidos: 1, 2, 3, 4


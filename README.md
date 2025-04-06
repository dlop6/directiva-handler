# Directiva Handler

Este servicio es el componente de la Directiva de la cooperativa. Se encarga de:

- Validar pagos
- Revisar préstamos
- Aprobar pagarés
- Generar archivos financieros

## Tecnologías

- Rust + Actix Web
- GraphQL (`async-graphql`)
- Docker + Podman
- Variables de entorno con `.env`

## Instalación

1. Clona el repositorio:
   ```bash
   git clone <url>
   cd directiva-handler
   ```
2. Crea un archivo .env con la configuración:

```
HOST=0.0.0.0
PORT=8080
DATABASE_URL=...

3. 
Corre con Docker/Podman:
```podman build -t directiva-handler .
podman run -p 8080:8080 --env-file .env directiva-handler```
#
Endpoint
POST /graphql — para enviar consultas GraphQL
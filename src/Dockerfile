# Etapa 1: compilación
FROM rust:1.74 as builder

WORKDIR /app



COPY .env /app/.env

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -r src

# Copia del resto del código fuente
COPY . .

# Compilación del binario en modo release
RUN cargo build --release


FROM debian:buster-slim

# Certificados necesarios para hacer requests HTTPS
RUN apt-get update && apt-get install -y ca-certificates && apt-get clean

# Copiamos el ejecutable desde la etapa de compilación
COPY --from=builder /app/target/release/directiva-handler /usr/local/bin/directiva-handler

# Comando por defecto al correr el contenedor
CMD ["directiva-handler"]

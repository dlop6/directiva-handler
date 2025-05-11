# --- Fase de construcción ---
FROM rust:1.77 as builder

# Instala dependencias del sistema (Build Tools equivalentes en Linux)
RUN apt-get update && \
    apt-get install -y \
        build-essential \
        cmake \
        pkg-config \
        libssl-dev \
        && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Cache de dependencias (igual que antes)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -r src

# Copia el código y compila
COPY . .
RUN cargo build --release

# --- Fase final ---
FROM debian:buster-slim

# Dependencias mínimas para ejecución
RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        libssl-dev \
        && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/directiva-handler /usr/local/bin/directiva-handler
COPY .env /app/.env  

CMD ["directiva-handler"]
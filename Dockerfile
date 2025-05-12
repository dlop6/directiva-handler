FROM rust:latest  
WORKDIR /app

# Instala dependencias del sistema
RUN apt-get update && \
    apt-get install -y \
        pkg-config \
        libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copia solo las dependencias primero
COPY Cargo.toml Cargo.lock ./

# Crea estructura dummy para cachear dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copia el código fuente real
COPY src ./src

# Build final
RUN cargo build --release

EXPOSE 8080
CMD ["./target/release/directiva-handler"]
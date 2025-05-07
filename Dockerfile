FROM rust:1.74 as builder

WORKDIR /app

COPY .env /app/.env

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -r src

COPY . .

RUN cargo build --release

FROM debian:buster-slim

# Certificados necesarios para hacer requests HTTPS
RUN apt-get update && apt-get install -y ca-certificates && apt-get clean

COPY --from=builder /app/target/release/directiva-handler /usr/local/bin/directiva-handler

CMD ["directiva-handler"]

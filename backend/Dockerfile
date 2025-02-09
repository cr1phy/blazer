FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src

COPY src ./src
COPY migration ./migration
RUN cargo build --release

COPY . .
RUN cargo build --release


FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/blazer-backend .
EXPOSE 8080
CMD ["./blazer-backend"]
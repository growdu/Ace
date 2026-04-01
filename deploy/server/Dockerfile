# Build stage
FROM rust:1.75 as builder

WORKDIR /app

COPY server/Cargo.toml server/src ./server/
COPY client/src-tauri/src/game ./client/src-tauri/src/game
COPY client/src-tauri/src/ai ./client/src-tauri/src/ai

RUN cargo build --release --manifest-path ./server/Cargo.toml

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/server/target/release/ace-server /app/server

ENV DATABASE_URL=postgres://postgres:password@localhost:5432/ace
ENV RUST_LOG=info

EXPOSE 8080

CMD ["/app/server"]

FROM rust:slim-buster as builder
WORKDIR /app-build
COPY . .
RUN cargo build --release

FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /app-build/target/release/backend /app-build/Rocket.toml /app-build/database/pass.pgpass /app/
EXPOSE 8000
CMD ["./backend"]
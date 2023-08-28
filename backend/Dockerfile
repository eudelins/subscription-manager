FROM rust:slim-buster as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /app/target/release/backend /app/Rocket.toml /app/database/pass.pgpass /app/
EXPOSE 8000
CMD ["./backend"]
FROM rust:bookworm as builder
COPY ./Cargo.lock ./Cargo.toml ./.infisical.json ./
COPY ./crates ./crates
COPY ./.sqlx ./.sqlx

RUN cargo build --release
RUN chmod +x ./target/release/bin

FROM debian:bookworm-slim
RUN apt-get update && apt install -y openssl
COPY --from=builder ./target/release/bin ./dattebayo/bin

CMD ["./dattebayo/bin"]

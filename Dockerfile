FROM rust:bookworm as builder
COPY ./Cargo.lock ./Cargo.toml ./.infisical.json ./
COPY ./crates ./crates
COPY ./.sqlx ./.sqlx
RUN apt-get update && apt-get install -y bash curl && curl -1sLf \
'https://dl.cloudsmith.io/public/infisical/infisical-cli/setup.deb.sh' | bash \
&& apt-get update && apt-get install -y infisical
RUN cargo build --release
RUN chmod +x ./target/release/bin

FROM debian:bookworm-slim
RUN apt-get update && apt install -y openssl
COPY --from=builder ./target/release/bin ./dattebayo/bin

CMD ["./dattebayo/bin"]

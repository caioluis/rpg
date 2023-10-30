FROM rust:latest as builder
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./crates ./crates
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder ./target/release/dattebayo ./target/release/dattebayo

CMD ["/target/release/dattebayo"]

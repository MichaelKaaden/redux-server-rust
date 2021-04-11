FROM rust:latest as builder
WORKDIR /usr/src/redux-server-rust
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/redux-server-rust/target/release/redux-server-rust /usr/local/bin/redux-server-rust
CMD ["redux-server-rust"]

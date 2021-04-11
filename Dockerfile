FROM rust:latest as builder
WORKDIR /usr/src/redux-server-rust
COPY . .
RUN cargo build --release

FROM debian:buster-slim
ENV APP_USER=appuser
RUN useradd -ms /bin/bash ${APP_USER}
EXPOSE 3000
COPY --from=builder /usr/src/redux-server-rust/target/release/redux-server-rust /usr/local/bin/redux-server-rust
RUN chown -R ${APP_USER} /usr/local/bin/redux-server-rust
USER ${APP_USER}
CMD ["redux-server-rust"]

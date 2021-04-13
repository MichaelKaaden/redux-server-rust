FROM rust:latest as builder

# download the target for static linking.
# RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y apt-utils musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# The idea is that as long as the base files have not changed,
# Docker will use its build cache to skip these step completely.
RUN cd /usr/src && cargo new redux-server-rust
WORKDIR /usr/src/redux-server-rust
COPY Cargo.lock Cargo.toml build.rs ./
RUN cargo build --release --target x86_64-unknown-linux-musl

# now copy the real source files
# RUN rm -rf src/*
COPY src ./src

# remove the build artifacts as Cargo might skip rebuilding otherwise
RUN rm -f ./target/release/deps/redux-server-rust*

# now build the application for release
RUN cargo build --release --target x86_64-unknown-linux-musl

#FROM debian:buster-slim
FROM alpine:latest
ENV APP_USER=appuser
#RUN useradd -u 10001 ${APP_USER}
RUN adduser -u 10001 -DH ${APP_USER}
EXPOSE 3000
COPY --from=builder /usr/src/redux-server-rust/target/x86_64-unknown-linux-musl/release/redux-server-rust /usr/local/bin/redux-server-rust
RUN chown ${APP_USER} /usr/local/bin/redux-server-rust
USER ${APP_USER}
CMD ["/usr/local/bin/redux-server-rust"]

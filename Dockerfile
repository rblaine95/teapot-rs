##### builder
FROM rust:1.62 AS builder

WORKDIR /opt/teapot

RUN rustup update stable
RUN cargo install cargo-audit
COPY . .
RUN cargo audit
RUN export ROCKET_CONFIG=/opt/teapot/rocket.toml; cargo test
RUN cargo build --release

##### runner
FROM gcr.io/distroless/cc

ENV ROCKET_CONFIG /opt/teapot/rocket.toml

WORKDIR /opt/teapot

COPY --chown=65532:65532 --from=builder /opt/teapot/target/release/teapot-rs /opt/teapot/bin/teapot
COPY --chown=65532:65532 ./rocket.toml ${ROCKET_CONFIG}

USER 65532
ENTRYPOINT [ "/opt/teapot/bin/teapot" ]

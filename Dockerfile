##### builder
FROM docker.io/rust:1.67 AS builder

WORKDIR /opt/teapot
COPY . .
RUN cargo build --release

##### runner
FROM gcr.io/distroless/cc

ENV ROCKET_CONFIG /opt/teapot/rocket.toml

WORKDIR /opt/teapot

COPY --chown=65532:65532 --from=builder /opt/teapot/target/release/teapot /opt/teapot/bin/teapot
COPY --chown=65532:65532 ./rocket.toml ${ROCKET_CONFIG}

USER 65532
ENTRYPOINT [ "/opt/teapot/bin/teapot" ]

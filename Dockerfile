FROM clux/muslrust:latest as builder

ADD . /volume

RUN cargo build --release

FROM scratch

COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/rust-microservice-sample /app

CMD [ "/app" ]
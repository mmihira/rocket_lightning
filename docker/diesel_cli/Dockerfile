FROM rust:1.26 as builder

RUN cargo install diesel_cli --no-default-features --features postgres
RUN mkdir -p /out && cp /usr/local/cargo/bin/diesel /out

FROM ubuntu:bionic
RUN apt-get update && apt-get -y install libpq5
COPY --from=builder /out/diesel /bin/
COPY ./migrations ./migrations

CMD diesel

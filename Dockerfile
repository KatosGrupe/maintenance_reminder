FROM rustlang/rust:nightly as builder
WORKDIR /root/

LABEL maintainer="Ignas Lapėnas <ignas@lapenas.dev>"
LABEL description="Test out docker as a repeatable build machine"

ADD Cargo.toml ./
ADD Cargo.lock ./
ADD src ./src

RUN cargo build --release

FROM ubuntu:latest
WORKDIR /www
COPY --from=builder /root/target/release/maintenance_reminder ./maintenance_reminder
CMD ["./maintenance_reminder"]

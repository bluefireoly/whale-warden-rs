FROM rust:slim as builder

WORKDIR /usr/src/whale-warden
COPY . .

RUN cargo install --path .


FROM alpine:latest

WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/whale-warden /usr/local/bin/whale-warden

CMD ["whale-warden"]

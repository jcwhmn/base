FROM rust:1.77.0-alpine3.18 as builder
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/actix-web-base
COPY . .
RUN cargo build --release

FROM alpine:3.18.0
COPY --from=builder /usr/src/actix-web-base/target/release/actix-web-base /
CMD ["./actix-web-base"]
FROM rust:1.76.0-alpine3.19 AS builder
RUN apk update && apk upgrade --no-cache
RUN apk add --no-cache musl-dev upx
WORKDIR /app
COPY ./src ./src
COPY ./templates ./templates
COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/htmx_collab_drawing /
COPY ./public /public

CMD ["./htmx_collab_drawing"]

FROM rust:alpine AS build-env
RUN apk add --no-cache protobuf-dev musl-dev gcc libc-dev
WORKDIR /app
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=build.rs,target=build.rs \
    --mount=type=bind,source=proto,target=proto \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp /app/target/release/web-capture /app/web-capture

FROM alpine:latest
RUN apk add --no-cache chromium font-noto-cjk
COPY --from=build-env /app/web-capture /
CMD ["/web-capture"]

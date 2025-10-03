
FROM rust:1.74-alpine AS builder
RUN apk add --no-cache musl-dev
ARG PROJECT=1
ENV PROJECT=${PROJECT}
WORKDIR /build
COPY . .
WORKDIR /build/${PROJECT}
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.19 AS runtime
ARG PROJECT=1
ENV PROJECT=${PROJECT}
WORKDIR /app
COPY --from=builder /build/${PROJECT}/target/x86_64-unknown-linux-musl/release/euler${PROJECT} /app/euler${PROJECT}
COPY --from=builder /build/${PROJECT}/README.md /app/README.md
CMD sh -c 'cat /app/README.md && echo && /app/euler${PROJECT}'

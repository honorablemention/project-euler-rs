FROM rust:1.85-alpine AS builder
RUN apk add --no-cache musl-dev
ARG PROJECT=1
ENV PROJECT=${PROJECT}
WORKDIR /build
COPY . .
WORKDIR /build/${PROJECT}
RUN cargo build --release && \
    mkdir -p /out && \
    cp target/release/euler${PROJECT} /out/euler${PROJECT}

FROM alpine:3.19 AS runtime
ARG PROJECT=1
ENV PROJECT=${PROJECT}
WORKDIR /app
COPY --from=builder /out/euler${PROJECT} /app/euler${PROJECT}
COPY --from=builder /build/${PROJECT}/README.md /app/README.md
CMD sh -c 'cat /app/README.md && echo && /app/euler${PROJECT}'

# stage 1 - Build application
FROM --platform=$BUILDPLATFORM rust:1.78.0-alpine3.20 as builder

WORKDIR /app
RUN apk add gcc g++ make musl-dev libpq-dev openssl-dev git upx curl
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migrations ./migrations
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN export ARCH=$(uname -m) \
  && cargo build --release
## Compress the binary
RUN cp /app/target/release/api /bin/api

# stage 2 - Create runtime image
FROM alpine:3.20
# Install runtime dependencies
RUN apk add --no-cache libgcc ca-certificates openssl libpq
## Copy the binary
COPY --from=builder /bin/api /bin/api

LABEL org.opencontainers.image.description Next Hat API

## Set entrypoint
ENTRYPOINT ["/bin/api"]

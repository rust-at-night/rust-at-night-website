# Build Stage
FROM rust:latest as builder
WORKDIR /usr/src/app

# Caching dependencies
RUN USER=root cargo new --bin website-backend
WORKDIR /usr/src/app/website-backend
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Copy source and build the application
COPY src ./src
RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim

# Default values for litestream
ENV LS_DB_PATH=/data/db
ENV LS_REPLICA_URL=s3://website-backend-db

# Define the Litestream version
ARG LITESTREAM_VERSION=0.3.9

# Install system dependencies and Litestream
RUN apt-get update && \
    apt-get install -y sqlite3 libsqlite3-dev libssl-dev pkg-config wget tar && \
    export CONTAINER_ARCH=$(uname -m) && if [ "$CONTAINER_ARCH" = "aarch64" ]; \
    then export CONTAINER_ARCH=arm64; fi && if [ "$CONTAINER_ARCH" = "x86_64" ]; \
    then export CONTAINER_ARCH=amd64; fi \
    wget -q -O litestream.tar.gz "https://github.com/benbjohnson/litestream/releases/download/v${LITESTREAM_VERSION}/litestream-v${LITESTREAM_VERSION}-linux-${CONTAINER_ARCH}.tar.gz" && \
    tar -xzf litestream.tar.gz -C /usr/local/bin litestream-v${LITESTREAM_VERSION}-linux-${CONTAINER_ARCH}/litestream && \
    rm litestream.tar.gz && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/website-backend/target/release/website_backend /usr/local/bin
COPY ./data/ /data/
COPY ./etc/ /etc/
COPY ./scripts/run.sh /scripts/run.sh
CMD [ "/scripts/run.sh" ]

EXPOSE 8081
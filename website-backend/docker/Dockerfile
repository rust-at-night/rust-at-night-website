# Build Stage
FROM rust:latest as builder

# Caching dependencies
WORKDIR /usr/src
COPY Cargo.toml Cargo.lock ./
RUN USER=root cargo new --lib website-backend && cargo new --lib website-frontend

# This is a dummy build to get the dependencies cached
RUN echo "" > website-backend/src/lib.rs && echo "" > website-frontend/src/lib.rs
RUN cargo build --release

# Now remove the dummy src directories
RUN rm -rf website-backend/src

# Copy the source and build the application
COPY website-backend ./website-backend
RUN cargo build --release 

# Runtime Stage
FROM debian:bookworm-slim

# Default values for litestream
ENV LS_DB_PATH=/data/db
ENV LS_REPLICA_URL=s3://website-backend-db

# Default value for container databse url
ENV DATABASE_URL=sqlite:///data/main.db

# Define the Litestream version
ARG LITESTREAM_VERSION=0.3.9

COPY ./website-backend/data/ /data/
COPY ./website-backend/etc/ /etc/
COPY ./website-backend/scripts/run.sh /scripts/run.sh

# Install system dependencies and Litestream
RUN apt-get update && \
    apt-get install -y sqlite3 libsqlite3-dev libssl-dev pkg-config tar curl && \
    export CONTAINER_ARCH=$(uname -m) && \
    if [ "$CONTAINER_ARCH" = "aarch64" ]; then \
    export CONTAINER_ARCH=arm64; \
    fi && \
    if [ "$CONTAINER_ARCH" = "x86_64" ]; then \
    export CONTAINER_ARCH=amd64; \
    fi && \
    curl -L "https://github.com/benbjohnson/litestream/releases/download/v0.3.9/litestream-v0.3.9-linux-${CONTAINER_ARCH}.tar.gz" -o litestream.tar.gz && \
    tar -xzf litestream.tar.gz -C /usr/local/bin litestream && \
    rm litestream.tar.gz && \
    rm -rf /var/lib/apt/lists/* && echo "done"

COPY --from=builder /usr/src/target/release/website-backend /usr/local/bin
CMD ["scripts/run.sh"]

EXPOSE 80
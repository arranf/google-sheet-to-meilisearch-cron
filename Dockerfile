FROM --platform=linux/amd64 rust:1.77.2-slim-bookworm AS builder
WORKDIR /usr/src/

RUN USER=root cargo new sheet-to-meilisearch
WORKDIR /usr/src/sheet-to-meilisearch
RUN USER=root apt-get update && apt-get -y install pkg-config libssl-dev
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Bundle Stage
FROM --platform=linux/amd64 debian:bookworm-slim AS runtime
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/sheet-to-meilisearch/target/release/sheet-to-meilisearch /usr/local/bin/sheet-to-meilisearch
CMD ["sheet-to-meilisearch"]
# ── Build stage ──────────────────────────────────────────────────────────────
FROM rust:1.91-slim AS builder

# Install musl toolchain and dependencies for a fully-static build
RUN apt-get update && apt-get install -y \
    musl-tools \
    perl \
    make \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build

# Copy workspace manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY apps/ apps/

# Build static binary (vendored OpenSSL avoids host pkg-config dependency)
ENV OPENSSL_VENDORED=1
RUN cargo build --release --target x86_64-unknown-linux-musl -p rh-cli

# ── Runtime stage ─────────────────────────────────────────────────────────────
# distroless/static includes CA certificates (needed for HTTPS) and /etc/passwd
FROM gcr.io/distroless/static-debian12:nonroot

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/rh /rh

ENTRYPOINT ["/rh"]

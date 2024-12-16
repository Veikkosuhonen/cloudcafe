FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin cloudcafe


FROM chef as builder-sqlx
WORKDIR /build
RUN cargo install sqlx-cli@^0.8.1 --locked --no-default-features --features rustls,postgres


# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/cloudcafe /usr/local/bin
COPY --from=builder-sqlx /usr/local/cargo/bin/sqlx /usr/local/bin

COPY configuration configuration
COPY migrations migrations

ENV APP_ENV=production
ENV RUST_LOG=info

CMD ["/usr/local/bin/cloudcafe"]

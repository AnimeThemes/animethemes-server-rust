FROM lukemathwalker/cargo-chef:latest-rust-1.95.0-trixie AS chef

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        pkg-config \
        libssl-dev \
        git \
    && rm -rf /var/lib/apt/lists/*

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .

RUN cargo build --release --locked --bin animethemes-server-rust

# We do not need the Rust toolchain to run the binary!
FROM debian:trixie-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3t64 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder \
    /app/target/release/animethemes-server-rust \
    /usr/local/bin/animethemes-server-rust

COPY --from=builder \
    /app/config \
    /app/config

ENTRYPOINT ["animethemes-server-rust"]

CMD ["start"]
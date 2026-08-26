FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        pkg-config \
        libssl-dev \
        git \
        curl \
    && rm -rf /var/lib/apt/lists/*


# --------------------------------------------------
# DEVELOPMENT
# --------------------------------------------------

FROM chef AS development

WORKDIR /app

CMD ["sleep", "infinity"]


# --------------------------------------------------
# PLANNER
# --------------------------------------------------

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


# --------------------------------------------------
# BUILDER
# --------------------------------------------------

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook \
    --release \
    --recipe-path recipe.json

COPY . .

RUN cargo build \
    --release \
    --bin animethemes-server-rust


# --------------------------------------------------
# PRODUCTION
# --------------------------------------------------

FROM debian:trixie-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder \
    /app/target/release/animethemes-server-rust \
    /usr/local/bin/animethemes-server-rust

COPY --from=builder \
    /app/config \
    /app/config

CMD ["animethemes-server-rust", "start"]
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --bins \
    && mkdir -p /app/bin \
    && find /app/target/release \
        -maxdepth 1 \
        -type f \
        -executable \
        -exec cp {} /app/bin/ \;

FROM debian:trixie-slim AS runtime
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/bin/ /usr/local/bin/

CMD ["animethemes-server-rust"]
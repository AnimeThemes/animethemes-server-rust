FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release --bins && \
    mkdir -p /out && \
    find /app/target/release \
        -maxdepth 1 \
        -type f \
        -executable \
        -exec cp {} /out/ \;


FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /out/ /usr/local/bin/

CMD ["/usr/local/bin/animethemes-server-rust"]
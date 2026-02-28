FROM rust:1.90.0-slim-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    libgpgme-dev \
    libgpg-error-dev \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN useradd -m -s /bin/bash envio

RUN apt-get update && apt-get install -y --no-install-recommends \
    bash \
    libgpgme-dev \
    libgpg-error-dev \
    libssl-dev \
    ca-certificates \
    sudo \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/envio /usr/local/bin/envio

RUN mkdir -p /app && chown -R envio:envio /app

USER envio
WORKDIR /app
ENV SHELL=/bin/bash
ENV HOME=/home/envio

CMD ["envio", "version"]

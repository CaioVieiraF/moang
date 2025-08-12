FROM rust:1.89.0-slim-bookworm AS builder

WORKDIR /server/

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir src/ && echo "fn main() {}" > src/main.rs
RUN cargo build --release

RUN rm -rf src/ && rm ./target/release/blog || true

COPY . .

RUN cargo build --release --bin blog

FROM debian:bookworm-slim AS release

RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /server/target/release/blog .

EXPOSE 5000

CMD ["./blog"]

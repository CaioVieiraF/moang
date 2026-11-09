FROM rust:1-bookworm AS builder

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

RUN rm -rf src/ && rm -f ./target/release/blog || true

COPY . .

RUN find src -name '*.rs' -exec touch {} + && cargo build --release --bin blog

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /server/target/release/blog ./blog

RUN mkdir -p content

EXPOSE 5000

CMD ["./blog"]

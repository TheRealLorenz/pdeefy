FROM lukemathwalker/cargo-chef:latest-rust-alpine3.18 AS chef
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
RUN cargo build --release

FROM zenika/alpine-chrome:117 AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/pdeefy /usr/local/bin

ENV RUST_LOG=info
ENTRYPOINT ["pdeefy"]

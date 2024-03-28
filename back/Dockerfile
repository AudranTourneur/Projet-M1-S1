FROM rust:1 AS chef 
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin app

# We do not need the Rust toolchain to run the binary!
FROM ubuntu:22.04 AS runtime
RUN apt-get update -y
RUN apt-get install cargo libssl-dev pkg-config gcc g++ libpq-dev libsqlite3-0 libsqlite3-dev -y 
WORKDIR app
COPY --from=builder /app/target/release/app /app/exe
COPY --from=builder /app/Rocket.toml /app/Rocket.toml
ENTRYPOINT ["/app/exe"]
FROM lukemathwalker/cargo-chef:latest AS chef
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
RUN cargo build --release --bin mock-s3-rs

FROM gcr.io/distroless/cc-debian13 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/mock-s3-rs /usr/local/bin/mock-s3-rs
CMD ["/usr/local/bin/mock-s3-rs"]

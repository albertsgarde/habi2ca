# Use an image with a specific version of Rust.
FROM lukemathwalker/cargo-chef:0.1.67-rust-1.79-slim-buster AS planner
# This container only exists to run 'cargo chef prepare' which sets up 'recipe.json' for the next stage.

WORKDIR /habi2ca
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM lukemathwalker/cargo-chef:0.1.67-rust-1.79-slim-buster AS backend-build
# This container builds the backend.

WORKDIR /habi2ca

# Build dependencies
COPY  --from=planner /habi2ca/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock .
COPY habi2ca-server habi2ca-server
COPY habi2ca-database habi2ca-database

# Build backend binary
RUN cargo build --release --bin habi2ca-server

# Our final base
FROM debian:buster-slim AS backend-prod
ENV DATABASE_PATH=/habi2ca/habi2ca.db
ENV PORT=8080

WORKDIR /habi2ca

# Copy the binary from the backend-build stage
COPY --from=backend-build /habi2ca/target/release/habi2ca-server ./habi2ca-server

# Set the startup command to run your binary
CMD ["sh", "-c", "./habi2ca-server ${DATABASE_PATH} 0.0.0.0 ${PORT}"]
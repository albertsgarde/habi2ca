# Use an image with a specific version of Rust.
FROM lukemathwalker/cargo-chef:0.1.68-rust-1.83.0-alpine3.21 AS planner
# This container only exists to run 'cargo chef prepare' which sets up 'recipe.json' for the next stage.

WORKDIR /habi2ca
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM lukemathwalker/cargo-chef:0.1.68-rust-1.83.0-alpine3.21 AS backend-build
# This container builds the backend.

WORKDIR /habi2ca

# Build dependencies
COPY  --from=planner /habi2ca/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock .
COPY habi2ca-server habi2ca-server
COPY habi2ca-database habi2ca-database
COPY gamedata gamedata

# Build backend binary
RUN cargo build --release --bin habi2ca-server

# Our final base
FROM alpine:3.21.0 AS backend-prod
ENV DATABASE_PATH=/habi2ca/db/data.db
ENV PORT=8080
ENV LOG_DIR=/habi2ca/log

WORKDIR /habi2ca

# Copy the binary from the backend-build stage
COPY --from=backend-build /habi2ca/target/release/habi2ca-server ./habi2ca-server

# Set the startup command to run your binary
CMD ["sh", "-c", "./habi2ca-server ${DATABASE_PATH} 0.0.0.0 ${PORT} --force-migrations --log-dir ${LOG_DIR}"]

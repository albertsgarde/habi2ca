FROM lukemathwalker/cargo-chef:0.1.67-rust-1.79-slim-buster AS chef-with-build-deps
RUN apt-get update && apt-get install -y libssl-dev pkg-config binaryen
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked wasm-bindgen-cli
RUN cargo install --locked trunk

# Use an image with a specific version of Rust.
FROM lukemathwalker/cargo-chef:0.1.67-rust-1.79-slim-buster AS planner
# This container only exists to run 'cargo chef prepare' which sets up 'recipe.json' for the next stage.

WORKDIR /habi2ca
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef-with-build-deps AS backend-build
# This container builds the backend.

WORKDIR /habi2ca

# Build dependencies
COPY  --from=planner /habi2ca/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
# Build WASM frontend
WORKDIR habi2ca-frontend
RUN trunk build --release
# Build backend binary
WORKDIR ..
RUN cargo build --release --bin habi2ca-server


# Our final base
FROM debian:buster-slim AS backend-prod
ENV DATABASE_PATH=/habi2ca/habi2ca.db
ENV PORT=8080

WORKDIR /habi2ca

# Copy the frontend ´dist´ directory.
COPY --from=backend-build /habi2ca/habi2ca-frontend/dist ./habi2ca-frontend/dist
# Copy the binary from the backend-build stage
COPY --from=backend-build /habi2ca/target/release/habi2ca-server ./habi2ca-server

# Set the startup command to run your binary
CMD ["sh", "-c", "./habi2ca-server ${DATABASE_PATH} 0.0.0.0 ${PORT}"]
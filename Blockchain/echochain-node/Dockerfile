# Use a Rust base image
FROM rustlang/rust:nightly-slim-bullseye as builder

# Set working directory
WORKDIR /app/echochain-node

# Install necessary dependencies for Substrate
RUN apt-get update && apt-get install -y \
    clang \
    libssl-dev \
    pkg-config \
    build-essential \
    git \
    protobuf-compiler \
    cmake \
    --no-install-recommends && \
    rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build the node
# Use --release for optimized build
# Adjust features if necessary based on your node's Cargo.toml
RUN cargo build --release --locked --features=runtime-benchmarks

# --- Runtime image ---
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS
RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/echochain-node/target/release/echochain-node /usr/local/bin/

# Expose the RPC port (default for Substrate is 9933 or 9944)
EXPOSE 9944

# Run the node
# Adjust command line arguments as needed for your specific node configuration
CMD ["echochain-node", "--dev", "--ws-external", "--rpc-external", "--rpc-cors=all", "--tmp"]

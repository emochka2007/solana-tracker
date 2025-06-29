# Build stage
FROM rust:1.88-slim-bullseye AS builder

WORKDIR /usr/src/app

# Copy workspace files
COPY Cargo.toml ./
COPY programs/ programs/
COPY app/ app/

# Install build dependencies, including make
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    make \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Build workspace in release mode and list binaries for debugging
RUN cargo build --release --workspace && \
    find target/release -maxdepth 1 -type f -executable

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the sol-tracker binary
COPY --from=builder /usr/src/app/target/release/sol-tracker /usr/local/bin/sol-tracker

# Copy potential config or data files (modify based on your app's needs)
# Example: COPY --from=builder /usr/src/app/app/config.toml /usr/local/bin/config.toml

# Ensure the binary is executable
RUN chmod +x /usr/local/bin/sol-tracker

# List files in /usr/local/bin for debugging
RUN ls -l /usr/local/bin

WORKDIR /usr/local/bin
RUN touch .env
# Run the main app with verbose output
CMD ["./sol-tracker"]
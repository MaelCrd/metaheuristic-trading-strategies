# Stage 1: Build
FROM rust:1.83 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies only to cache them
# RUN cargo fetch
# RUN cargo build --release

# Copy the project files
COPY src ./src

# Compile the application in release mode
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/backend ./backend

# Set the Rocket address to listen on all interfaces
ENV ROCKET_ADDRESS=0.0.0.0

# Expose the application port (change as necessary)
EXPOSE 9797

# Run the application
CMD ["/app/backend"]

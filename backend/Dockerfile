# Stage 1: Build
FROM rust:1.83 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Build dependencies only to cache them
# RUN cargo check
# RUN cargo build --release

RUN cargo install sqlx-cli --no-default-features --features postgres

# Copy the project files
COPY src ./src
COPY .sqlx ./.sqlx

RUN cargo sqlx prepare --check

# RUN cargo sqlx prepare

# Compile the application in release mode
RUN SQLX_OFFLINE=true cargo build --release

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

# ENV DATABASE_URL=postgres://postgres:postgres@localhost:5432/mydb 

# Expose the application port (change as necessary)
EXPOSE 9797

# Run the application
CMD ["/app/backend"]

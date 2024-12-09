# Stage 1: Build
FROM rust:1.72 as builder

# Set the working directory
WORKDIR /usr/src/app

# Install system dependencies for MongoDB driver
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy the manifest file
COPY Cargo.toml Cargo.lock ./

# Pre-build dependencies
RUN cargo fetch
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

# Install system dependencies for MongoDB driver
RUN apt-get update && apt-get install -y libssl-dev && apt-get clean

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/app/target/release/my_app .

# Expose the application port
EXPOSE 8080

# Set the startup command
CMD ["./my_app"]

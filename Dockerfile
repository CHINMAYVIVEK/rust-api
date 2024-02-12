ARG PROJECT_NAME=rust-api

# Use the official Rust image as a builder
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies to cache them
RUN cargo build --release

# Copy the source code
COPY src ./src

# Build the application
RUN cargo install --path .

# Start a new build stage
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/${PROJECT_NAME} .

# Run the application
CMD ["./${PROJECT_NAME}"]

# Use the official Rust image as a builder
FROM rust:latest AS builder

# Install dependencies and update Cargo
RUN apt-get update && apt-get install -y libssl-dev

# Set the working directory inside the container
WORKDIR /usr/src/rust-cli-calculator

# Copy the Cargo.toml and Cargo.lock into the working directory
COPY Cargo.toml Cargo.lock ./

# Update Rust and Cargo
RUN rustup update stable

# Copy the src/ directory into the container
COPY ./src ./src

# Build the dependencies (this will cache dependencies if not changed)
RUN cargo build --release

# Final image - smaller and without unnecessary build tools
FROM debian:bookworm-slim 

# Install necessary runtime dependencies (with newer OpenSSL)
RUN apt-get update && apt-get install -y libssl3 libc6

# Set the working directory
WORKDIR /usr/src/rust-cli-calculator

# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/rust-cli-calculator/target/release/rust-cli-calculator .

# Command to run the binary
CMD ["./rust-cli-calculator"]

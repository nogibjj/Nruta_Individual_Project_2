# Start with a Rust image
FROM rust:1.70-slim-buster

# Install necessary tools
RUN apt-get update && \
    apt-get install -y sqlite3 libsqlite3-dev && \
    rustup default stable

# Set the working directory
WORKDIR /workspace

# Copy only the necessary files first to cache dependencies efficiently
COPY Cargo.toml Cargo.lock ./

# Build dependencies to speed up rebuilds
RUN cargo fetch

# Copy the rest of the project files
COPY . .

# Build the Rust project in release mode
RUN cargo build --release

# Run the CLI binary by default
CMD ["./target/release/your_binary_name"]
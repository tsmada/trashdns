# First stage: Build
# Use an official Rust runtime as a parent image
FROM rust:1.54 as builder

# Set the working directory
WORKDIR /usr/src/trashdns

# Copy the current directory contents into the container
COPY . .

RUN rustup default nightly && rustup update

# Build the application
RUN cargo build --release

# Second stage: Runtime
# Use a lightweight image for the application runtime
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/local/bin

# Copy the built application from the builder stage
COPY --from=builder /usr/src/trashdns/target/release/trashdns .

# Expose the DNS port
EXPOSE 53/udp

# Set the startup command to run the application
CMD ["./trashdns"]

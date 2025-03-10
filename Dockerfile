# Use Rust as the base image
FROM rust:1.85.0

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the current directory contents into the container
COPY . .

# Build the application
RUN cargo build --release

# Expose port 7979
EXPOSE 10000

# Define the command to run the application
CMD ["./target/release/test-axum"]
# Use the official Rust image from the Docker Hub
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Expose the port your application will run on
EXPOSE 5000

# Run the compiled binary
CMD ["cargo", "build"]

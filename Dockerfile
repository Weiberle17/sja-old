# This Dockerfile is used to build the entire application

# Use the official Docker image for Docker Compose
FROM docker/compose:1.29.2

# Set the working directory
WORKDIR /usr/src/app

# Copy the Docker Compose file
COPY docker-compose.yaml .

# Install Docker Compose
RUN apk add --no-cache python3 && pip3 install docker-compose

# Start Docker Compose
CMD ["docker-compose", "up"]

# Use an official Python runtime as a parent image
FROM python:3.12.4-slim

# Set the working directory in the container
WORKDIR /app

# Install pipx
RUN pip install pipx
RUN pipx ensurepath

# Install goose-ai using pipx
RUN pipx install goose-ai

# Copy the entrypoint script
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Create the profile directory
RUN mkdir -p /root/.goose/config

ENTRYPOINT ["/entrypoint.sh"]
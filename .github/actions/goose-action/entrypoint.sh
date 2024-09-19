#!/bin/sh

# Write the profile content to the expected location
echo "$PROFILE" > /root/.goose/config/profile.yaml

# Execute the command passed to the Docker container
goose session start --plan plan.yaml
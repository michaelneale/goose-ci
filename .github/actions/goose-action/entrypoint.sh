#!/bin/sh

# Write the profile content to the expected location
mv /root/.goose/config/profiles.yaml /root/.goose/config/profile.yaml

# Execute the command passed to the Docker container
# Start the Goose session and run it in the background
goose version

# Poll for success or failure file
while true; do
    if [ -f /root/.goose/config/success ]; then
        echo "Goose session succeeded"
        exit 0
    elif [ -f /root/.goose/config/failure ]; then
        echo "Goose session failed"
        exit 1
    fi
    sleep 10  # Adjust the sleep interval as needed
done
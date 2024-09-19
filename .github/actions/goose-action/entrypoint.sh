#!/bin/sh


# Execute the command passed to the Docker container
# Start the Goose session and run it in the background
goose version

# Poll for success or failure file
while true; do
    if [ -f /app/success ]; then
        echo "Goose session succeeded"
        exit 0
    elif [ -f /app/failure ]; then
        echo "Goose session failed"
        exit 1
    fi
    sleep 10  # Adjust the sleep interval as needed
done
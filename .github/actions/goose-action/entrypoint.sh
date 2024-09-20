#!/bin/sh

# Access the 'task_request' input
TASK_REQUEST=${INPUT_TASK_REQUEST}
echo "Task Request: $TASK_REQUEST"

# Replace newlines with spaces in TASK_REQUEST
TASK_REQUEST=$(echo $TASK_REQUEST | tr '\n' ' ')

# Replace {TASK} in plan.yaml with the TASK_REQUEST value
sed -i "s/{TASK}/$TASK_REQUEST/g" plan.yaml

# Start the Goose session in the background
goose session start --plan plan.yaml &

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

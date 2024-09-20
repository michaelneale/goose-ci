#!/bin/sh

# Access the 'task_request' input
TASK_REQUEST=${INPUT_TASK_REQUEST}
echo "Task Request: $TASK_REQUEST"

# Replace newlines with spaces in TASK_REQUEST
TASK_REQUEST=$(echo $TASK_REQUEST | tr '\n' ' ')

echo "Formatted Task Request: $TASK_REQUEST"

# Replace {TASK} in plan.yaml with the TASK_REQUEST value
sed -i "s/{TASK}/$TASK_REQUEST/g" /app/plan.yaml

# Start the Goose session in the background
goose session start --plan /app/plan.yaml &

# Save the PID of the Goose session
GOOSE_PID=$!

# Poll for success or failure file
while true; do
    if [ -f /app/success ]; then
        echo "Goose session succeeded"
        kill -9 $GOOSE_PID  # Gracefully stop Goose session
        exit 0
    elif [ -f /app/failure ]; then
        echo "Goose session failed"
        kill -9 $GOOSE_PID  # Gracefully stop Goose session
        exit 1
    fi
    sleep 10  # Adjust the sleep interval as needed
done

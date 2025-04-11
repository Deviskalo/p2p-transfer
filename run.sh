#!/bin/bash

# Run script for Unix-based systems

# Navigate to the project directory
cd "$(dirname "$0")/p2p-transfer"

# Start the backend server
echo "Starting the backend server..."
( cd backend && cargo run ) &

# Start the frontend server
echo "Starting the frontend server..."
( cd frontend && npm run dev ) &

# Wait for both processes to finish
wait

echo "Both backend and frontend servers are running."

sleep 2
echo "The backend server is running on port 5000."
echo "The frontend server is typically running on port 3000."

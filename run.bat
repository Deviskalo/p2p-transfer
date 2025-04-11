@echo off
REM Run script for Windows

REM Navigate to the project directory
cd p2p-transfer

REM Start the backend server
start cmd /k "cd backend && cargo run"

REM Start the frontend server
start cmd /k "cd frontend && npm run dev"

echo Both backend and frontend servers are running.

echo The backend server is running on port 5000.
echo The frontend server is typically running on port 3000.

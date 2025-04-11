#!/bin/bash

# Setup script for Unix-based systems

echo "Setting up the environment for P2P File Transfer..."

# Check for Rust installation
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    echo "Please restart your terminal or run 'source $HOME/.cargo/env' to update your PATH."
else
    echo "Rust is already installed."
fi

# Check for Node.js installation
if ! command -v node &> /dev/null; then
    echo "Node.js is not installed. Installing Node.js..."
    curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
    sudo apt-get install -y nodejs
    echo "Please restart your terminal or run 'source $HOME/.nvm/nvm.sh' to update your PATH."
else
    echo "Node.js is already installed."
fi

# Check for Yarn installation
if ! command -v yarn &> /dev/null; then
    echo "Yarn is not installed. Installing Yarn..."
    npm install -g yarn
    echo "Please restart your terminal or run 'source $HOME/.nvm/nvm.sh' to update your PATH."
else
    echo "Yarn is already installed."
fi

# Install dependencies in the frontend
echo "Installing dependencies in the frontend..."
cd frontend
if [ -d "frontend" ]; then
    cd frontend
    if command -v yarn &> /dev/null; then
        echo "Installing frontend dependencies using Yarn..."
        yarn install
    else
        echo "Yarn is not installed, using npm instead..."
        npm install
    fi
    cd ..
else
    echo "Frontend directory not found."
    exit 1
fi



# Navigate to the backend directory if it exists
if [ -d "backend" ]; then
    cd backend
else
    echo "Backend directory not found."
    exit 1
fi

# Install dependencies
echo "Installing dependencies..."
cargo build
echo "Dependencies installed successfully."

# Run the server
echo "Starting the server..."
cargo run
echo "The backend server is running on port 5000."

echo "Setup complete! You can now run the server using 'cargo run'."

@echo off
REM Setup script for Windows

echo Setting up the environment for P2P File Transfer...

REM Check for Rust installation
where rustc >nul 2>nul
if %errorlevel% neq 0 (
    echo Rust is not installed. Installing Rust...
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    echo Please restart your terminal or run 'source %USERPROFILE%\.cargo\env' to update your PATH.
) else (
    echo Rust is already installed.
)


REM Clone the repository
if not exist "p2p-transfer" (
    echo Cloning the repository...
    git clone https://github.com/Deviskalo/p2p-transfer.git
)

cd p2p-transfer

REM Navigate to the backend directory if it exists
if exist "backend" (
    cd backend
) else (
    echo Backend directory not found.
    exit /b 1
)

REM Install backend dependencies
echo Installing backend dependencies...
cargo build

REM Navigate to the frontend directory if it exists
if exist "frontend" (
    cd ..\frontend
) else (
    echo Frontend directory not found.
    exit /b 1
)

REM Install frontend dependencies using yarn or npm
echo Installing frontend dependencies...
where yarn >nul 2>nul
if %errorlevel% neq 0 (
    echo Yarn is not installed. Using npm to install dependencies...
    npm install
) else (
    echo Yarn is installed. Using yarn to install dependencies...
    yarn install
)

echo Setup complete!.
echo You can now run the run.bat script to start the servers.
echo Hangon! Running the run.bat script to start the servers...
call run.bat


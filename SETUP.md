# Backend Setup for P2P File Transfer

This document provides instructions for setting up the backend environment for the P2P File Transfer project, which is built using Rust and the Axum framework.

## Prerequisites

Before you begin, ensure you have the following installed on your machine:

- **Rust**: You can install Rust using [rustup](https://rustup.rs/).
- **Cargo**: This is included with Rust and is used for managing Rust packages.
- **PostgreSQL** (optional): If your application requires a database, ensure PostgreSQL is installed and running.

## Installation Steps

### 1. Clone the Repository

First, clone the repository to your local machine:

```bash
git clone https://github.com/Deviskalo/p2p-transfer.git
cd p2p-transfer
```

### 2. Navigate to the Backend Directory

If your backend code is in a specific directory (e.g., `backend`), navigate to that directory:

```bash
cd backend
```

### 3. Install Dependencies

Use Cargo to install the necessary dependencies. Run the following command:

```bash
cargo build
```

This command will download and compile all the dependencies specified in the `Cargo.toml` file.

### 4. Run the Server

To start the backend server, use the following command:

```bash
cargo run
```

The server should now be running and listening on the specified port (default is 5000).

## Testing the Backend

You can test the backend by sending requests to the server using tools like [Postman](https://www.postman.com/) or `curl`. For example:

```bash
curl http://localhost:5000/api/your-endpoint
```

## Additional Notes

- Ensure that your Rust version is up to date. You can check your Rust version with:

```bash
rustc --version
```

- If you encounter any issues, refer to the [Axum documentation](https://docs.rs/axum/latest/axum/) for more information on setting up and configuring Axum.

## Getting Help

For any issues or questions, please refer to the [issues](https://github.com/Deviskalo/p2p-transfer/issues) or contact Dev Iskalo at [deviskalo2000@gmail.com](mailto:deviskalo2000@gmail.com).

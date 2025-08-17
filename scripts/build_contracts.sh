# !/bin/bash

# Check if Docker daemon is running
if ! docker info >/dev/null 2>&1; then
  echo "Docker daemon is not running. Attempting to start Docker..."
  # Start Docker Desktop (macOS-specific; adjust for other systems)
  open -a Docker
  # Wait for Docker to start (adjust timeout as needed)
  echo "Waiting for Docker to start..."
  sleep 10
  # Verify Docker is running
  if ! docker info >/dev/null 2>&1; then
    echo "Error: Failed to start Docker daemon. Please ensure Docker is installed and running."
    exit 1
  fi
fi

# Start Stellar container
stellar container start

sleep 30

# Add Stellar local network
stellar network add local \
  --rpc-url http://localhost:8000/soroban/rpc \
  --network-passphrase "Standalone Network ; February 2017"

# Generate and fund mulla account, ignore if already exists
echo "Generating and funding default account..."
stellar keys generate default --network local || {
  echo "Identity 'default' already exists. Proceeding..."
}

stellar keys fund default --network local

# Build the contract
cargo build --target wasm32v1-none --release

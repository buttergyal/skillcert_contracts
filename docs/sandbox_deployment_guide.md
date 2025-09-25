# SkillCert Contracts - Sandbox Deployment Guide

This comprehensive guide explains how to build, deploy, and interact with the SkillCert smart contracts (CourseAccess, CourseRegistry, and UserManagement) on a local Stellar network using the provided shell scripts.

## 📋 Prerequisites

Before running the deployment scripts, ensure the following tools are installed:

### Required Tools

**Docker** - Required to run the Stellar local network
```bash
# Install Docker Desktop on macOS/Windows or Docker on Linux
# Verify installation:
docker --version
```

**Stellar CLI** - Used to interact with the Stellar network and contracts
```bash
# Install via Cargo:
cargo install stellar-cli

# Verify installation:
stellar --version
```

**Rust and Cargo** - Needed to build the contract WASM files
```bash
# Install Rust: https://rustup.rs/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target:
rustup target add wasm32v1-none

# Verify installation:
cargo --version
rustc --version
```

**jq** (Recommended) - Simplifies JSON parsing for contract.json
```bash
# macOS:
brew install jq

# Ubuntu/Debian:
sudo apt-get install jq

# Windows (via Chocolatey):
choco install jq

# Verify installation:
jq --version
```

### Repository Setup

Clone and navigate to the repository:
```bash
git clone https://github.com/SkillCert/skillcert_contracts/
cd skillcert_contracts
```

## 🛠️ Scripts Overview

The repository includes three main shell scripts to manage the complete contract lifecycle:

| Script | Purpose | Output |
|--------|---------|---------|
| `build_contracts.sh` | Sets up Stellar local network, funds default account, builds WASM files | Contract WASM files in `target/` |
| `deploy_contracts.sh` | Deploys all contracts to local network | Contract IDs in `contract.json` |
| `invoke_examples.sh` | Demonstrates contract interactions | Example transactions |

### Available Contracts

- **CourseRegistry**: Manages course metadata, creation, and retrieval
- **CourseAccess**: Handles user permissions and course access control  
- **UserManagement**: Manages user profiles and administrative functions

## 🚀 Step-by-Step Deployment

### Step 1: Build Environment Setup

The `build_contracts.sh` script initializes the Stellar local network and builds all contract WASM files.

```bash
# Make script executable and run
chmod +x scripts/build_contracts.sh
./scripts/build_contracts.sh
```

**What this script does:**

1. **Docker Setup**: Checks if Docker daemon is running and starts Docker Desktop if needed
2. **Stellar Network**: Starts the Stellar container with local network configuration:
   - RPC URL: `http://localhost:8000/soroban/rpc`
   - Passphrase: `Standalone Network ; February 2017`
3. **Account Funding**: Generates and funds the default account for transactions
4. **Contract Compilation**: Builds all contracts producing WASM files:
   - `target/wasm32v1-none/release/course_access.wasm`
   - `target/wasm32v1-none/release/course_registry.wasm`  
   - `target/wasm32v1-none/release/user_management.wasm`

**Expected Output:**
```
✅ Docker is running
✅ Stellar container started
✅ Network configured: local
✅ Default account funded: GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
✅ Contracts built successfully
```

### Step 2: Deploy Contracts

The `deploy_contracts.sh` script deploys all three contracts and saves their addresses to `contract.json`.

```bash
# Make script executable and run
chmod +x scripts/deploy_contracts.sh
./scripts/deploy_contracts.sh
```

**What this script does:**

1. **Deploys CourseAccess Contract**: Handles user permissions and access control
2. **Deploys CourseRegistry Contract**: Manages course metadata and operations
3. **Deploys UserManagement Contract**: Manages user profiles and admin functions
4. **Saves Contract IDs**: Creates `contract.json` with all deployed contract addresses

**Generated contract.json:**
```json
{
  "course_access_contract": "CBF3FNVSN3EQKOCK7AVWHDUEUPPAOQCFUVO3RR7ABSAVG3CQH4HVDYRD",
  "course_registry_contract": "CAFNV3LJDUGATSPS6UJFGDIJLFZ2DDN3RIWKLSRGCIMXEWUZKDLOQM43",
  "user_management_contract": "CDAF4MNVSN3EQKOCK7AVWHDUEUPPAOQCFUVO3RR7ABSAVG3CQH4HVEXAMPLE"
}
```

**Expected Output:**
```
Deploying Course Access Contract...
Course Access Contract ID: CBF3FNVSN3EQKOCK7AVWHDUEUPPAOQCFUVO3RR7ABSAVG3CQH4HVDYRD
Deploying Course Registry Contract...
Course Registry Contract ID: CAFNV3LJDUGATSPS6UJFGDIJLFZ2DDN3RIWKLSRGCIMXEWUZKDLOQM43
Deploying User Management Contract...
User Management Contract ID: CDAF4MNVSN3EQKOCK7AVWHDUEUPPAOQCFUVO3RR7ABSAVG3CQH4HVEXAMPLE
✅ Saving contract addresses to contract.json...
```


### Step 3: Test Contract Functions

The `invoke_examples.sh` script demonstrates contract interactions by creating sample courses and testing various functions.

```bash
# Make script executable and run
chmod +x scripts/invoke_examples.sh
./scripts/invoke_examples.sh
```

**What this script demonstrates:**

1. **Course Creation**: Creates sample courses using CourseRegistry contract
2. **Course Retrieval**: Gets course data by ID and instructor
3. **Access Management**: Demonstrates user permission handling
4. **Data Validation**: Shows how contracts handle different data types

**Example Operations:**

**Create Course:**
```bash
stellar contract invoke \
  --id $(jq -r '.course_registry_contract' contract.json) \
  --source-account default \
  --network local \
  -- create_course \
  --creator $(stellar keys public-key default) \
  --title "Blockchain Fundamentals" \
  --description "Learn the basics of blockchain technology" \
  --price 1000 \
  --category null \
  --language null \
  --thumbnail_url null
```

**Grant Course Access:**
```bash
stellar contract invoke \
  --id $(jq -r '.course_access_contract' contract.json) \
  --source-account default \
  --network local \
  -- grant_access \
  --course_id '{"string": "1"}' \
  --user $(stellar keys public-key default)
```

**Get Courses by Instructor:**
```bash
stellar contract invoke \
  --id $(jq -r '.course_registry_contract' contract.json) \
  --source-account default \
  --network local \
  -- get_courses_by_instructor \
  --instructor $(stellar keys public-key default)
```


## 🔄 Complete Workflow Example

Here's the complete deployment workflow from start to finish:

```bash
# 1. Build environment and contracts
./scripts/build_contracts.sh

# 2. Deploy all contracts  
./scripts/deploy_contracts.sh

# 3. Verify deployment
cat contract.json

# 4. Test contract functions
./scripts/invoke_examples.sh
```

## 🛠️ Advanced Usage

### Manual Contract Interactions

**Create a Custom Course:**
```bash
stellar contract invoke \
  --id $(jq -r '.course_registry_contract' contract.json) \
  --source-account default \
  --network local \
  -- create_course \
  --creator $(stellar keys public-key default) \
  --title "Advanced Stellar Development" \
  --description "Master smart contract development on Stellar" \
  --price 2500 \
  --category "Development" \
  --language "English" \
  --thumbnail_url "https://example.com/thumbnail.jpg"
```

**Create User Profile:**
```bash
stellar contract invoke \
  --id $(jq -r '.user_management_contract' contract.json) \
  --source-account default \
  --network local \
  -- create_user_profile \
  --user_id $(stellar keys public-key default) \
  --name "John Developer" \
  --email "john@example.com" \
  --profile_picture_url "https://example.com/avatar.jpg"
```

**List All Courses:**
```bash
stellar contract invoke \
  --id $(jq -r '.course_registry_contract' contract.json) \
  --source-account default \
  --network local \
  -- get_all_courses
```

### Useful Commands

**Check Account Balance:**
```bash
stellar keys fund default --network local
stellar account balance --id default --network local
```

**Get Contract Info:**
```bash
stellar contract inspect --id $(jq -r '.course_registry_contract' contract.json) --network local
```

**View Transaction History:**
```bash
stellar events --start-ledger 1 --network local
```

## 🚨 Troubleshooting Guide

### Common Issues and Solutions

#### Docker Issues

**Problem**: `Cannot connect to the Docker daemon`
```bash
# Solution: Start Docker service
# macOS/Windows: Start Docker Desktop
# Linux: sudo systemctl start docker
docker --version  # Verify Docker is running
```

**Problem**: `docker start stellar` fails with "No such container"
```bash
# Solution: Create the Stellar container first
stellar network start local
# Or manually create container:
docker run -d --name stellar -p 8000:8000 stellar/quickstart:testing --local
```

#### Build Issues

**Problem**: `rustup target add wasm32v1-none` fails
```bash
# Solution: Update Rust and add target
rustup update
rustup target add wasm32v1-none
```

**Problem**: `cargo build` fails with dependency errors
```bash
# Solution: Clean and rebuild
cargo clean
cargo build --target wasm32v1-none --release
```

#### Deployment Issues

**Problem**: `Error: Failed to capture Contract ID`
```bash
# Solution: Check WASM files exist
ls -la target/wasm32v1-none/release/*.wasm

# Re-fund account if needed
stellar keys fund default --network local
```

**Problem**: `Insufficient funds` during deployment
```bash
# Solution: Fund the default account
stellar keys fund default --network local

# Check balance
stellar account balance --id default --network local
```

#### Network Issues

**Problem**: `connection refused` when invoking contracts
```bash
# Solution: Ensure Stellar network is running
stellar network ls
stellar network start local

# Check if container is running
docker ps | grep stellar
```

**Problem**: `Invalid network passphrase`
```bash
# Solution: Reset network configuration
stellar network remove local
stellar network add local \
  --rpc-url http://localhost:8000/soroban/rpc \
  --network-passphrase "Standalone Network ; February 2017"
```

### Environment Verification

Run this verification script to check your environment:

```bash
#!/bin/bash
echo "🔍 Environment Verification"
echo "=========================="

# Check Docker
if command -v docker &> /dev/null; then
    echo "✅ Docker: $(docker --version)"
else
    echo "❌ Docker: Not installed"
fi

# Check Stellar CLI
if command -v stellar &> /dev/null; then
    echo "✅ Stellar CLI: $(stellar --version)"
else
    echo "❌ Stellar CLI: Not installed"
fi

# Check Rust
if command -v cargo &> /dev/null; then
    echo "✅ Rust: $(cargo --version)"
    if rustup target list | grep -q "wasm32v1-none (installed)"; then
        echo "✅ WASM target: Installed"
    else
        echo "❌ WASM target: Not installed"
    fi
else
    echo "❌ Rust: Not installed"
fi

# Check jq
if command -v jq &> /dev/null; then
    echo "✅ jq: $(jq --version)"
else
    echo "⚠️  jq: Not installed (optional)"
fi

# Check network
if stellar network ls | grep -q "local"; then
    echo "✅ Stellar local network: Configured"
else
    echo "❌ Stellar local network: Not configured"
fi

echo "=========================="
```

## 📚 Additional Resources

- **Stellar CLI Documentation**: [stellar --help](https://stellar.org/docs)
- **Soroban Smart Contracts**: [soroban.stellar.org](https://soroban.stellar.org)
- **SkillCert Roadmap**: [Notion Roadmap](https://www.notion.so/Skillcert-240bfdf2613c805898c9c91f0990600e)
- **Contract Source Code**: [GitHub Repository](https://github.com/SkillCert/skillcert_contracts)

---

**Note**: This guide assumes you're working in the `skillcert_contracts` repository root directory. All scripts and commands should be executed from this location.
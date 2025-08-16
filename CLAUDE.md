# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Skillcert is a Web3 platform for issuing NFT-based digital certificates on the Stellar blockchain. The project consists of Soroban smart contracts organized in a workspace structure. Currently in integration phase between Web3 logic and frontend application.

## Build and Development Commands

### Building Contracts
```bash
# Build all contracts in release mode
cargo build --release --target wasm32-unknown-unknown

# Build a specific contract
cargo build --release --target wasm32-unknown-unknown -p course_registry

# Build with logging enabled
cargo build --profile release-with-logs --target wasm32-unknown-unknown
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for a specific contract
cargo test -p course_registry

# Run a specific test
cargo test test_create_course

# Run tests with output
cargo test -- --nocapture
```

### Deploy Contract (Soroban)
```bash
# Deploy contract to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/[contract_name].wasm \
  --source [SOURCE_KEY] \
  --network testnet

# Initialize contract after deployment
stellar contract invoke \
  --id [CONTRACT_ID] \
  --source [SOURCE_KEY] \
  --network testnet \
  -- initialize \
  --caller [CALLER_ADDRESS] \
  --user_mgmt_addr [USER_MGMT_CONTRACT] \
  --course_registry_addr [COURSE_REGISTRY_CONTRACT]
```

## Architecture

### Contract Structure
The repository follows a modular architecture with contracts organized by domain:

- **Course Module** (`contracts/course/`)
  - `course_registry`: Manages course metadata, modules, goals, and prerequisites
  - `course_access`: Handles user permissions and course enrollment

- **User Module** (`contracts/`)
  - `user_profile`: Manages user profile data with privacy controls
  - `user_management`: Handles user administration, roles, and authentication

### Key Design Patterns

1. **Function Organization**: Each contract follows a pattern where business logic is separated into individual function files under `src/functions/`:
   - One function per file for maintainability
   - Functions are imported and exposed through the main contract impl
   - Naming convention: `[contract_name]_[function_name]`

2. **Schema Definitions**: Each contract has a `schema.rs` file containing:
   - `contracttype` structs for data models
   - `DataKey` enums for storage organization
   - Shared types used across functions

3. **Storage Pattern**: Contracts use Soroban SDK's storage with typed keys:
   - Composite keys for relationships: `DataKey::CourseGoal(course_id, goal_id)`
   - List keys for collections: `DataKey::CourseGoalList(course_id)`
   - Single keys for entities: `DataKey::Module(module_id)`

4. **Cross-Contract Communication**: Contracts interact through:
   - Initialization with contract addresses
   - Direct invocation using stored contract references
   - Admin verification across contracts

### Testing Strategy
- Unit tests are colocated in `test.rs` files
- Snapshot testing for complex operations (see `test_snapshots/`)
- Test utilities mock contract environments and user interactions
- Tests verify both success paths and error conditions

## Contract Interfaces

### CourseRegistry
- `create_course`: Creates new course with metadata
- `add_module/remove_module`: Manages course content
- `add_goal/edit_goal`: Manages learning objectives
- `create_prerequisite/remove_prerequisite`: Manages course dependencies
- `archive_course/delete_course`: Course lifecycle management

### CourseAccess
- `initialize`: One-time setup with dependency contracts
- `grant_access/revoke_access`: Manage user enrollment
- `list_user_courses/list_course_access`: Query enrollments
- `transfer_course_access`: Transfer ownership between users

### UserManagement
- `save_profile`: Create/update user profiles
- `is_admin`: Check administrative privileges
- `delete_user`: Soft delete user accounts
- `list_all_registered_users`: Query active users

### UserProfile
- `get_user_profile`: Retrieve profile data
- `get_user_profile_with_privacy`: Privacy-aware profile access

## Important Notes

- All contracts use Soroban SDK v22
- Contracts compile to WASM for Stellar blockchain deployment
- Profile `release` optimizes for size with `opt-level = "z"`
- Profile `release-with-logs` enables debug assertions for development
- Storage keys follow typed patterns to prevent collisions
- Functions return Results for proper error handling
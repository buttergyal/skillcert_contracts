# GitHub Actions Workflows

This directory contains CI/CD workflows for the SkillCert Contracts project.

## Workflows

### `ci.yml` - Continuous Integration
Runs on every push and pull request to `main`:
- ✅ Checks code formatting
- 🔧 Runs Clippy for static analysis
- 🏗️ Builds all contracts
- 🧪 Runs all tests
- 📦 Builds optimized contracts with Soroban

### `release.yml` - Release
Runs when a `v*` tag is created:
- 📦 Builds optimized contracts
- 🚀 Creates a release with WASM files

## System Dependencies

The workflows automatically install:
- `libdbus-1-dev` - Required by Soroban CLI
- `pkg-config` - To find system libraries

## Test Locally

Run the local test script before pushing:

```bash
chmod +x scripts/ci-test.sh
./scripts/ci-test.sh
```

## Caching

The workflows use cache for:
- Cargo registry
- Compiled dependencies
- Soroban CLI binary

This significantly speeds up builds.

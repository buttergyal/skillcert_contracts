#!/bin/bash

# ============================================================================
# SkillCert Contracts Deployment Script
# ============================================================================
# This script deploys all SkillCert smart contracts to the local Stellar network
# and saves their contract IDs to contract.json for later use.
#
# Prerequisites:
# - Stellar CLI installed and configured
# - Local Stellar network running (run build_contracts.sh first)
# - Contract WASM files built in target/wasm32v1-none/release/
#
# Usage: ./scripts/deploy_contracts.sh
# ============================================================================

set -e  # Exit on any error

echo "🚀 SkillCert Contracts Deployment"
echo "=================================="

# Check if WASM files exist
WASM_DIR="target/wasm32v1-none/release"
if [ ! -d "$WASM_DIR" ]; then
    echo "❌ Error: WASM directory not found. Run ./scripts/build_contracts.sh first."
    exit 1
fi

# Check required WASM files
REQUIRED_WASM_FILES=("course_access.wasm" "course_registry.wasm" "user_management.wasm")
for wasm_file in "${REQUIRED_WASM_FILES[@]}"; do
    if [ ! -f "$WASM_DIR/$wasm_file" ]; then
        echo "❌ Error: $wasm_file not found. Run ./scripts/build_contracts.sh first."
        exit 1
    fi
done

echo "✅ All WASM files found"

# Deploy Course Access Contract
echo ""
echo "📦 Deploying Course Access Contract..."
course_access_output=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/course_access.wasm \
  --source-account default \
  --network local 2>&1)
course_access_id=$(echo "$course_access_output" | grep -E '^[A-Z0-9]{56}$' | tail -n 1)
if [ -z "$course_access_id" ]; then
  echo "❌ Error: Failed to capture Course Access Contract ID."
  echo "Output: $course_access_output"
  exit 1
fi
echo "✅ Course Access Contract ID: $course_access_id"

# Deploy Course Registry Contract
echo ""
echo "📚 Deploying Course Registry Contract..."
course_registry_output=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/course_registry.wasm \
  --source-account default \
  --network local 2>&1)
course_registry_id=$(echo "$course_registry_output" | grep -E '^[A-Z0-9]{56}$' | tail -n 1)
if [ -z "$course_registry_id" ]; then
  echo "❌ Error: Failed to capture Course Registry Contract ID."
  echo "Output: $course_registry_output"
  exit 1
fi
echo "✅ Course Registry Contract ID: $course_registry_id"

# Deploy User Management Contract
echo ""
echo "👤 Deploying User Management Contract..."
user_management_output=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/user_management.wasm \
  --source-account default \
  --network local 2>&1)
user_management_id=$(echo "$user_management_output" | grep -E '^[A-Z0-9]{56}$' | tail -n 1)
if [ -z "$user_management_id" ]; then
  echo "❌ Error: Failed to capture User Management Contract ID."
  echo "Output: $user_management_output"
  exit 1
fi
echo "✅ User Management Contract ID: $user_management_id"

# Create contract.json with all deployed contract addresses
echo ""
echo "💾 Saving contract addresses to contract.json..."
cat > contract.json << EOF
{
  "course_access_contract": "$course_access_id",
  "course_registry_contract": "$course_registry_id",
  "user_management_contract": "$user_management_id"
}
EOF

echo "✅ Contract addresses saved to contract.json"
echo ""
echo "🎉 Deployment completed successfully!"
echo "=================================="
echo "📄 Contract Summary:"
echo "  • CourseAccess: $course_access_id"
echo "  • CourseRegistry: $course_registry_id" 
echo "  • UserManagement: $user_management_id"
echo ""
echo "📝 Next steps:"
echo "  1. Verify deployment: cat contract.json"
echo "  2. Test contracts: ./scripts/invoke_examples.sh"
echo "  3. View full guide: docs/sandbox_deployment_guide.md"
echo ""
# !/bin/bash

# Deploy Course Access Contract and capture address
echo "Deploying Course Access Contract..."
course_access_output=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/course_access.wasm \
  --source-account default \
  --network local 2>&1)
course_access_id=$(echo "$course_access_output" | grep -E '^[A-Z0-9]{56}$' | tail -n 1)
if [ -z "$course_access_id" ]; then
  echo "Error: Failed to capture Course Access Contract ID."
  exit 1
fi
echo "Course Access Contract ID: $course_access_id"

# Deploy Course Registry Contract and capture address
echo "Deploying Course Registry Contract..."
course_registry_output=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/course_registry.wasm \
  --source-account default \
  --network local 2>&1)
course_registry_id=$(echo "$course_registry_output" | grep -E '^[A-Z0-9]{56}$' | tail -n 1)
if [ -z "$course_registry_id" ]; then
  echo "Error: Failed to capture Course Registry Contract ID."
  exit 1
fi
echo "Course Registry Contract ID: $course_registry_id"

# Deploy User Management and capture address
echo "Deploying User Management Contract..."
user_management_output=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/user_management.wasm \
  --source-account default \
  --network local 2>&1)
user_management_id=$(echo "$user_management_output" | grep -E '^[A-Z0-9]{56}$' | tail -n 1)
if [ -z "$user_management_id" ]; then
  echo "Error: Failed to capture Course Registry Contract ID."
  exit 1
fi
echo "Course Registry Contract ID: $user_management_id"

# Create contract.json with the deployed addresses
echo "Saving contract addresses to contract.json..."
cat > contract.json << EOF
{
  "course_access_contract": "$course_access_id",
  "course_registry_contract": "$course_registry_id",
  "user_management_contract": "$user_management_id"
}
EOF
#!/bin/bash

# Check if contract.json exists
if [ ! -f contract.json ]; then
  echo "Error: contract.json not found. Please run deploy_contracts.sh first."
  exit 1
fi

# Extract course_registry_contract ID from contract.json
if command -v jq >/dev/null 2>&1; then
  contract_id=$(jq -r '.course_registry_contract' contract.json)
else
  # Fallback to grep/sed if jq is not installed
  contract_id=$(grep -o '"course_registry_contract": "[A-Z0-9]\{56\}"' contract.json | sed 's/.*: "\(.*\)"/\1/')
fi

if [ -z "$contract_id" ]; then
  echo "Error: Failed to extract course_registry_contract ID from contract.json."
  exit 1
fi
echo "Using CourseRegistry Contract ID: $contract_id"

# Get the default account's public key for --creator
echo "Retrieving default account public key..."
creator_address=$(stellar keys public-key default)
if [ -z "$creator_address" ]; then
  echo "Error: Failed to retrieve default account public key."
  exit 1
fi
echo "Default account public key: $creator_address"

# Create first course
echo "Creating first course..."
create_course_1_output=$(stellar contract invoke \
  --id "$contract_id" \
  --source-account default \
  --network local \
  -- create_course \
  --creator "$creator_address" \
  --title "title" \
  --description "A description" \
  --price 1000 \
  --category null \
  --language null \
  --thumbnail_url null)
if [ $? -eq 0 ]; then
  echo "First course created successfully."
  echo "$create_course_1_output"
else
  echo "Error: Failed to create first course."
  exit 1
fi

# Create second course
echo "Creating second course..."
create_course_2_output=$(stellar contract invoke \
  --id "$contract_id" \
  --source-account default \
  --network local \
  -- create_course \
  --creator "$creator_address" \
  --title "A new title" \
  --description "A new description" \
  --price 2000 \
  --category null \
  --language null \
  --thumbnail_url null)
if [ $? -eq 0 ]; then
  echo "Second course created successfully."
  echo "$create_course_2_output"
else
  echo "Error: Failed to create second course."
  exit 1
fi

# Get course with course_id="1"
echo "Fetching course with course_id=1..."
get_course_output=$(stellar contract invoke \
  --id "$contract_id" \
  --source-account default \
  --network local \
  -- get_course \
  --course_id "1")
if [ $? -eq 0 ]; then
  echo "Successfully fetched course with course_id=1."
  echo "$get_course_output"
else
  echo "Error: Failed to fetch course with course_id=1. Ensure the course exists and is not archived."
fi

# Get courses by instructor
echo "Fetching courses by instructor..."
get_courses_output=$(stellar contract invoke \
  --id "$contract_id" \
  --source-account default \
  --network local \
  -- get_courses_by_instructor \
  --instructor "$creator_address")
if [ $? -eq 0 ]; then
  echo "Successfully fetched courses by instructor."
  echo "$get_courses_output"
else
  echo "Error: Failed to fetch courses by instructor."
fi
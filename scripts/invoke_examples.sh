#!/bin/bash

# Check if contract.json exists
if [ ! -f contract.json ]; then
  echo "Error: contract.json not found. Please run deploy_contracts.sh first."
  exit 1
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"

# Extract course_access_contract and course_registry_contract IDs from contract.json
if command -v jq >/dev/null 2>&1; then
  course_access_id=$(jq -r '.course_access_contract' contract.json)
  course_registry_id=$(jq -r '.course_registry_contract' contract.json)
  user_management_id=$(jq -r '.user_management_contract' contract.json)
else
  # Fallback to grep/sed if jq is not installed
  course_access_id=$(grep -o '"course_access_contract": "[A-Z0-9]\{56\}"' contract.json | sed 's/.*: "\(.*\)"/\1/')
  course_registry_id=$(grep -o '"course_registry_contract": "[A-Z0-9]\{56\}"' contract.json | sed 's/.*: "\(.*\)"/\1/')
  user_management_id=$(grep -o '"user_management_contract": "[A-Z0-9]\{56\}"' contract.json | sed 's/.*: "\(.*\)"/\1/')
fi

if [ -z "$course_access_id" ] || [ -z "$course_registry_id" ] || [ -z "$user_management_id" ]; then
  echo "Error: Failed to extract contract IDs from contract.json."
  exit 1
fi
echo "Using CourseAccess Contract ID: $course_access_id"
echo "Using CourseRegistry Contract ID: $course_registry_id"
echo "Using UserManagement Contract ID: $user_management_id"

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Get the default account's public key for --creator
echo "Retrieving default account public key..."
creator_address=$(stellar keys public-key default)
if [ -z "$creator_address" ]; then
  echo "Error: Failed to retrieve default account public key."
  exit 1
fi
echo "Default account public key: $creator_address"

$(stellar keys generate another)
another_address=$(stellar keys public-key another)

echo "Another account public key: $another_address"

# Course Registry
echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Create first course
echo "Creating first course..."
create_course_1_output=$(stellar contract invoke \
  --id "$course_registry_id" \
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

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Create second course
echo "Creating second course..."
create_course_2_output=$(stellar contract invoke \
  --id "$course_registry_id" \
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

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Get course with course_id="1"
echo "Fetching course with course_id=1..."
get_course_output=$(stellar contract invoke \
  --id "$course_registry_id" \
  --source-account default \
  --network local \
  -- get_course \
  --course_id '{"string": "1"}' )
if [ $? -eq 0 ]; then
  echo "Successfully fetched course with course_id=1."
  echo "$get_course_output"
else
  echo "Error: Failed to fetch course with course_id=1. Ensure the course exists and is not archived."
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Get course with course_id="1"
echo "Fetching course with course_id=1..."
get_course_output=$(stellar contract invoke \
  --id "$course_registry_id" \
  --source-account default \
  --network local \
  -- get_course \
  --course_id '{"string": "1"}' )
if [ $? -eq 0 ]; then
  echo "Successfully fetched course with course_id=1."
  echo "$get_course_output"
else
  echo "Error: Failed to fetch course with course_id=1. Ensure the course exists and is not archived."
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Get courses by instructor
echo "Fetching courses by instructor..."
get_courses_output=$(stellar contract invoke \
  --id "$course_registry_id" \
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


# CourseA Access

# Initialize CourseAccess contract
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Initializing CourseAccess contract..."
initialize_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- initialize \
  --caller "$creator_address" \
  --user_mgmt_addr "$user_management_id" \
  --course_registry_addr "$course_registry_id")
if [ $? -eq 0 ]; then
  echo "CourseAccess contract initialized successfully."
  echo "$initialize_output"
else
  echo "Error: Failed to initialize CourseAccess contract."
  exit 1
fi


# Save profile
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Saving user profile..."
save_profile_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- save_profile \
  --name "name" \
  --email "email@gmail.com" \
  --profession "Software Developer" \
  --goals null \
  --country "Nigeria")
if [ $? -eq 0 ]; then
  echo "User profile saved successfully."
  echo "$save_profile_output"
else
  echo "Error: Failed to save user profile."
  exit 1
fi

# Grant access to course
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Granting access to course_id=1..."
grant_access_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- grant_access \
  --course_id '{"string": "1"}' \
  --user "$another_address")
if [ $? -eq 0 ]; then
  echo "Access granted successfully for course_id=1."
  echo "$grant_access_output"
else
  echo "Error: Failed to grant access for course_id=1. Ensure the course exists."
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Granting access to course_id=1..."
grant_access_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- grant_access \
  --course_id '{"string": "1"}' \
  --user "$creator_address")
if [ $? -eq 0 ]; then
  echo "Access granted successfully for course_id=1."
  echo "$grant_access_output"
else
  echo "Error: Failed to grant access for course_id=1. Ensure the course exists."
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Granting access to course_id=2..."
grant_access_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- grant_access \
  --course_id '{"string": "2"}' \
  --user "$another_address")
if [ $? -eq 0 ]; then
  echo "Access granted successfully for course_id=2."
  echo "$grant_access_output"
else
  echo "Error: Failed to grant access for course_id=2. Ensure the course exists."
fi

# List course access
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Listing access for course_id=1..."
list_course_access_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- list_course_access \
  --course_id '{"string": "1"}')
if [ $? -eq 0 ]; then
  echo "Successfully listed access for course_id=1."
  echo "$list_course_access_output"
else
  echo "Error: Failed to list access for course_id=1."
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Listing access for course_id=2..."
list_course_access_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- list_course_access \
  --course_id '{"string": "2"}')
if [ $? -eq 0 ]; then
  echo "Successfully listed access for course_id=1."
  echo "$list_course_access_output"
else
  echo "Error: Failed to list access for course_id=1."
fi

# List user courses
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Listing courses for user..."
list_user_courses_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- list_user_courses \
  --user "$another_address")
if [ $? -eq 0 ]; then
  echo "Successfully listed courses for user."
  echo "$list_user_courses_output"
else
  echo "Error: Failed to list courses for user."
fi

# List user courses
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Listing courses for user creator..."
list_user_courses_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- list_user_courses \
  --user "$creator_address")
if [ $? -eq 0 ]; then
  echo "Successfully listed courses for user."
  echo "$list_user_courses_output"
else
  echo "Error: Failed to list courses for user."
fi

# Revoke user courses
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Revoke access for user ..."
list_user_courses_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- revoke_access \
  --course_id '{"string": "2"}' \
  --user "$another_address")
if [ $? -eq 0 ]; then
  echo "Successfully revoked course for user."
  echo "$list_user_courses_output"
else
  echo "Error: Failed to list courses for user."
fi



# List course access
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Listing access for course_id=2 After Revoke..."
list_course_access_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- list_course_access \
  --course_id '{"string": "2"}')
if [ $? -eq 0 ]; then
  echo "Successfully listed access for course_id=2."
  echo "$list_course_access_output"
else
  echo "Error: Failed to list access for course_id=2."
fi

# List user courses
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "Listing courses for user..."
list_user_courses_output=$(stellar contract invoke \
  --id "$course_access_id" \
  --source-account default \
  --network local \
  -- list_user_courses \
  --user "$another_address")
if [ $? -eq 0 ]; then
  echo "Successfully listed courses for user."
  echo "$list_user_courses_output"
else
  echo "Error: Failed to list courses for user."
fi


# UserManagement contract
echo "--------------------------------------------------------------------------------------------------------------------------------------"
echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Initialize UserManagement contract
echo "Initializing UserManagement contract..."
initialize_user_mgmt_output=$(stellar contract invoke \
  --id "$user_management_id" \
  --source-account default \
  --network local \
  -- initialize_system \
  --initializer "$creator_address" \
  --super_admin "$creator_address" \
  --max_page_size 100)
if [ $? -eq 0 ]; then
  echo "UserManagement contract initialized successfully."
  echo "$initialize_user_mgmt_output"
else
  echo "Error: Failed to initialize UserManagement contract."
  exit 1
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Check if UserManagement is initialized
echo "Check if UserManagement is initialized..."
is_initialize_user_mgmt_output=$(stellar contract invoke \
  --id "$user_management_id" \
  --source-account default \
  --network local \
  -- is_system_initialized \
  )
if [ $? -eq 0 ]; then
  echo "$is_initialize_user_mgmt_output"
else
  echo "Error: Failed to initialize UserManagement contract."
  exit 1
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Save profile in UserManagement contract
echo "Saving user profile in UserManagement..."
save_user_mgmt_profile_output=$(stellar contract invoke \
  --id "$user_management_id" \
  --source-account default \
  --network local \
  -- save_profile \
  --name "name" \
  --email "email@gmail.com" \
  --profession "Software Developer" \
  --goals null \
  --country "Nigeria" \
  --user "$creator_address")
if [ $? -eq 0 ]; then
  echo "User profile saved successfully in UserManagement."
  echo "$save_user_mgmt_profile_output"
else
  echo "Error: Failed to save user profile in UserManagement."
  exit 1
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Get user by ID
echo "Fetching user by ID..."
get_user_by_id_output=$(stellar contract invoke \
  --id "$user_management_id" \
  --source-account default \
  --network local \
  -- get_user_by_id \
  --requester "$creator_address" \
  --user_id "$creator_address")
if [ $? -eq 0 ]; then
  echo "Successfully fetched user by ID."
  echo "$get_user_by_id_output"
else
  echo "Error: Failed to fetch user by ID."
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Check if user is admin
echo "Checking if user is admin..."
is_admin_output=$(stellar contract invoke \
  --id "$user_management_id" \
  --source-account default \
  --network local \
  -- is_admin \
  --who "$creator_address")
if [ $? -eq 0 ]; then
  echo "Successfully checked admin status."
  echo "$is_admin_output"
else
  echo "Error: Failed to check admin status."
fi

echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Adding admin
echo "Adding admin..."
add_admin_output=$(stellar contract invoke \
  --id "$user_management_id" \
  --source-account default \
  --network local \
  -- add_admin \
  --caller "$creator_address" \
  --new_admin "$another_address")
if [ $? -eq 0 ]; then
  echo "Successfully added admin."
  echo "$add_admin_output"
else
  echo "Error: Failed to add admin."
fi


echo "--------------------------------------------------------------------------------------------------------------------------------------"
# Get list of admins
echo "Fetching list of admins..."
get_admins_output=$(stellar contract invoke \
  --id "$user_management_id" \
  --source-account default \
  --network local \
  -- get_admins \
  --caller "$creator_address")
if [ $? -eq 0 ]; then
  echo "Successfully fetched list of admins."
  echo "$get_admins_output"
else
  echo "Error: Failed to fetch list of admins."
fi
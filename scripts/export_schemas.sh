#!/bin/bash

set -e

echo "📋 Exporting Contract Schemas"
echo "=================================="

# Create schemas directory
SCHEMA_DIR="schemas"
mkdir -p $SCHEMA_DIR

# Fix the WASM paths to match your build target
WASM_DIR="target/wasm32v1-none/release"

# Function to export contract bindings
export_contract_schemas() {
    local contract_name=$1
    local wasm_path=$2
    
    echo "📦 Exporting schema for $contract_name..."
    
    # Generate TypeScript bindings
    stellar contract bindings typescript \
        --wasm "$wasm_path" \
        --output-dir "$SCHEMA_DIR/$contract_name-ts" \
        --network local || {
            echo "⚠️  Warning: Could not generate TypeScript bindings for $contract_name"
        }
    
    # Generate JSON bindings
    stellar contract bindings json \
        --wasm "$wasm_path" > "$SCHEMA_DIR/${contract_name}_schema.json" || {
            echo "⚠️  Warning: Could not generate JSON bindings for $contract_name"
        }
    
    # Export human-readable documentation
    stellar contract inspect \
        --wasm "$wasm_path" \
        --output docs > "$SCHEMA_DIR/${contract_name}_docs.md" || {
            echo "⚠️  Warning: Could not export docs for $contract_name"
        }
}

# Export schemas for each contract
export_contract_schemas "course_access" "$WASM_DIR/course_access.wasm"
export_contract_schemas "course_registry" "$WASM_DIR/course_registry.wasm"
export_contract_schemas "user_management" "$WASM_DIR/user_management.wasm"

# Create a combined metadata file
echo ""
echo "📄 Creating combined metadata file..."
cat > "$SCHEMA_DIR/contracts_metadata.json" << EOF
{
  "contracts": {
    "course_access": {
      "wasm": "$WASM_DIR/course_access.wasm",
      "schema": "schemas/course_access_schema.json",
      "docs": "schemas/course_access_docs.md",
      "typescript": "schemas/course_access-ts/"
    },
    "course_registry": {
      "wasm": "$WASM_DIR/course_registry.wasm",
      "schema": "schemas/course_registry_schema.json",
      "docs": "schemas/course_registry_docs.md",
      "typescript": "schemas/course_registry-ts/"
    },
    "user_management": {
      "wasm": "$WASM_DIR/user_management.wasm",
      "schema": "schemas/user_management_schema.json",
      "docs": "schemas/user_management_docs.md",
      "typescript": "schemas/user_management-ts/"
    }
  },
  "generated_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF

echo "✅ Schemas exported to $SCHEMA_DIR/"
echo ""
echo "📝 Generated files:"
echo "  • TypeScript bindings in schemas/*/ts/"
echo "  • JSON schemas in schemas/*_schema.json"
echo "  • Documentation in schemas/*_docs.md"
echo "  • Metadata in schemas/contracts_metadata.json"
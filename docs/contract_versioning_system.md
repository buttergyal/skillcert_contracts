# Contract Versioning System

## Overview

The SkillCert contracts implement a comprehensive versioning system that enables:
- **Version tracking**: Monitor contract deployments and upgrades
- **Data migration**: Safely migrate data between contract versions
- **Compatibility checking**: Ensure data compatibility across versions
- **Migration status**: Track migration progress and status

## Supported Contracts

The versioning system is implemented in the following contracts:
- `user_management`: Manages user data versioning
- `course_registry`: Handles course data versioning  
- `course_access`: Manages access control data versioning

## Versioning Functions

### 1. Get Current Version
```rust
pub fn get_contract_version(_env: Env) -> String
```
Returns the current semantic version of the contract.

**Example:**
```bash
soroban contract invoke --id $CONTRACT_ADDR -- get_contract_version
# Returns: "1.0.0"
```

### 2. Get Version History
```rust
pub fn get_version_history(env: Env) -> Vec<String>
```
Returns a chronological list of all deployed versions.

**Example:**
```bash
soroban contract invoke --id $CONTRACT_ADDR -- get_version_history
# Returns: ["1.0.0", "1.1.0", "2.0.0"]
```

### 3. Check Version Compatibility
```rust
pub fn is_version_compatible(env: Env, from_version: String, to_version: String) -> bool
```
Determines if data can be safely migrated between versions. Compatible versions share the same major version number.

**Example:**
```bash
# Compatible (same major version)
soroban contract invoke --id $CONTRACT_ADDR -- is_version_compatible --from-version "1.0.0" --to-version "1.1.0"
# Returns: true

# Incompatible (different major versions)
soroban contract invoke --id $CONTRACT_ADDR -- is_version_compatible --from-version "1.0.0" --to-version "2.0.0"
# Returns: false
```

### 4. Migrate Data
```rust
pub fn migrate_user_data(env: Env, caller: Address, from_version: String, to_version: String) -> bool
pub fn migrate_course_data(env: Env, caller: Address, from_version: String, to_version: String) -> bool
pub fn migrate_access_data(env: Env, caller: Address, from_version: String, to_version: String) -> bool
```
Performs data migration between contract versions. Requires appropriate authorization.

**Example:**
```bash
soroban contract invoke --id $USER_MGMT_ADDR -- migrate_user_data \
    --caller $ADMIN_ADDRESS \
    --from-version "1.0.0" \
    --to-version "1.1.0"
# Returns: true (successful migration)
```

### 5. Get Migration Status
```rust
pub fn get_migration_status(env: Env) -> String
```
Returns the current migration status and any pending migrations.

**Example:**
```bash
soroban contract invoke --id $CONTRACT_ADDR -- get_migration_status
# Returns: "Migration completed: 1.0.0 -> 1.1.0"
```

## Version Compatibility Rules

### Semantic Versioning
The system follows semantic versioning (SemVer) principles:
- **Major version** (X.0.0): Breaking changes, incompatible data structures
- **Minor version** (1.X.0): New features, backward compatible
- **Patch version** (1.0.X): Bug fixes, fully compatible

### Compatibility Matrix
| From Version | To Version | Compatible | Reason |
|--------------|------------|------------|---------|
| 1.0.0 | 1.1.0 | ✅ Yes | Same major version |
| 1.1.0 | 1.2.0 | ✅ Yes | Same major version |
| 1.0.0 | 2.0.0 | ❌ No | Different major version |
| 2.0.0 | 2.1.0 | ✅ Yes | Same major version |

## Migration Process

### 1. Pre-Migration Checks
- Verify caller authorization (admin privileges)
- Check if source version exists in history
- Validate version compatibility
- Confirm migration hasn't already been completed

### 2. Data Transformation
The migration process includes specific logic for different version combinations:

#### User Management Migrations
- **v1.0.0 → v1.1.0**: Add new optional fields to user profiles
- **v1.1.0 → v2.0.0**: Restructure user data format

#### Course Registry Migrations
- **v1.0.0 → v1.1.0**: Add new course metadata fields (tags, difficulty)
- **v1.1.0 → v2.0.0**: Restructure course data schema

#### Course Access Migrations
- **v1.0.0 → v1.1.0**: Add new access levels and permissions
- **v1.1.0 → v2.0.0**: Restructure access control data

### 3. Post-Migration Actions
- Update version history
- Set migration status
- Emit migration events
- Return success/failure status

## Authorization

### User Management
- Migration requires admin privileges
- Verified through `is_admin()` function

### Course Registry
- Migration requires course creator or admin privileges
- Authorization logic depends on contract implementation

### Course Access
- Migration requires authenticated user (placeholder implementation)
- Can be extended to check admin privileges through user management contract

## Error Handling

The versioning system includes comprehensive error handling:

```rust
#[contracterror]
pub enum VersioningError {
    InvalidVersion = 1,
    VersionNotFound = 2,
    MigrationNotCompatible = 3,
    MigrationAlreadyCompleted = 4,
    UnauthorizedMigration = 5,
    MigrationFailed = 6,
}
```

## Storage Keys

The system uses the following storage keys:
- `version_history`: Stores chronological version list
- `migration_status`: Current migration status information
- `current_migration`: Active migration details

## Best Practices

### 1. Version Planning
- Plan version upgrades carefully
- Ensure backward compatibility when possible
- Document breaking changes thoroughly

### 2. Migration Testing
- Test migrations on development networks first
- Verify data integrity after migration
- Have rollback procedures ready

### 3. Authorization
- Always verify caller permissions
- Use admin-only access for sensitive migrations
- Log all migration attempts

### 4. Monitoring
- Monitor migration status regularly
- Set up alerts for failed migrations
- Track version history for audit purposes

## Usage Examples

See `scripts/versioning_examples.sh` for comprehensive usage examples including:
- Getting current versions
- Checking compatibility
- Performing migrations
- Monitoring status

## Future Enhancements

Potential improvements to the versioning system:
- Automatic migration triggers
- Batch migration support
- Migration rollback functionality
- Enhanced event logging
- Integration with external monitoring systems

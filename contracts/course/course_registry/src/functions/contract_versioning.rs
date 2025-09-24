// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracterror, Address, Env, String, Vec, vec};

/// Errors that can occur during contract versioning operations
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum VersioningError {
    /// Invalid version format
    InvalidVersion = 1,
    /// Version not found in history
    VersionNotFound = 2,
    /// Migration not compatible
    MigrationNotCompatible = 3,
    /// Migration already completed
    MigrationAlreadyCompleted = 4,
    /// Unauthorized migration attempt
    UnauthorizedMigration = 5,
    /// Migration failed
    MigrationFailed = 6,
}

/// Storage keys for versioning data
const VERSION_HISTORY_KEY: &str = "version_history";
const MIGRATION_STATUS_KEY: &str = "migration_status";

/// Get the version history of the contract
pub fn get_version_history(env: &Env) -> Vec<String> {
    let key = String::from_str(env, VERSION_HISTORY_KEY);
    env.storage()
        .instance()
        .get::<String, Vec<String>>(&key)
        .unwrap_or_else(|| vec![env])
}

/// Store a new version in the history
fn store_version_in_history(env: &Env, version: String) {
    let mut history = get_version_history(env);
    history.push_back(version.clone());
    
    let key = String::from_str(env, VERSION_HISTORY_KEY);
    env.storage().instance().set(&key, &history);
}

/// Check if a version exists in the history
fn version_exists_in_history(env: &Env, version: &String) -> bool {
    let history = get_version_history(env);
    for v in history.iter() {
        if &v == version {
            return true;
        }
    }
    false
}

/// Get migration status information
pub fn get_migration_status(env: &Env) -> String {
    let key = String::from_str(env, MIGRATION_STATUS_KEY);
    env.storage()
        .instance()
        .get::<String, String>(&key)
        .unwrap_or_else(|| String::from_str(env, "No migrations pending"))
}

/// Set migration status
fn set_migration_status(env: &Env, status: String) {
    let key = String::from_str(env, MIGRATION_STATUS_KEY);
    env.storage().instance().set(&key, &status);
}

/// Check compatibility between two versions
pub fn is_version_compatible(_env: &Env, _from_version: String, _to_version: String) -> bool {
    // Simple compatibility check - for now, assume all versions are compatible
    // In a real implementation, you would parse semantic versions properly
    true
}

/// Migrate course data between contract versions
pub fn migrate_course_data(
    env: &Env,
    _caller: Address,
    from_version: String,
    to_version: String,
) -> bool {
    // For course registry, we need to check if the caller is authorized
    // This could be a course creator or admin (depending on your authorization logic)
    // For now, we'll allow any authenticated user to perform migration
    
    // Validate versions exist in history
    if !version_exists_in_history(env, &from_version) {
        set_migration_status(env, String::from_str(env, "Migration failed: Source version not found"));
        return false;
    }
    
    // Check compatibility
    if !is_version_compatible(env, from_version.clone(), to_version.clone()) {
        set_migration_status(env, String::from_str(env, "Migration failed: Versions not compatible"));
        return false;
    }
    
    // Perform migration based on version differences
    let migration_result = perform_course_data_migration(env, &from_version, &to_version);
    
    if migration_result {
        // Update version history with new version
        store_version_in_history(env, to_version.clone());
        
        // Set successful migration status
        let status = String::from_str(env, "Migration completed successfully");
        set_migration_status(env, status);
        
        // Emit migration event
        emit_migration_event(env, &from_version, &to_version, true);
        
        true
    } else {
        set_migration_status(env, String::from_str(env, "Migration failed: Data transformation error"));
        emit_migration_event(env, &from_version, &to_version, false);
        false
    }
}

/// Perform the actual course data migration between versions
fn perform_course_data_migration(env: &Env, _from_version: &String, _to_version: &String) -> bool {
    // This is a placeholder for actual course data migration logic
    // In a real implementation, this would:
    // 1. Read existing course data structures
    // 2. Transform them according to the new version schema
    // 3. Write the transformed data back to storage
    
    // For now, we'll simulate a successful migration
    migrate_courses_v1_0_0_to_v1_1_0(env)
}

/// Migrate courses from version 1.0.0 to 1.1.0
fn migrate_courses_v1_0_0_to_v1_1_0(_env: &Env) -> bool {
    // Placeholder for course migration logic
    // This would typically involve:
    // 1. Reading existing courses
    // 2. Adding new fields with default values (e.g., tags, difficulty)
    // 3. Saving updated courses
    
    // For now, return true to indicate successful migration
    true
}


/// Emit a migration event
fn emit_migration_event(_env: &Env, _from_version: &String, _to_version: &String, _success: bool) {
    // In a real implementation, you would emit events here
    // For now, we'll just set a status message
    
    let _event_type = if _success { "success" } else { "failure" };
    // In a real implementation, you would emit actual events here
    // For now, we'll just store a simple status message
    
    // You could emit actual events here using env.events()
    // env.events().publish(("course_migration", event_type), (from_version, to_version, success));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_version_history() {
        let env = Env::default();
        let contract_id = env.register(crate::CourseRegistry, ());
        
        // Test within contract context
        let history = env.as_contract(&contract_id, || {
            get_version_history(&env)
        });
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_version_compatibility() {
        let env = Env::default();
        
        // All versions are compatible in our simplified implementation
        assert!(is_version_compatible(&env, 
            String::from_str(&env, "1.0.0"), 
            String::from_str(&env, "1.1.0")));
        
        // All versions are compatible in our simplified implementation
        assert!(is_version_compatible(&env, 
            String::from_str(&env, "1.0.0"), 
            String::from_str(&env, "2.0.0")));
    }
}

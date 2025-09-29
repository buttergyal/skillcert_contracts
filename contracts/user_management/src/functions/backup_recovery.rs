// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

// Remove unused import - not needed for backup functions
use crate::schema::{AdminConfig, DataKey, LightProfile, UserBackupData, UserProfile};
use soroban_sdk::{Address, Env, Map, String, Vec};

/// Export all user data for backup purposes
///
/// This function creates a complete backup of all user data including profiles,
/// administrative configuration, and system metadata.
///
/// # Arguments
/// * `env` - Soroban environment
/// * `caller` - Address requesting the backup (must be admin)
///
/// # Returns
/// * `UserBackupData` - Complete backup structure
///
/// # Panics
/// * If caller is not an admin
/// * If system is not initialized
pub fn export_user_data(env: Env, caller: Address) -> UserBackupData {
    caller.require_auth();

    // Verify caller is admin
    if !crate::functions::is_admin::is_admin(env.clone(), caller) {
        panic!("Unauthorized: Only admins can export user data");
    }

    // Verify system is initialized
    if !crate::functions::admin_management::is_system_initialized(env.clone()) {
        panic!("System not initialized");
    }

    // Get all user addresses
    let users_index: Vec<Address> = env
        .storage()
        .persistent()
        .get(&DataKey::UsersIndex)
        .unwrap_or(Vec::new(&env));

    // Initialize maps for backup data
    let mut user_profiles = Map::new(&env);
    let mut light_profiles = Map::new(&env);
    let mut email_mappings = Map::new(&env);

    // Collect all user data
    for user_address in users_index.iter() {
        // Get full user profile
        if let Some(profile) = env
            .storage()
            .persistent()
            .get::<DataKey, UserProfile>(&DataKey::UserProfile(user_address.clone()))
        {
            user_profiles.set(user_address.clone(), profile.clone());
            
            // Map email to address
            email_mappings.set(profile.contact_email.clone(), user_address.clone());
        }

        // Get light profile
        if let Some(light_profile) = env
            .storage()
            .persistent()
            .get::<DataKey, LightProfile>(&DataKey::UserProfileLight(user_address.clone()))
        {
            light_profiles.set(user_address.clone(), light_profile);
        }
    }

    // Get admin configuration
    let admin_config: AdminConfig = env
        .storage()
        .persistent()
        .get(&DataKey::AdminConfig)
        .unwrap_or_else(|| panic!("Admin config not found"));

    // Get admin list
    let admins: Vec<Address> = env
        .storage()
        .persistent()
        .get(&DataKey::Admins)
        .unwrap_or(Vec::new(&env));

    // Create backup data structure
    UserBackupData {
        user_profiles,
        light_profiles,
        email_mappings,
        users_index,
        admin_config,
        admins,
        backup_timestamp: env.ledger().timestamp(),
        backup_version: String::from_str(&env, "1.0.0"),
    }
}

/// Import user data from backup
///
/// This function restores user data from a backup structure.
/// This operation will overwrite existing data.
///
/// # Arguments
/// * `env` - Soroban environment
/// * `caller` - Address performing the import (must be admin)
/// * `backup_data` - Backup data to restore
///
/// # Returns
/// * `u32` - Number of users imported
///
/// # Panics
/// * If caller is not an admin
/// * If backup data is invalid
pub fn import_user_data(env: Env, caller: Address, backup_data: UserBackupData) -> u32 {
    caller.require_auth();

    // Verify caller is admin
    if !crate::functions::is_admin::is_admin(env.clone(), caller) {
        panic!("Unauthorized: Only admins can import user data");
    }

    // Validate backup version compatibility
    let expected_version = String::from_str(&env, "1.0.0");
    if backup_data.backup_version != expected_version {
        panic!("Incompatible backup version");
    }

    let mut imported_count = 0u32;

    // Import user profiles
    for (address, profile) in backup_data.user_profiles.iter() {
        env.storage()
            .persistent()
            .set(&DataKey::UserProfile(address.clone()), &profile);
        imported_count += 1;
    }

    // Import light profiles
    for (address, light_profile) in backup_data.light_profiles.iter() {
        env.storage()
            .persistent()
            .set(&DataKey::UserProfileLight(address.clone()), &light_profile);
    }

    // Import email mappings
    for (email, address) in backup_data.email_mappings.iter() {
        env.storage()
            .persistent()
            .set(&DataKey::EmailIndex(email.clone()), &address);
    }

    // Import users index
    env.storage()
        .persistent()
        .set(&DataKey::UsersIndex, &backup_data.users_index);

    // Import admin configuration
    env.storage()
        .persistent()
        .set(&DataKey::AdminConfig, &backup_data.admin_config);

    // Import admin list
    env.storage()
        .persistent()
        .set(&DataKey::Admins, &backup_data.admins);

    // Set individual admin flags
    for admin in backup_data.admins.iter() {
        env.storage()
            .persistent()
            .set(&DataKey::Admin(admin.clone()), &true);
    }

    // Emit import event
    env.events().publish(
        (String::from_str(&env, "user_data_imported"),),
        (imported_count, backup_data.backup_timestamp),
    );

    imported_count
}

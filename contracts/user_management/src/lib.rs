// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

/// Contract version for tracking deployments and upgrades
pub const VERSION: &str = "1.0.0";

pub mod error;
pub mod functions;
pub mod models;
pub mod schema;

use error::Error;
use schema::UserProfile;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

/// User Management Contract
///
/// This contract handles user registration, profile management, authentication,
/// and administrative functions for the SkillCert platform. It manages user roles,
/// permissions, and provides comprehensive user lifecycle management.
#[contract]
pub struct UserManagement;

#[contractimpl]
impl UserManagement {
    /// Retrieve a user profile for the authenticated user.
    ///
    /// This function fetches the complete user profile associated with the provided
    /// blockchain address. The user must be authenticated; otherwise, the function
    /// will panic.
    ///
    /// ### Arguments
    ///
    /// * `env` - The Soroban environment.
    /// * `user` - The address of the user whose profile is being requested.
    ///
    /// ### Returns
    ///
    /// Returns the `UserProfile` corresponding to the authenticated user.
    ///
    /// ### Panics
    ///
    /// * If the user is not authenticated (`require_auth` fails).
    /// * If the user profile does not exist (`UserNotFound` error).
    ///
    /// ### Examples
    ///
    /// ```rust
    /// // Assuming the user is authenticated in the environment
    /// let profile = contract.get_user_profile(env.clone(), my_address);
    /// println!("User full name: {}", profile.full_name);
    /// ```
    ///
    /// ### Notes
    ///
    /// * Only the user themselves can fetch their profile; there is no admin override
    ///   in this function.
    /// * If the profile is not found in storage, the function will panic with
    ///   `UserNotFound`.
    pub fn get_user_profile(env: Env, user: Address) -> Result<UserProfile, Error> {
        functions::user::get_user_profile(env, user)
    }

    /// Create a new user profile
    ///
    /// Creates a new user profile using a UserProfile struct.
    /// Validates mandatory fields (full_name and contact_email) and saves the profile.
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `user` - Address of the user whose profile is being created
    /// * `profile` - UserProfile struct containing all profile data
    ///
    /// # Returns
    /// * `UserProfile` - The created user profile
    ///
    /// # Panics
    /// * If mandatory fields (full_name, contact_email) are missing
    /// * If user profile already exists
    /// * If email format is invalid
    /// * If validation rules are violated
    ///
    /// # Events
    /// Emits a user creation event upon successful creation
    ///
    /// # Examples
    ///
    /// ```rust
    /// let profile = UserProfile {
    ///     full_name: "John Doe".try_into().unwrap(),
    ///     contact_email: "john@example.com".try_into().unwrap(),
    ///     role: UserRole::Student,
    ///     status: UserStatus::Active,
    ///     country: Some("US".try_into().unwrap()),
    ///     ..Default::default()
    /// };
    ///
    /// let created_profile = contract.create_user_profile(env, user_address, profile);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Duplicate profile**: Will panic if user already has a profile
    /// * **Empty required fields**: Will panic if full_name or contact_email are empty
    /// * **Invalid email**: Will panic if email format is not valid
    pub fn create_user_profile(env: Env, user: Address, profile: UserProfile) -> UserProfile {
        functions::user::create_user_profile(env, user, profile)
    }

    /// Get the current contract version
    ///
    /// Returns the semantic version of the current contract deployment.
    /// This is useful for tracking contract upgrades and compatibility.
    ///
    /// # Arguments
    /// * `_env` - The Soroban environment (unused)
    ///
    /// # Returns
    /// * `String` - The current contract version
    pub fn get_contract_version(_env: Env) -> String {
        String::from_str(&_env, VERSION)
    }

    /// Export all user data for backup purposes (admin only)
    ///
    /// This function exports all user profiles and administrative data
    /// for backup and recovery purposes. Only admins can perform this operation.
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the export (must be admin)
    ///
    /// # Returns
    /// * `UserBackupData` - Complete backup data structure
    ///
    /// # Panics
    /// * If caller is not an admin
    /// * If system is not initialized
    pub fn export_user_data(env: Env, caller: Address) -> crate::schema::UserBackupData {
        functions::backup_recovery::export_user_data(env, caller)
    }

    /// Import user data from backup (admin only)
    ///
    /// This function imports user data from a backup structure.
    /// Only admins can perform this operation. This will overwrite existing data.
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the import (must be admin)
    /// * `backup_data` - Backup data structure to import
    ///
    /// # Returns
    /// * `u32` - Number of users imported
    ///
    /// # Panics
    /// * If caller is not an admin
    /// * If backup data is invalid
    /// * If import operation fails
    pub fn import_user_data(env: Env, caller: Address, backup_data: crate::schema::UserBackupData) -> u32 {
        functions::backup_recovery::import_user_data(env, caller, backup_data)
    }

    /// Initialize the user management system
    pub fn initialize_system(env: Env, initializer: Address, super_admin: Address, max_page_size: Option<u32>) {
        functions::admin_management::initialize_system(env, initializer, super_admin, max_page_size);
    }

    /// Check if the system is initialized
    pub fn is_system_initialized(env: Env) -> bool {
        functions::admin_management::is_system_initialized(env)
    }

    /// Add an admin to the system
    pub fn add_admin(env: Env, caller: Address, new_admin: Address) {
        functions::admin_management::add_admin(env, caller, new_admin)
    }

    /// Remove an admin from the system
    pub fn remove_admin(env: Env, caller: Address, admin_to_remove: Address) {
        functions::admin_management::remove_admin(env, caller, admin_to_remove)
    }

    /// Get list of all admins
    pub fn get_admins(env: Env, caller: Address) -> Vec<Address> {
        functions::admin_management::get_admins(env, caller)
    }

    /// Delete a user from the system
    pub fn delete_user(env: Env, caller: Address, user_to_delete: Address) {
        functions::delete_user::delete_user(env, caller, user_to_delete)
    }

    /// Edit user profile
    pub fn edit_user_profile(env: Env, caller: Address, user: Address, updates: schema::ProfileUpdateParams) -> UserProfile {
        functions::edit_user_profile::edit_user_profile(env, caller, user, updates)
    }
}

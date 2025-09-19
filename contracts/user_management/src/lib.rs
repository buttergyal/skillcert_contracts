// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

/// Contract version for tracking deployments and upgrades
pub const VERSION: &str = "1.0.0";

pub mod error;
pub mod functions;
pub mod schema;

#[cfg(test)]
mod test;

use crate::schema::{AdminConfig, LightProfile, ProfileUpdateParams, UserProfile, UserRole, UserStatus};
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


    /// Retrieve a user profile by their address.
    ///
    /// This function fetches a complete user profile using the user's blockchain address.
    /// Access may be restricted based on the requester's permissions.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `requester` - The address of the user requesting the profile
    /// * `user_id` - The address of the user whose profile is being requested
    ///
    /// # Returns
    ///
    /// Returns the requested `UserProfile`.
    pub fn get_user_by_id(env: Env, requester: Address, user_id: Address) -> UserProfile {
        functions::get_user_by_id::get_user_by_id(env, requester, user_id)
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
    /// # Events
    /// Emits a user creation event upon successful creation
    pub fn create_user_profile(env: Env, user: Address, profile: UserProfile) -> UserProfile {
        functions::create_user_profile::create_user_profile(env, user, profile)
    }

    /// Edit an existing user profile
    ///
    /// Updates an existing user profile with new values for allowed fields.
    /// Only the user themselves or administrators can perform updates.
    /// Email and role fields cannot be updated through this function.
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address of the user performing the update
    /// * `user_id` - Address of the user whose profile is being updated
    /// * `updates` - ProfileUpdateParams containing fields to update
    ///
    /// # Returns
    /// * `UserProfile` - The updated user profile
    ///
    /// # Panics
    /// * If caller authentication fails
    /// * If user profile doesn't exist
    /// * If caller lacks permission to edit
    /// * If any field validation fails
    /// * If user is inactive
    ///
    /// # Events
    /// Emits a user update event upon successful profile update
    pub fn edit_user_profile(
        env: Env,
        caller: Address,
        user_id: Address,
        updates: ProfileUpdateParams,
    ) -> UserProfile {
        functions::edit_user_profile::edit_user_profile(env, caller, user_id, updates)
    }

    /// Check if an address has admin privileges.
    ///
    /// This function is used by other contracts to verify admin status
    /// for cross-contract authorization checks.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `who` - The address to check for admin privileges
    ///
    /// # Returns
    ///
    /// Returns `true` if the address has admin privileges, `false` otherwise.
    pub fn is_admin(env: Env, who: Address) -> bool {
        functions::is_admin::is_admin(env, who)
    }

    /// Delete (deactivate) a user account
    ///
    /// Performs a soft delete by marking the user as inactive instead of permanent deletion.
    /// Only admins or the user themselves can trigger deletion.
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the deletion (must be admin or the user themselves)
    /// * `user_id` - Address of the user to be deactivated
    ///
    /// # Panics
    /// * If caller authentication fails
    /// * If user doesn't exist
    /// * If caller is neither admin nor the user themselves
    /// * If user is already inactive
    ///
    /// # Events
    /// Emits a user deactivation event upon successful deletion
    pub fn delete_user(env: Env, caller: Address, user_id: Address) {
        functions::delete_user::delete_user(env, caller, user_id)
    }

    /// Lists all registered users with pagination and filtering (admin-only)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must be admin)
    /// * `page` - Zero-based page index
    /// * `page_size` - Number of items per page (must be > 0)
    /// * `role_filter` - Optional role filter
    /// * `country_filter` - Optional country filter
    /// * `status_filter` - Optional status filter
    ///
    /// # Returns
    /// * `Vec<LightProfile>` - Filtered and paginated lightweight user profiles
    pub fn list_all_users(
        env: Env,
        caller: Address,
        page: u32,
        page_size: u32,
        role_filter: Option<UserRole>,
        country_filter: Option<String>,
        status_filter: Option<UserStatus>,
    ) -> Vec<LightProfile> {
        functions::list_all_registered_users::list_all_users(
            env,
            caller,
            page,
            page_size,
            role_filter,
            country_filter,
            status_filter,
        )
    }

    /// Initialize the admin system (one-time only)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `initializer` - Address performing the initialization
    /// * `super_admin` - Address that will become the super admin
    /// * `max_page_size` - Optional maximum page size (default: 100, max: 1000)
    ///
    /// # Returns
    /// * `AdminConfig` - The created admin configuration
    pub fn initialize_system(
        env: Env,
        initializer: Address,
        super_admin: Address,
        max_page_size: Option<u32>,
    ) -> AdminConfig {
        functions::admin_management::initialize_system(env, initializer, super_admin, max_page_size)
    }

    /// Add a new admin (super admin only)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must be super admin)
    /// * `new_admin` - Address to be added as admin
    pub fn add_admin(env: Env, caller: Address, new_admin: Address) {
        functions::admin_management::add_admin(env, caller, new_admin)
    }

    /// Remove an admin (super admin only)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must be super admin)
    /// * `admin_to_remove` - Address to be removed from admins
    pub fn remove_admin(env: Env, caller: Address, admin_to_remove: Address) {
        functions::admin_management::remove_admin(env, caller, admin_to_remove)
    }

    /// Get list of all admins (admin only)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must be admin)
    ///
    /// # Returns
    /// * `Vec<Address>` - List of all admin addresses including super admin
    pub fn get_admins(env: Env, caller: Address) -> Vec<Address> {
        functions::admin_management::get_admins(env, caller)
    }

    /// Check if the system is initialized
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    ///
    /// # Returns
    /// * `bool` - True if system is initialized
    pub fn is_system_initialized(env: Env) -> bool {
        functions::admin_management::is_system_initialized(env)
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

pub mod functions;
pub mod schema;
pub mod error;

#[cfg(test)]
mod test;

use crate::schema::{AdminConfig, LightProfile, UserProfile, UserRole, UserStatus};
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

#[contract]
pub struct UserManagement;

#[contractimpl]
impl UserManagement {
    pub fn save_profile(
        env: Env,
        name: String,
        lastname: String,
        email: String,
        password: String,
        confirm_password: String,
        specialization: String,
        languages: Vec<String>,
        teaching_categories: Vec<String>,
        user: Address,
    ) -> UserProfile {
        functions::save_profile::user_management_save_profile(
            env, user, name, lastname, email, password, confirm_password, specialization, languages, teaching_categories,
        )
    }

    // Aquí agregamos la nueva función get_user_by_id
    pub fn get_user_by_id(env: Env, requester: Address, user_id: Address) -> UserProfile {
        functions::get_user_by_id::get_user_by_id(env, requester, user_id)
    }

    /// Create a new user profile
    ///
    /// Creates a new user profile with the provided information.
    /// Validates required fields, ensures email uniqueness, and assigns default values.
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `creator` - Address creating the profile (usually an admin or the user themselves)
    /// * `user_address` - Address of the user whose profile is being created
    /// * `name` - User's full name (required)
    /// * `email` - User's email address (required, must be unique)
    /// * `role` - User's role in the system (required)
    /// * `country` - User's country (required)
    /// * `profession` - User's profession (optional)
    /// * `goals` - User's goals or bio (optional)
    /// * `profile_picture` - URL to profile picture (optional)
    /// * `language` - User's preferred language (optional, defaults to "en")
    ///
    /// # Returns
    /// * `UserProfile` - The created user profile
    ///
    /// # Events
    /// Emits a user creation event upon successful creation
    pub fn create_user_profile(
        env: Env,
        creator: Address,
        user_address: Address,
        name: String,
        email: String,
        role: UserRole,
        country: String,
        profession: Option<String>,
        goals: Option<String>,
        profile_picture: Option<String>,
        language: Option<String>,
    ) -> UserProfile {
        functions::create_user_profile::create_user_profile(
            env,
            creator,
            user_address,
            name,
            email,
            role,
            country,
            profession,
            goals,
            profile_picture,
            language,
        )
    }

    /// Public admin check for cross-contract calls
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
        functions::admin_management::is_initialized(env)
    }
}

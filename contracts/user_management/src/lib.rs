#![cfg_attr(target_arch = "wasm32", no_std)]

pub mod functions;
pub mod schema;

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
        email: String,
        profession: Option<String>,
        goals: Option<String>,
        country: String,
        user: Address,
    ) -> UserProfile {
        functions::save_profile::user_management_save_profile(
            env, user, name, email, profession, goals, country,
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

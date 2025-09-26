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

use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};
use crate::schema::{AdminConfig, LightProfile, PaginatedLightProfiles, PaginationParams, Permission, ProfileUpdateParams, UserProfile, UserRole, UserStatus};

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
    ///
    /// # Panics
    ///
    /// * If the user profile doesn't exist
    /// * If the requester doesn't have permission to view the profile
    /// * If the requester is not the user themselves or an admin
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Get your own profile
    /// let my_profile = contract.get_user_by_id(env.clone(), my_address, my_address);
    /// 
    /// // Admin getting any user's profile
    /// let user_profile = contract.get_user_by_id(env.clone(), admin_address, user_address);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Non-existent user**: Will panic with appropriate error message
    /// * **Inactive user**: Returns profile but status will be `UserStatus::Inactive`
    /// * **Permission denied**: Non-admin users can only view their own profiles
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// let updates = ProfileUpdateParams {
    ///     full_name: Some("Jane Doe".try_into().unwrap()),
    ///     country: Some("CA".try_into().unwrap()),
    ///     bio: Some("Updated bio".try_into().unwrap()),
    ///     ..Default::default()
    /// };
    /// 
    /// let updated_profile = contract.edit_user_profile(env, caller_address, user_address, updates);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Partial updates**: Only provided fields are updated, others remain unchanged
    /// * **Admin override**: Admins can edit any user's profile except email/role
    /// * **Inactive user**: Cannot edit profiles of inactive users
    /// * **Invalid updates**: Empty strings or invalid data will cause panic
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Check if user is admin
    /// let is_admin = contract.is_admin(env.clone(), user_address);
    /// if is_admin {
    ///     // Perform admin operations
    /// }
    /// 
    /// // Cross-contract admin check
    /// let can_perform_action = contract.is_admin(env.clone(), caller_address);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **System not initialized**: Returns `false` if admin system hasn't been set up
    /// * **Non-existent user**: Returns `false` for addresses that don't exist
    /// * **Regular users**: Always returns `false` for non-admin users
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// // User deleting their own account
    /// contract.delete_user(env.clone(), user_address, user_address);
    /// 
    /// // Admin deleting another user's account
    /// contract.delete_user(env.clone(), admin_address, user_to_delete);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Already inactive**: Will panic if trying to delete an already inactive user
    /// * **Permission denied**: Non-admin users can only delete their own accounts
    /// * **Data preservation**: User data is preserved but marked as inactive
    /// * **Irreversible**: Once deactivated, user cannot be reactivated through this contract
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
    ///
    /// # Panics
    /// * If caller is not an admin
    /// * If page_size is 0 or exceeds maximum allowed
    /// * If system is not initialized
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Get first page with 10 users
    /// let users = contract.list_all_users(
    ///     env.clone(),
    ///     admin_address,
    ///     0,  // page 0
    ///     10, // page size
    ///     None, None, None // no filters
    /// );
    /// 
    /// // Filter by role and country
    /// let students = contract.list_all_users(
    ///     env.clone(),
    ///     admin_address,
    ///     0, 20,
    ///     Some(UserRole::Student),
    ///     Some("US".try_into().unwrap()),
    ///     None
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Empty results**: Returns empty vector if no users match filters
    /// * **Large page sizes**: Limited by system configuration (max 1000)
    /// * **Invalid page**: Returns empty vector for non-existent pages
    /// * **Multiple filters**: All filters are applied with AND logic
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

    /// Lists all registered users with cursor-based pagination and filtering (admin-only)
    ///
    /// This function implements efficient cursor-based pagination to avoid gas limit issues
    /// when dealing with large datasets. It returns a PaginatedResult with metadata for
    /// efficient navigation.
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must be admin)
    /// * `pagination` - Pagination parameters including cursor and limit
    /// * `role_filter` - Optional filter for user role
    /// * `status_filter` - Optional filter for user status
    ///
    /// # Returns
    /// * `PaginatedLightProfiles` - Paginated results with navigation metadata
    pub fn list_all_users_cursor(
        env: Env,
        caller: Address,
        pagination: PaginationParams,
        role_filter: Option<UserRole>,
        status_filter: Option<UserStatus>,
    ) -> PaginatedLightProfiles {
        functions::list_all_registered_users::list_all_users_cursor(
            env,
            caller,
            pagination,
            role_filter,
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
    ///
    /// # Panics
    /// * If system has already been initialized
    /// * If max_page_size exceeds 1000
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Initialize with default settings
    /// let config = contract.initialize_system(
    ///     env.clone(),
    ///     deployer_address,
    ///     super_admin_address,
    ///     None
    /// );
    /// 
    /// // Initialize with custom page size
    /// let config = contract.initialize_system(
    ///     env.clone(),
    ///     deployer_address,
    ///     super_admin_address,
    ///     Some(500)
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Double initialization**: Will panic if called more than once
    /// * **Invalid page size**: Will panic if max_page_size > 1000
    /// * **Super admin privileges**: Super admin cannot be removed after initialization
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
    ///
    /// # Panics
    /// * If caller is not the super admin
    /// * If system is not initialized
    /// * If new_admin is already an admin
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Super admin adding a new admin
    /// contract.add_admin(env.clone(), super_admin_address, new_admin_address);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Already admin**: Will panic if trying to add an existing admin
    /// * **Self-promotion**: Super admin cannot add themselves (redundant)
    /// * **Non-existent user**: Can add admin privileges to any address
    pub fn add_admin(env: Env, caller: Address, new_admin: Address) {
        functions::admin_management::add_admin(env, caller, new_admin)
    }

    /// Remove an admin (super admin only)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must be super admin)
    /// * `admin_to_remove` - Address to be removed from admins
    ///
    /// # Panics
    /// * If caller is not the super admin
    /// * If system is not initialized
    /// * If admin_to_remove is not an admin
    /// * If trying to remove the super admin
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Super admin removing another admin
    /// contract.remove_admin(env.clone(), super_admin_address, admin_to_remove);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Super admin protection**: Cannot remove the super admin
    /// * **Non-admin**: Will panic if trying to remove a non-admin address
    /// * **Self-removal**: Super admin cannot remove themselves
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
    ///
    /// # Panics
    /// * If caller is not an admin
    /// * If system is not initialized
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Get all admin addresses
    /// let admins = contract.get_admins(env.clone(), admin_address);
    /// for admin in admins {
    ///     // Process each admin address
    /// }
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Empty list**: Returns vector with only super admin if no other admins exist
    /// * **Admin only**: Only admins can view the admin list
    /// * **Order**: Super admin is typically first in the list
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Check if admin system is ready
    /// let is_initialized = contract.is_system_initialized(env.clone());
    /// if !is_initialized {
    ///     // Initialize the system first
    ///     contract.initialize_system(env, deployer, super_admin, None);
    /// }
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Fresh deployment**: Returns `false` for newly deployed contracts
    /// * **Public access**: Anyone can check initialization status
    /// * **One-time check**: Once initialized, always returns `true`
    pub fn is_system_initialized(env: Env) -> bool {
        functions::admin_management::is_system_initialized(env)
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

    /// Get contract version history
    ///
    /// Returns a list of all versions that have been deployed for this contract.
    /// This helps track the evolution of the contract over time.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    ///
    /// # Returns
    /// * `Vec<String>` - Vector of version strings in chronological order
    pub fn get_version_history(env: Env) -> Vec<String> {
        functions::contract_versioning::get_version_history(&env)
    }

    /// Check compatibility between contract versions
    ///
    /// Determines if data from one version can be safely used with another version.
    /// This is crucial for migration processes and backward compatibility.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `from_version` - The source version to check compatibility from
    /// * `to_version` - The target version to check compatibility to
    ///
    /// # Returns
    /// * `bool` - True if the versions are compatible, false otherwise
    pub fn is_version_compatible(env: Env, from_version: String, to_version: String) -> bool {
        functions::contract_versioning::is_version_compatible(&env, from_version, to_version)
    }

    /// Migrate user data between contract versions
    ///
    /// Performs data migration from one contract version to another.
    /// This function handles the transformation of user data structures
    /// when upgrading contract versions.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `caller` - The address performing the migration (must be admin)
    /// * `from_version` - The source version to migrate from
    /// * `to_version` - The target version to migrate to
    ///
    /// # Returns
    /// * `bool` - True if migration was successful, false otherwise
    ///
    /// # Events
    /// Emits a migration event upon successful completion
    pub fn migrate_user_data(env: Env, caller: Address, from_version: String, to_version: String) -> bool {
        functions::contract_versioning::migrate_user_data(&env, caller, from_version, to_version)
    }

    /// Get migration status for the current contract
    ///
    /// Returns information about the current migration status and any
    /// pending migrations that need to be completed.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    ///
    /// # Returns
    /// * `String` - Migration status information
    pub fn get_migration_status(env: Env) -> String {
        functions::contract_versioning::get_migration_status(&env)
    }

    // RBAC Functions

    /// Set a user's role (admin only)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must have ManageAdmins permission)
    /// * `user` - Address of the user to assign role to
    /// * `role` - New role to assign
    pub fn set_user_role(env: Env, caller: Address, user: Address, role: UserRole) {
        functions::rbac::set_user_role(env, caller, user, role)
    }

    /// Check if a user has a specific permission
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `user` - Address of the user to check
    /// * `permission` - Permission to check for
    ///
    /// # Returns
    /// * `bool` - True if user has the permission
    pub fn has_permission(env: Env, user: Address, permission: Permission) -> bool {
        functions::rbac::has_permission(&env, &user, &permission)
    }

    /// Grant additional permission to a user (admin only)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must have ManageAdmins permission)
    /// * `user` - Address of the user to grant permission to
    /// * `permission` - Permission to grant
    pub fn grant_user_permission(env: Env, caller: Address, user: Address, permission: Permission) {
        functions::rbac::grant_user_permission(env, caller, user, permission)
    }

    /// Revoke permission from a user (admin only)
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must have ManageAdmins permission)
    /// * `user` - Address of the user to revoke permission from
    /// * `permission` - Permission to revoke
    pub fn revoke_user_permission(env: Env, caller: Address, user: Address, permission: Permission) {
        functions::rbac::revoke_user_permission(env, caller, user, permission)
    }

    /// Get all permissions for a user
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `caller` - Address performing the call (must be user themselves or have ViewUsers permission)
    /// * `user` - Address of the user to get permissions for
    ///
    /// # Returns
    /// * `Vec<Permission>` - List of all permissions the user has
    pub fn get_user_permissions(env: Env, caller: Address, user: Address) -> Vec<Permission> {
        functions::rbac::get_user_permissions(env, caller, user)
    }
}

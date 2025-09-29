// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

/// Contract version for tracking deployments and upgrades
pub const VERSION: &str = "1.0.0";

mod error;
mod functions;
mod schema;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

pub use error::Error;
pub use functions::*;
pub use schema::{CourseUsers, UserCourses};

/// Course Access Contract
///
/// This contract manages user access to courses in the SkillCert platform.
/// It provides functionality to grant, revoke, and query course access permissions,
/// as well as manage user profiles.
#[contract]
pub struct CourseAccessContract;

#[contractimpl]
impl CourseAccessContract {
    /// One-time constructor to set owner and config addresses.
    ///
    /// Initializes the contract with the necessary external contract addresses.
    /// This function can only be called once during contract deployment.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `caller` - The address of the contract deployer/owner
    /// * `user_mgmt_addr` - Address of the user management contract
    /// * `course_registry_addr` - Address of the course registry contract
    ///
    /// # Panics
    ///
    /// * Fails if the contract has already been initialized
    /// * If any of the provided addresses are invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Initialize contract during deployment
    /// contract.initialize(
    ///     env.clone(),
    ///     deployer_address,
    ///     user_mgmt_contract_address,
    ///     course_registry_contract_address
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Double initialization**: Will panic if called more than once
    /// * **Invalid addresses**: Contract addresses must be valid
    /// * **Deployment only**: Should only be called during contract deployment
    pub fn initialize(
        env: Env,
        caller: Address,
        user_mgmt_addr: Address,
        course_registry_addr: Address,
    ) {
        functions::config::initialize(env, caller, user_mgmt_addr, course_registry_addr)
    }

    /// Grant access to a specific user for a given course.
    ///
    /// Allows a user to access a specific course. Only authorized users
    /// (course creators or admins) can grant access.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `course_id` - The unique identifier of the course
    /// * `user` - The address of the user to grant access to
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If caller is not authorized (not course creator or admin)
    /// * If user already has access
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Course creator granting access
    /// contract.grant_access(
    ///     env.clone(),
    ///     "course_123".try_into().unwrap(),
    ///     student_address
    /// );
    /// 
    /// // Admin granting access
    /// contract.grant_access(
    ///     env.clone(),
    ///     "course_456".try_into().unwrap(),
    ///     student_address
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Already has access**: Will panic if user already has access
    /// * **Non-existent course**: Will panic if course doesn't exist
    /// * **Permission denied**: Only course creators and admins can grant access
    /// * **User validation**: User address must be valid
    pub fn grant_access(env: Env, course_id: String, user: Address) {
        functions::grant_access::course_access_grant_access(env, course_id, user)
    }

    /// Revoke access for a specific user from a course.
    ///
    /// Removes a user's access to a specific course. Only authorized users
    /// (course creators or admins) can revoke access.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `course_id` - The unique identifier of the course
    /// * `user` - The address of the user to revoke access from
    ///
    /// # Returns
    ///
    /// Returns `true` if access was successfully revoked, `false` otherwise.
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If caller is not authorized (not course creator or admin)
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Revoke access from a user
    /// let success = contract.revoke_access(
    ///     env.clone(),
    ///     "course_123".try_into().unwrap(),
    ///     student_address
    /// );
    /// 
    /// if success {
    ///     println!("Access revoked successfully");
    /// } else {
    ///     println!("User didn't have access");
    /// }
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **No access to revoke**: Returns `false` if user didn't have access
    /// * **Non-existent course**: Will panic if course doesn't exist
    /// * **Permission denied**: Only course creators and admins can revoke access
    /// * **Idempotent**: Safe to call multiple times
    pub fn revoke_access(env: Env, course_id: String, user: Address) -> bool {
        functions::revoke_access::course_access_revoke_access(env, course_id, user)
    }

    /// Save or update a user's profile on-chain.
    ///
    /// Stores user profile information in the contract storage.
    /// This includes personal and professional information.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `name` - The user's full name
    /// * `email` - The user's email address
    /// * `profession` - Optional profession/job title
    /// * `goals` - Optional learning goals or objectives
    /// * `country` - The user's country of residence
    ///
    /// # Panics
    ///
    /// * If name, email, or country are empty
    /// * If email format is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Save user profile
    /// contract.save_user_profile(
    ///     env.clone(),
    ///     "John Doe".try_into().unwrap(),
    ///     "john@example.com".try_into().unwrap(),
    ///     Some("Software Developer".try_into().unwrap()),
    ///     Some("Learn Rust programming".try_into().unwrap()),
    ///     "US".try_into().unwrap()
    /// );
    /// 
    /// // Save minimal profile
    /// contract.save_user_profile(
    ///     env.clone(),
    ///     "Jane Smith".try_into().unwrap(),
    ///     "jane@example.com".try_into().unwrap(),
    ///     None,
    ///     None,
    ///     "CA".try_into().unwrap()
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Empty required fields**: Name, email, and country cannot be empty
    /// * **Invalid email**: Email must be in valid format
    /// * **Profile updates**: Overwrites existing profile data
    /// * **Optional fields**: Profession and goals can be None
    pub fn save_user_profile(
        env: Env,
        name: String,
        email: String,
        profession: Option<String>,
        goals: Option<String>,
        country: String,
    ) {
        let user: Address = env.current_contract_address();
        functions::save_profile::save_user_profile(env, name, email, profession, goals, country, user);
    }

    /// List all courses a user has access to.
    ///
    /// Retrieves all courses that the specified user is enrolled in
    /// or has been granted access to.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `user` - The address of the user to query
    ///
    /// # Returns
    ///
    /// Returns a `UserCourses` struct containing the list of accessible courses.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Get user's accessible courses
    /// let user_courses = contract.list_user_courses(env.clone(), user_address);
    /// 
    /// for course_id in user_courses.course_ids {
    ///     println!("User has access to course: {}", course_id);
    /// }
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **No access**: Returns empty list if user has no course access
    /// * **Non-existent user**: Returns empty list for non-existent users
    /// * **Public access**: Anyone can query user courses
    /// * **Revoked courses**: Only includes currently accessible courses
    pub fn list_user_courses(env: Env, user: Address) -> UserCourses {
        functions::list_user_courses::list_user_courses(env, user)
    }

    /// List all users who have access to a course.
    ///
    /// Retrieves all users who have been granted access to the specified course.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `course_id` - The unique identifier of the course
    ///
    /// # Returns
    ///
    /// Returns a `CourseUsers` struct containing the list of users with access.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Get all users with access to a course
    /// let course_users = contract.list_course_access(env.clone(), "course_123".try_into().unwrap());
    /// 
    /// for user in course_users.users {
    ///     println!("User with access: {}", user);
    /// }
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **No users**: Returns empty list if no users have access
    /// * **Non-existent course**: Returns empty list for non-existent courses
    /// * **Public access**: Anyone can query course access
    /// * **Real-time data**: Always returns current access status
    pub fn list_course_access(env: Env, course_id: String) -> CourseUsers {
        functions::list_course_access::course_access_list_course_access(env, course_id)
    }

    /// Revoke all user access for a course.
    ///
    /// Removes access for all users from the specified course.
    /// Only admin or course creator is allowed to perform this operation.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `user` - The address of the user requesting the operation
    /// * `course_id` - The unique identifier of the course
    ///
    /// # Returns
    ///
    /// Returns the number of users affected by the revocation and emits an event.
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If caller is not authorized (not course creator or admin)
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Revoke all access for a course
    /// let affected_users = contract.revoke_all_access(
    ///     env.clone(),
    ///     admin_address,
    ///     "course_123".try_into().unwrap()
    /// );
    /// 
    /// println!("Revoked access for {} users", affected_users);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **No users**: Returns 0 if no users had access
    /// * **Non-existent course**: Will panic if course doesn't exist
    /// * **Permission denied**: Only course creators and admins can perform this
    /// * **Bulk operation**: Efficiently removes all access in one transaction
    pub fn revoke_all_access(env: Env, user: Address, course_id: String) -> u32 {
        functions::revoke_all_access::revoke_all_access(env, user, course_id)
    }

    /// Configure external contract addresses used for auth checks.
    ///
    /// Updates the addresses of external contracts that this contract
    /// depends on for authentication and authorization checks.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `caller` - The address of the user making the configuration change
    /// * `user_mgmt_addr` - Address of the user management contract
    /// * `course_registry_addr` - Address of the course registry contract
    ///
    /// # Panics
    ///
    /// * If caller is not the contract owner
    /// * If any of the provided addresses are invalid
    ///
    /// # Storage
    ///
    /// Stores the addresses in instance storage keys: ("user_mgmt_addr",) and ("course_registry_addr",)
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Update contract addresses
    /// contract.set_config(
    ///     env.clone(),
    ///     contract_owner_address,
    ///     new_user_mgmt_address,
    ///     new_course_registry_address
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Owner only**: Only contract owner can update addresses
    /// * **Invalid addresses**: Will panic if addresses are invalid
    /// * **Runtime updates**: Can be called after contract deployment
    /// * **Immediate effect**: Changes take effect immediately
    pub fn set_config(
        env: Env,
        caller: Address,
        user_mgmt_addr: Address,
        course_registry_addr: Address,
    ) {
        functions::config::set_contract_addrs(env, caller, user_mgmt_addr, course_registry_addr)
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

    /// Migrate access data between contract versions
    ///
    /// Performs data migration from one contract version to another.
    /// This function handles the transformation of course access data structures
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
    pub fn migrate_access_data(env: Env, caller: Address, from_version: String, to_version: String) -> bool {
        functions::contract_versioning::migrate_access_data(&env, caller, from_version, to_version)
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
}

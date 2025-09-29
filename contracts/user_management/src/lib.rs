// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

/// Contract version for tracking deployments and upgrades
pub const VERSION: &str = "1.0.0";

pub mod error;
pub mod functions;
pub mod models;

use error::Error;
use models::user::UserProfile;
use soroban_sdk::{contract, contractimpl, Address, Env, String};

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
}

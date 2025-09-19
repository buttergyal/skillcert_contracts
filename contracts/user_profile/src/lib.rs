// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

/// Contract version for tracking deployments and upgrades
pub const VERSION: &str = "1.0.0";

pub mod functions;
pub mod error;
pub mod schema;

#[cfg(test)]
mod test;

use crate::schema::UserProfile;
use soroban_sdk::{contract, contractimpl, Address, Env};

/// User Profile Contract
///
/// This contract provides read-only access to user profile information
/// with privacy controls and permission checks.
#[contract]
pub struct UserProfileContract;

#[contractimpl]
impl UserProfileContract {
    /// Get a user profile by address.
    ///
    /// This function retrieves a user's profile information using their blockchain address.
    /// This is a public function that returns basic profile information.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `user_address` - The blockchain address of the user whose profile to retrieve
    ///
    /// # Returns
    ///
    /// Returns the `UserProfile` containing the user's information.
    pub fn get_user_profile(env: Env, user_address: Address) -> UserProfile {
        functions::get_user_profile::user_profile_get_user_profile(&env, user_address)
    }

    /// Get a user profile with privacy controls.
    ///
    /// This function retrieves a user's profile information while respecting
    /// privacy settings. Sensitive information like email may be hidden
    /// depending on the requester's relationship to the profile owner.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `user_address` - The blockchain address of the user whose profile to retrieve
    /// * `requester_address` - The address of the user requesting the profile
    ///
    /// # Returns
    ///
    /// Returns the `UserProfile` with privacy-filtered information.
    pub fn get_user_profile_with_privacy(
        env: Env,
        user_address: Address,
        requester_address: Address,
    ) -> UserProfile {
        functions::get_user_profile::get_user_profile_with_privacy(
            &env,
            user_address,
            requester_address,
        )
    }
}

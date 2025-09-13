// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracttype, Address, String};

/// User profile information with privacy controls.
///
/// This struct represents a user's profile with optional privacy settings
/// and timestamps for tracking creation and updates.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserProfile {
    /// User's blockchain address
    pub address: Address,
    /// User's full name
    pub name: String,
    /// Optional email address (may be hidden for privacy)
    pub email: Option<String>,
    /// User's country of residence
    pub country: String,
    /// User's profession or job title
    pub profession: String,
    /// User's learning goals or objectives
    pub goals: String,
    /// Whether the profile is publicly viewable
    pub privacy_public: bool,
    /// Timestamp when the profile was created
    pub created_at: u64,
    /// Timestamp when the profile was last updated
    pub updated_at: u64,
}

/// Storage keys for user profile data.
///
/// This enum defines the keys used to store and retrieve
/// user profile data from the contract's persistent storage.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Key for storing user profiles: address -> UserProfile
    Profile(Address),
}

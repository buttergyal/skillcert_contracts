// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{DataKey, UserProfile};
use soroban_sdk::{Address, Env, String};

/// Save or update a user's profile information on-chain.
///
/// This function stores user profile data in persistent storage, including
/// personal and professional information. It validates required fields
/// before saving the profile.
///
/// # Arguments
///
/// * `env` - The Soroban environment
/// * `name` - The user's full name (required)
/// * `email` - The user's email address (required)
/// * `profession` - Optional profession or job title
/// * `goals` - Optional learning goals or objectives
/// * `country` - The user's country of residence (required)
/// * `user` - The address of the user whose profile is being saved
///
/// # Panics
///
/// Panics with appropriate error if any required field is empty:
/// - `Error::NameRequired` if name is empty
/// - `Error::EmailRequired` if email is empty
/// - `Error::CountryRequired` if country is empty
pub fn SaveUserProfile(
    env: Env,
    name: String,
    email: String,
    profession: Option<String>,
    goals: Option<String>,
    country: String,
    user: Address,
) {
    // Validate required fields
    if name.is_empty() {
        handle_error(&env, Error::NameRequired)
    }
    // TODO: Implement full email validation according to RFC 5322 standard
    if email.is_empty() {
        handle_error(&env, Error::EmailRequired)
    }
    if country.is_empty() {
        handle_error(&env, Error::CountryRequired)
    }

    let profile = UserProfile {
        name,
        email,
        profession,
        goals,
        country,
    };

    env.storage()
        .persistent()
        .set(&DataKey::UserProfile(user), &profile);
}

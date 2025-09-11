// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::{DataKey, UserProfile};
use soroban_sdk::{Address, Env, String};
use crate::error::{Error, handle_error};

pub fn save_profile(
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

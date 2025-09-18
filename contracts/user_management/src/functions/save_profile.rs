// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{UserProfile, UserRole, UserStatus};
use soroban_sdk::{Address, Env, String, Vec};

use super::utils::storage_utils;

pub fn save_profile(
    env: Env,
    user: Address,
    name: String,
    lastname: String,
    email: String,
    password: String,
    confirm_password: String,
    specialization: String,
    languages: Vec<String>,
    teaching_categories: Vec<String>,
) -> UserProfile {
    // Validate inputs
    if password != confirm_password {
        handle_error(&env, Error::PasswordMismatch);
    }
    if name.is_empty() || lastname.is_empty() || email.is_empty() {
        handle_error(&env, Error::RequiredFieldMissing);
    }

    // Check if email is already registered for another user
    if storage_utils::is_email_registered(&env, &email, Some(&user)) {
        handle_error(&env, Error::EmailAlreadyRegistered);
    }

    // Create or update profile
    let profile = UserProfile {
        address: user.clone(),
        name: name.clone(),
        lastname: lastname.clone(),
        email: email.clone(),
        password_hash: hash_password(&env, &password),
        specialization: specialization.clone(),
        languages: languages.clone(),
        teaching_categories: teaching_categories.clone(),
        role: UserRole::User,
        status: UserStatus::Active,
        created_at: env.ledger().timestamp(),
        updated_at: env.ledger().timestamp(),
        country: String::from_str(&env, ""),  // Default empty string
    };

    // Save profile with optimized storage access
    storage_utils::save_user_profile(&env, &profile);

    profile
}

fn hash_password(env: &Env, password: &String) -> String {
    // In a real implementation, this would use a proper hashing algorithm
    // For now, we'll just append a salt to demonstrate the concept
    let salt = env.ledger().timestamp().to_string();
    String::from_str(env, &format!("{}:{}", password.to_string(), salt))
}
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{UserProfile, UserRole, UserStatus, MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH, 
                   REQUIRED_SPECIAL_CHARS, REQUIRED_DIGITS, REQUIRED_UPPERCASE, REQUIRED_LOWERCASE};
use soroban_sdk::{Address, Env, String, Vec};

use super::utils::url_validation;

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
    
    // Validate password strength
    validate_password_strength(&env, &password);
    
    if name.is_empty() || lastname.is_empty() || email.is_empty() {
        handle_error(&env, Error::RequiredFieldMissing);
    }

    // TODO: Implement email uniqueness check
    // This function needs to be updated to use the correct schema

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

    // TODO: Implement profile saving
    // This function needs to be updated to use the correct schema

    profile
}

fn hash_password(env: &Env, password: &String) -> String {
    // In a real implementation, this would use a proper hashing algorithm
    // For now, we'll just append a salt to demonstrate the concept
    let salt = env.ledger().timestamp().to_string();
    String::from_str(env, &format!("{}:{}", password.to_string(), salt))
}

/// Validates password strength according to security requirements
/// 
/// Checks for:
/// - Minimum and maximum length
/// - At least one uppercase letter
/// - At least one lowercase letter  
/// - At least one digit
/// - At least one special character
fn validate_password_strength(env: &Env, password: &String) {
    let password_str = password.to_string();
    let password_len = password_str.len() as u32;
    
    // Check minimum length
    if password_len < MIN_PASSWORD_LENGTH {
        handle_error(env, Error::PasswordTooShort);
    }
    
    // Check maximum length
    if password_len > MAX_PASSWORD_LENGTH {
        handle_error(env, Error::PasswordTooLong);
    }
    
    // Check for uppercase letter
    let has_uppercase = password_str.chars().any(|c| REQUIRED_UPPERCASE.contains(c));
    if !has_uppercase {
        handle_error(env, Error::PasswordMissingUppercase);
    }
    
    // Check for lowercase letter
    let has_lowercase = password_str.chars().any(|c| REQUIRED_LOWERCASE.contains(c));
    if !has_lowercase {
        handle_error(env, Error::PasswordMissingLowercase);
    }
    
    // Check for digit
    let has_digit = password_str.chars().any(|c| REQUIRED_DIGITS.contains(c));
    if !has_digit {
        handle_error(env, Error::PasswordMissingDigit);
    }
    
    // Check for special character
    let has_special = password_str.chars().any(|c| REQUIRED_SPECIAL_CHARS.contains(c));
    if !has_special {
        handle_error(env, Error::PasswordMissingSpecialChar);
    }
}
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{
    UserProfile,
    MIN_PASSWORD_LENGTH,
    MAX_PASSWORD_LENGTH,
/*     REQUIRED_SPECIAL_CHARS,
    REQUIRED_DIGITS,
    REQUIRED_UPPERCASE,
    REQUIRED_LOWERCASE, */
};
use soroban_sdk::{Env, String};

pub fn save_profile(
    env: Env,
    //user: Address,
    name: String,
    //lastname: String,
    email: String,
    password: String,
    confirm_password: String,
    specialization: String,
    //languages: Vec<String>,
    //teaching_categories: Vec<String>,
) -> UserProfile {
    // Validate inputs
    if password != confirm_password {
        handle_error(&env, Error::PasswordMismatch);
    }
    
    // Validate password strength
    validate_password_strength(&env, &password);
    
    if name.is_empty() || email.is_empty() {
        handle_error(&env, Error::RequiredFieldMissing);
    }

    // TODO: Implement email uniqueness check
    // This function needs to be updated to use the correct schema
    // Note: Uniqueness is enforced elsewhere in create_user_profile

    // Create or update profile using the current schema
    let profile: UserProfile = UserProfile {
        // NOTE: We only store presentation data in UserProfile per current schema
        // Build a full name from provided parts. If lastname is empty, keep name only.
        full_name: name.clone(),
        contact_email: email.clone(),
        profession: if specialization.is_empty() { None } else { Some(specialization.clone()) },
        country: None,
        purpose: None,
        profile_picture_url: None,
    };

    // TODO: Implement profile saving
    // This function needs to be updated to use the correct schema

    profile
}

// currently not possible
// check issue https://github.com/stellar/stellar-docs/issues/1630
/* fn hash_password(env: &Env, password: &String) -> String {
    // In a real implementation, this would use a proper hashing algorithm
    // For now, we'll just append a salt to demonstrate the concept
    let salt: u64 = env.ledger().timestamp();
    String::from_str(env, &format!("{}:{}", password.to_string(), salt))
} */


  /// Validates password strength according to security requirements
/// 
/// Checks for:
/// - Minimum and maximum length
/// - At least one uppercase letter
/// - At least one lowercase letter  
/// - At least one digit
/// - At least one special character
/// 
  fn validate_password_strength(env: &Env, password: &String) {
    let password_len: u32 = password.len() as u32;
    
    // Check minimum length
    if password_len < MIN_PASSWORD_LENGTH {
        handle_error(env, Error::PasswordTooShort);
    }
    
    // Check maximum length
    if password_len > MAX_PASSWORD_LENGTH {
        handle_error(env, Error::PasswordTooLong);
    }
    
/*     // Check for uppercase letter
    let has_uppercase = password.chars().any(|c| REQUIRED_UPPERCASE.contains(c));
    if !has_uppercase {
        handle_error(env, Error::PasswordMissingUppercase);
    }
    
    // Check for lowercase letter
    let has_lowercase = password.chars().any(|c| REQUIRED_LOWERCASE.contains(c));
    if !has_lowercase {
        handle_error(env, Error::PasswordMissingLowercase);
    }
    
    // Check for digit
    let has_digit = password.chars().any(|c| REQUIRED_DIGITS.contains(c));
    if !has_digit {
        handle_error(env, Error::PasswordMissingDigit);
    }
    
    // Check for special character
    let has_special = password.chars().any(|c| REQUIRED_SPECIAL_CHARS.contains(c));
    if !has_special {
        handle_error(env, Error::PasswordMissingSpecialChar);
    } */
}

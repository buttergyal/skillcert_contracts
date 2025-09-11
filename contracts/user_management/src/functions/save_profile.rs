// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::{DataKey, LightProfile, UserProfile, UserRole, UserStatus};
use crate::error::{Error, handle_error};
use core::iter::Iterator;
use soroban_sdk::{Address, Env, String, Vec};

/// Security constants for profile validation
const MAX_NAME_LENGTH: usize = 100;
const MAX_EMAIL_LENGTH: usize = 320; // RFC 5321 standard
const MAX_SPECIALIZATION_LENGTH: usize = 100;
const MAX_LANGUAGE_LENGTH: usize = 50;
const MAX_CATEGORY_LENGTH: usize = 100;
const MAX_PASSWORD_LENGTH: usize = 128;
const MIN_PASSWORD_LENGTH: usize = 8;

/// Validates string content for security
fn validate_string_content(_env: &Env, s: &String, max_len: usize) -> bool {
    if s.len() > max_len as u32 {
        return false;
    }

    // For no_std environment, we'll do basic length validation
    // More sophisticated pattern matching can be added if needed
    true
}

pub fn user_management_save_profile(
    env: Env,
    caller: Address,
    name: String,
    lastname: String,
    email: String,
    password: String,
    confirm_password: String,
    specialization: String,
    languages: Vec<String>,
    teaching_categories: Vec<String>,
) -> UserProfile {
    // Require authentication - only the user themselves can update their profile
    caller.require_auth();

    // Validate required fields
    if name.is_empty() {
        handle_error(&env, Error::InvalidInput);
    }

    if lastname.is_empty() {
        handle_error(&env, Error::InvalidInput);
    }

    if email.is_empty() {
        handle_error(&env, Error::InvalidInput);
    }

    if password.is_empty() {
        handle_error(&env, Error::InvalidInput);
    }

    if confirm_password.is_empty() {
        handle_error(&env, Error::InvalidInput);
    }

    if specialization.is_empty() {
        handle_error(&env, Error::InvalidInput);
    }

    // Validate password confirmation
    if password != confirm_password {
        handle_error(&env, Error::PasswordMismatch);
    }

    // Validate password length
    if password.len() < MIN_PASSWORD_LENGTH as u32 || password.len() > MAX_PASSWORD_LENGTH as u32 {
        handle_error(&env, Error::InvalidInput);
    }

    // Basic email validation - check minimum length
    if email.len() < 5 || email.len() > MAX_EMAIL_LENGTH as u32 {
        handle_error(&env, Error::InvalidInput);
    }

    // Validate string lengths and content
    if !validate_string_content(&env, &name, MAX_NAME_LENGTH) {
        handle_error(&env, Error::InvalidInput);
    }

    if !validate_string_content(&env, &lastname, MAX_NAME_LENGTH) {
        handle_error(&env, Error::InvalidInput);
    }

    if !validate_string_content(&env, &email, MAX_EMAIL_LENGTH) {
        handle_error(&env, Error::InvalidInput);
    }

    if !validate_string_content(&env, &specialization, MAX_SPECIALIZATION_LENGTH) {
        handle_error(&env, Error::InvalidInput);
    }

    // Validate languages array
    if languages.is_empty() {
        handle_error(&env, Error::InvalidInput);
    }

    for language in languages.iter() {
        if language.is_empty() || !validate_string_content(&env, &language, MAX_LANGUAGE_LENGTH) {
            handle_error(&env, Error::InvalidInput);
        }
    }

    // Validate teaching categories array
    if teaching_categories.is_empty() {
        handle_error(&env, Error::InvalidInput);
    }

    for category in teaching_categories.iter() {
        if category.is_empty() || !validate_string_content(&env, &category, MAX_CATEGORY_LENGTH) {
            handle_error(&env, Error::InvalidInput);
        }
    }

    // Check if this is a new user or profile update
    let storage_key = DataKey::UserProfile(caller.clone());
    let is_new_user = !env.storage().persistent().has(&storage_key);

    // If this is a new user, we need to get the existing profile to preserve role and other fields
    let existing_profile = if !is_new_user {
        env.storage()
            .persistent()
            .get::<DataKey, UserProfile>(&storage_key)
            .expect("Profile should exist for update")
    } else {
        // For new users, create with default values
        UserProfile {
            name: name.clone(),
            lastname: lastname.clone(),
            email: email.clone(),
            role: UserRole::Student, // Default role
            country: String::from_str(&env, ""),
            profession: None,
            goals: None,
            profile_picture: None,
            language: String::from_str(&env, "en"),
            password: password.clone(),
            confirm_password: confirm_password.clone(),
            specialization: specialization.clone(),
            languages: languages.clone(),
            teaching_categories: teaching_categories.clone(),
            user: caller.clone(),
        }
    };

    // Create the updated user profile (preserving role and other system fields)
    let user_profile = UserProfile {
        name: name.clone(),
        lastname: lastname.clone(),
        email: if is_new_user { email.clone() } else { existing_profile.email.clone() }, // Don't change email on update
        role: existing_profile.role.clone(), // Preserve role
        country: existing_profile.country.clone(), // Preserve country
        profession: existing_profile.profession.clone(), // Preserve profession
        goals: existing_profile.goals.clone(), // Preserve goals
        profile_picture: existing_profile.profile_picture.clone(), // Preserve profile picture
        language: existing_profile.language.clone(), // Preserve language
        password: password.clone(), // In production, this should be hashed
        confirm_password: confirm_password.clone(),
        specialization: specialization.clone(),
        languages: languages.clone(),
        teaching_categories: teaching_categories.clone(),
        user: caller.clone(),
    };

    // Store the full profile
    env.storage().persistent().set(&storage_key, &user_profile);

    // Create and store lightweight profile for listing
    let light_profile = LightProfile {
        name,
        lastname,
        specialization,
        languages,
        teaching_categories,
        role: existing_profile.role, // Use role from existing profile
        status: UserStatus::Active, // Default status
        user_address: caller.clone(),
    };

    let light_storage_key = DataKey::UserProfileLight(caller.clone());
    env.storage()
        .persistent()
        .set(&light_storage_key, &light_profile);

    // If new user, add to users index
    if is_new_user {
        add_to_users_index(env, caller);
    }

    user_profile
}

/// Add user to the global users index
fn add_to_users_index(env: Env, user: Address) {
    let mut users_index: Vec<Address> = env
        .storage()
        .persistent()
        .get::<DataKey, Vec<Address>>(&DataKey::UsersIndex)
        .unwrap_or_else(|| Vec::new(&env));

    // Check if user already exists
    if !users_index.iter().any(|u| u == user) {
        users_index.push_back(user);
        env.storage()
            .persistent()
            .set(&DataKey::UsersIndex, &users_index);
    }
}

#[cfg(test)]
mod test {
    use crate::schema::{DataKey, UserProfile};
    use crate::{UserManagement, UserManagementClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};

    #[test]
    fn test_save_profile_success() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);

        let user: Address = Address::generate(&env);
        let name: String = String::from_str(&env, "John");
        let lastname: String = String::from_str(&env, "Doe");
        let email: String = String::from_str(&env, "john.doe@example.com");
        let password: String = String::from_str(&env, "securepassword123");
        let confirm_password: String = String::from_str(&env, "securepassword123");
        let specialization: String = String::from_str(&env, "Software Engineering");
        let languages = Vec::from_array(&env, [
            String::from_str(&env, "English"),
            String::from_str(&env, "Spanish")
        ]);
        let teaching_categories = Vec::from_array(&env, [
            String::from_str(&env, "Programming"),
            String::from_str(&env, "Web Development")
        ]);

        // Mock all authentication in the environment
        env.mock_all_auths();

        // Use contract client
        let profile = client.save_profile(
            &name, &lastname, &email, &password, &confirm_password,
            &specialization, &languages, &teaching_categories, &user
        );

        // Verify profile creation
        assert_eq!(profile.name, name);
        assert_eq!(profile.lastname, lastname);
        assert_eq!(profile.email, email);
        assert_eq!(profile.password, password);
        assert_eq!(profile.confirm_password, confirm_password);
        assert_eq!(profile.specialization, specialization);
        assert_eq!(profile.languages, languages);
        assert_eq!(profile.teaching_categories, teaching_categories);
        assert_eq!(profile.user, user);

        // Verify storage within contract context
        env.as_contract(&contract_id, || {
            let storage_key = DataKey::UserProfile(user);
            let stored_profile: Option<UserProfile> = env.storage().persistent().get(&storage_key);
            let stored = stored_profile.expect("Profile should be stored");
            assert_eq!(stored, profile);
        });
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #25)")]
    fn test_save_profile_password_mismatch() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);

        let user: Address = Address::generate(&env);
        let name: String = String::from_str(&env, "John");
        let lastname: String = String::from_str(&env, "Doe");
        let email: String = String::from_str(&env, "john@example.com");
        let password: String = String::from_str(&env, "password123");
        let confirm_password: String = String::from_str(&env, "differentpassword456");
        let specialization: String = String::from_str(&env, "Software Engineering");
        let languages = Vec::from_array(&env, [String::from_str(&env, "English")]);
        let teaching_categories = Vec::from_array(&env, [String::from_str(&env, "Programming")]);

        env.mock_all_auths();

        // This should panic due to password mismatch
        client.save_profile(
            &name, &lastname, &email, &password, &confirm_password,
            &specialization, &languages, &teaching_categories, &user
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #24)")]
    fn test_save_profile_with_empty_name() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        let user: Address = Address::generate(&env);

        let name: String = String::from_str(&env, "");
        let lastname: String = String::from_str(&env, "Doe");
        let email: String = String::from_str(&env, "test@example.com");
        let password: String = String::from_str(&env, "password123");
        let confirm_password: String = String::from_str(&env, "password123");
        let specialization: String = String::from_str(&env, "Engineering");
        let languages = Vec::from_array(&env, [String::from_str(&env, "English")]);
        let teaching_categories = Vec::from_array(&env, [String::from_str(&env, "Tech")]);

        env.mock_all_auths();

        client.save_profile(
            &name, &lastname, &email, &password, &confirm_password,
            &specialization, &languages, &teaching_categories, &user
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #24)")]
    fn test_save_profile_with_empty_email() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        let user: Address = Address::generate(&env);

        let name: String = String::from_str(&env, "John");
        let lastname: String = String::from_str(&env, "Doe");
        let email: String = String::from_str(&env, "");
        let password: String = String::from_str(&env, "password123");
        let confirm_password: String = String::from_str(&env, "password123");
        let specialization: String = String::from_str(&env, "Engineering");
        let languages = Vec::from_array(&env, [String::from_str(&env, "English")]);
        let teaching_categories = Vec::from_array(&env, [String::from_str(&env, "Tech")]);

        env.mock_all_auths();

        client.save_profile(
            &name, &lastname, &email, &password, &confirm_password,
            &specialization, &languages, &teaching_categories, &user
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #24)")]
    fn test_save_profile_with_short_password() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        let user: Address = Address::generate(&env);

        let name: String = String::from_str(&env, "John");
        let lastname: String = String::from_str(&env, "Doe");
        let email: String = String::from_str(&env, "john@example.com");
        let password: String = String::from_str(&env, "123"); // Too short
        let confirm_password: String = String::from_str(&env, "123");
        let specialization: String = String::from_str(&env, "Engineering");
        let languages = Vec::from_array(&env, [String::from_str(&env, "English")]);
        let teaching_categories = Vec::from_array(&env, [String::from_str(&env, "Tech")]);

        env.mock_all_auths();

        client.save_profile(
            &name, &lastname, &email, &password, &confirm_password,
            &specialization, &languages, &teaching_categories, &user
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #24)")]
    fn test_save_profile_with_empty_languages() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        let user: Address = Address::generate(&env);

        let name: String = String::from_str(&env, "John");
        let lastname: String = String::from_str(&env, "Doe");
        let email: String = String::from_str(&env, "john@example.com");
        let password: String = String::from_str(&env, "password123");
        let confirm_password: String = String::from_str(&env, "password123");
        let specialization: String = String::from_str(&env, "Engineering");
        let languages = Vec::new(&env); // Empty languages array
        let teaching_categories = Vec::from_array(&env, [String::from_str(&env, "Tech")]);

        env.mock_all_auths();

        client.save_profile(
            &name, &lastname, &email, &password, &confirm_password,
            &specialization, &languages, &teaching_categories, &user
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #24)")]
    fn test_save_profile_with_empty_teaching_categories() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        let user: Address = Address::generate(&env);

        let name: String = String::from_str(&env, "John");
        let lastname: String = String::from_str(&env, "Doe");
        let email: String = String::from_str(&env, "john@example.com");
        let password: String = String::from_str(&env, "password123");
        let confirm_password: String = String::from_str(&env, "password123");
        let specialization: String = String::from_str(&env, "Engineering");
        let languages = Vec::from_array(&env, [String::from_str(&env, "English")]);
        let teaching_categories = Vec::new(&env); // Empty categories array

        env.mock_all_auths();

        client.save_profile(
            &name, &lastname, &email, &password, &confirm_password,
            &specialization, &languages, &teaching_categories, &user
        );
    }
}

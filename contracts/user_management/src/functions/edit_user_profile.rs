// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::functions::is_admin::is_admin;
use crate::schema::{DataKey, LightProfile, ProfileUpdateParams, UserProfile};
use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

// Event symbol for user profile update
const EVT_USER_UPDATED: Symbol = symbol_short!("usr_updt");

// Security constants for profile validation (matching create_user_profile)
const MAX_NAME_LENGTH: usize = 100;
const MAX_PROFESSION_LENGTH: usize = 100;
const MAX_PURPOSE_LENGTH: usize = 500;
const MAX_COUNTRY_LENGTH: usize = 56; // Longest country name


/// Validates string content for security (reused from create_user_profile)
fn validate_string_content(_env: &Env, s: &String, max_len: usize) -> bool {
    if s.len() > max_len as u32 {
        return false;
    }
    
    true
}

/// Check if the caller has permission to edit the user profile
/// Only the user themselves or admins can edit
fn check_edit_permission(env: &Env, caller: &Address, user_id: &Address) -> bool {
    // User can edit their own profile
    if caller == user_id {
        return true;
    }

    // Admins can edit any profile
    is_admin(env.clone(), caller.clone())
}

/// Edit an existing user profile
///
/// Updates an existing user profile with new values for allowed fields.
/// Only the user themselves or administrators can perform updates.
/// Email and role fields cannot be updated through this function.
///
/// # Arguments
/// * `env` - Soroban environment
/// * `caller` - Address of the user performing the update
/// * `user_id` - Address of the user whose profile is being updated
/// * `updates` - ProfileUpdateParams containing fields to update
///
/// # Returns
/// * `UserProfile` - The updated user profile
///
/// # Panics
/// * If caller authentication fails
/// * If user profile doesn't exist
/// * If caller lacks permission to edit
/// * If any field validation fails
/// * If user is inactive
///
/// # Events
/// Emits a user update event upon successful profile update
pub fn edit_user_profile(
    env: Env,
    caller: Address,
    user_id: Address,
    updates: ProfileUpdateParams,
) -> UserProfile {
    // Require authentication for the caller
    caller.require_auth();

    // Check if user profile exists
    let storage_key = DataKey::UserProfile(user_id.clone());
    let mut profile: UserProfile = env
        .storage()
        .persistent()
        .get(&storage_key)
        .unwrap_or_else(|| handle_error(&env, Error::UserProfileNotFound));

    // Check permission to edit
    if !check_edit_permission(&env, &caller, &user_id) {
        handle_error(&env, Error::AccessDenied);
    }

    // Check if user is active by looking at light profile
    let light_storage_key = DataKey::UserProfileLight(user_id.clone());
    let light_profile: LightProfile = env
        .storage()
        .persistent()
        .get(&light_storage_key)
        .unwrap_or_else(|| handle_error(&env, Error::UserProfileNotFound));

    // Prevent editing inactive users
    if light_profile.status == crate::schema::UserStatus::Inactive {
        handle_error(&env, Error::InactiveUser);
    }

    // Apply updates with validation
    if let Some(ref name) = updates.full_name {
        if name.is_empty() {
            handle_error(&env, Error::NameRequired);
        }
        if !validate_string_content(&env, name, MAX_NAME_LENGTH) {
            handle_error(&env, Error::InvalidName);
        }
        profile.full_name = name.clone();
    }

    if let Some(ref profession) = updates.profession {
        if !profession.is_empty() && !validate_string_content(&env, profession, MAX_PROFESSION_LENGTH) {
            handle_error(&env, Error::InvalidProfession);
        }
        profile.profession = if profession.is_empty() { None } else { Some(profession.clone()) };
    }

    if let Some(ref country) = updates.country {
        if !country.is_empty() && !validate_string_content(&env, country, MAX_COUNTRY_LENGTH) {
            handle_error(&env, Error::InvalidCountry);
        }
        profile.country = if country.is_empty() { None } else { Some(country.clone()) };
    }

    if let Some(ref purpose) = updates.purpose {
        if !purpose.is_empty() && !validate_string_content(&env, purpose, MAX_PURPOSE_LENGTH) {
            handle_error(&env, Error::InvalidGoals);
        }
        profile.purpose = if purpose.is_empty() { None } else { Some(purpose.clone()) };
    }

    // Update the full profile in storage
    env.storage().persistent().set(&storage_key, &profile);

    // Update the light profile with new data
    let updated_light_profile = LightProfile {
        full_name: profile.full_name.clone(),
        profession: profile.profession.clone(),
        country: profile.country.clone(),
        role: light_profile.role, // Role cannot be changed through this function
        status: light_profile.status, // Status cannot be changed through this function
        user_address: user_id.clone(),
    };

    env.storage()
        .persistent()
        .set(&light_storage_key, &updated_light_profile);

    // Emit user update event
    env.events()
        .publish((EVT_USER_UPDATED, &user_id), user_id.clone());

    profile
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ProfileUpdateParams, UserProfile};
    use crate::{UserManagement, UserManagementClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn setup_test_env() -> (Env, Address, UserManagementClient<'static>) {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        (env, contract_id, client)
    }

    fn create_test_user(
        env: &Env,
        client: &UserManagementClient,
        user: &Address,
    ) -> UserProfile {
        let profile = UserProfile {
            full_name: String::from_str(env, "John Doe"),
            contact_email: String::from_str(env, "john@example.com"),
            profession: Some(String::from_str(env, "Software Engineer")),
            country: Some(String::from_str(env, "United States")),
            purpose: Some(String::from_str(env, "Learn blockchain development")),
        };

        env.mock_all_auths();
        client.create_user_profile(user, &profile)
    }

    #[test]
    fn test_edit_user_profile_by_self() {
        let (env, _contract_id, client) = setup_test_env();
        let user = Address::generate(&env);

        // Create user first
        create_test_user(&env, &client, &user);

        // Prepare updates
        let updates = ProfileUpdateParams {
            full_name: Some(String::from_str(&env, "Jane Doe")),
            profession: Some(String::from_str(&env, "Data Scientist")),
            country: Some(String::from_str(&env, "Canada")),
            purpose: Some(String::from_str(&env, "Master AI and ML")),
        };

        env.mock_all_auths();

        // Edit profile
        let updated_profile = client.edit_user_profile(&user, &user, &updates);

        // Verify updates
        assert_eq!(updated_profile.full_name, String::from_str(&env, "Jane Doe"));
        assert_eq!(
            updated_profile.profession,
            Some(String::from_str(&env, "Data Scientist"))
        );
        assert_eq!(
            updated_profile.country,
            Some(String::from_str(&env, "Canada"))
        );
        assert_eq!(
            updated_profile.purpose,
            Some(String::from_str(&env, "Master AI and ML"))
        );

        // Email should remain unchanged
        assert_eq!(
            updated_profile.contact_email,
            String::from_str(&env, "john@example.com")
        );
    }

    #[test]
    fn test_edit_user_profile_partial_update() {
        let (env, _contract_id, client) = setup_test_env();
        let user = Address::generate(&env);

        // Create user first
        let original_profile = create_test_user(&env, &client, &user);

        // Prepare partial updates (only name and country)
        let updates = ProfileUpdateParams {
            full_name: Some(String::from_str(&env, "Updated Name")),
            profession: None,
            country: Some(String::from_str(&env, "Germany")),
            purpose: None,
        };

        env.mock_all_auths();

        // Edit profile
        let updated_profile = client.edit_user_profile(&user, &user, &updates);

        // Verify only specified fields were updated
        assert_eq!(updated_profile.full_name, String::from_str(&env, "Updated Name"));
        assert_eq!(updated_profile.country, Some(String::from_str(&env, "Germany")));

        // Unchanged fields should retain original values
        assert_eq!(updated_profile.profession, original_profile.profession);
        assert_eq!(updated_profile.purpose, original_profile.purpose);
        assert_eq!(updated_profile.contact_email, original_profile.contact_email);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #21)")]
    fn test_edit_user_profile_nonexistent_user() {
        let (env, _contract_id, client) = setup_test_env();
        let user = Address::generate(&env);
        let caller = Address::generate(&env);

        let updates = ProfileUpdateParams {
            full_name: Some(String::from_str(&env, "New Name")),
            profession: None,
            country: None,
            purpose: None,
        };

        env.mock_all_auths();

        // Try to edit non-existent user profile
        client.edit_user_profile(&caller, &user, &updates);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #4)")]
    fn test_edit_user_profile_unauthorized() {
        let (env, _contract_id, client) = setup_test_env();
        let user = Address::generate(&env);
        let unauthorized_caller = Address::generate(&env);

        // Create user first
        create_test_user(&env, &client, &user);

        let updates = ProfileUpdateParams {
            full_name: Some(String::from_str(&env, "Hacker Name")),
            profession: None,
            country: None,
            purpose: None,
        };

        env.mock_all_auths();

        // Try to edit another user's profile without admin privileges
        client.edit_user_profile(&unauthorized_caller, &user, &updates);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #10)")]
    fn test_edit_user_profile_empty_name() {
        let (env, _contract_id, client) = setup_test_env();
        let user = Address::generate(&env);

        // Create user first
        create_test_user(&env, &client, &user);

        let updates = ProfileUpdateParams {
            full_name: Some(String::from_str(&env, "")), // Empty name
            profession: None,
            country: None,
            purpose: None,
        };

        env.mock_all_auths();

        // Try to set empty name
        client.edit_user_profile(&user, &user, &updates);
    }
}

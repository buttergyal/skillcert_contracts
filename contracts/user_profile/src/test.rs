// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::{UserProfile, UserProfileContract, UserProfileContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String, Symbol};

/// Helper function to create a test user profile
fn create_test_profile(env: &Env, address: Address) -> UserProfile {
    UserProfile {
        address: address.clone(),
        name: String::from_str(env, "John Doe"),
        email: Some(String::from_str(env, "john.doe@example.com")),
        country: String::from_str(env, "United States"),
        profession: String::from_str(env, "Software Engineer"),
        goals: String::from_str(env, "Learn blockchain development"),
        privacy_public: true,
        created_at: env.ledger().timestamp(),
        updated_at: env.ledger().timestamp(),
    }
}

/// Helper function to save a profile to storage
fn save_profile_to_storage(env: &Env, profile: &UserProfile) {
    let key = Symbol::new(env, "profile");
    env.storage()
        .instance()
        .set(&(key, profile.address.clone()), profile);
}

#[test]
fn test_get_user_profile_success() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let profile = create_test_profile(&env, user_address.clone());

    // Save profile to storage
    env.as_contract(&contract_id, || {
        save_profile_to_storage(&env, &profile);
    });

    // Test getting the profile
    let result = client.get_user_profile(&user_address);
    assert_eq!(result, profile);
}

#[test]
#[should_panic(expected = "User profile not found")]
fn test_get_user_profile_not_found() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);

    // Try to get a profile that doesn't exist
    client.get_user_profile(&user_address);
}

#[test]
fn test_get_user_profile_with_privacy_public_profile() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let requester_address = Address::generate(&env);
    let mut profile = create_test_profile(&env, user_address.clone());
    profile.privacy_public = true;

    // Save profile to storage
    env.as_contract(&contract_id, || {
        save_profile_to_storage(&env, &profile);
    });

    // Test getting the profile with privacy (should show email for public profile)
    let result = client.get_user_profile_with_privacy(&user_address, &requester_address);
    assert_eq!(result.email, profile.email);
    assert_eq!(result.privacy_public, true);
}

#[test]
fn test_get_user_profile_with_privacy_private_profile_owner() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let mut profile = create_test_profile(&env, user_address.clone());
    profile.privacy_public = false;

    // Save profile to storage
    env.as_contract(&contract_id, || {
        save_profile_to_storage(&env, &profile);
    });

    // Test getting the profile with privacy (owner should see email)
    let result = client.get_user_profile_with_privacy(&user_address, &user_address);
    assert_eq!(result.email, profile.email);
    assert_eq!(result.privacy_public, false);
}

#[test]
fn test_get_user_profile_with_privacy_private_profile_other_user() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let requester_address = Address::generate(&env);
    let mut profile = create_test_profile(&env, user_address.clone());
    profile.privacy_public = false;

    // Save profile to storage
    env.as_contract(&contract_id, || {
        save_profile_to_storage(&env, &profile);
    });

    // Test getting the profile with privacy (other user should not see email)
    let result = client.get_user_profile_with_privacy(&user_address, &requester_address);
    assert_eq!(result.email, None); // Email should be hidden
    assert_eq!(result.privacy_public, false);
}

#[test]
fn test_get_user_profile_with_privacy_public_profile_other_user() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let requester_address = Address::generate(&env);
    let profile = create_test_profile(&env, user_address.clone());

    // Save profile to storage
    env.as_contract(&contract_id, || {
        save_profile_to_storage(&env, &profile);
    });

    // Test getting the profile with privacy (other user should see email for public profile)
    let result = client.get_user_profile_with_privacy(&user_address, &requester_address);
    assert_eq!(result.email, profile.email);
    assert_eq!(result.privacy_public, true);
}

#[test]
fn test_get_user_profile_with_privacy_same_user() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let profile = create_test_profile(&env, user_address.clone());

    // Save profile to storage
    env.as_contract(&contract_id, || {
        save_profile_to_storage(&env, &profile);
    });

    // Test getting the profile with privacy (same user should always see their own data)
    let result = client.get_user_profile_with_privacy(&user_address, &user_address);
    assert_eq!(result.email, profile.email);
    assert_eq!(result, profile);
}

#[test]
#[should_panic(expected = "User profile not found")]
fn test_get_user_profile_with_privacy_not_found() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let requester_address = Address::generate(&env);

    // Try to get a profile that doesn't exist
    client.get_user_profile_with_privacy(&user_address, &requester_address);
}

#[test]
fn test_profile_data_integrity() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user_address = Address::generate(&env);
    let mut profile = create_test_profile(&env, user_address.clone());
    profile.privacy_public = false;
    profile.email = None; // Test with no email

    // Save profile to storage
    env.as_contract(&contract_id, || {
        save_profile_to_storage(&env, &profile);
    });

    // Test that all data is preserved correctly
    let result = client.get_user_profile(&user_address);
    assert_eq!(result.address, profile.address);
    assert_eq!(result.name, profile.name);
    assert_eq!(result.email, profile.email);
    assert_eq!(result.country, profile.country);
    assert_eq!(result.profession, profile.profession);
    assert_eq!(result.goals, profile.goals);
    assert_eq!(result.privacy_public, profile.privacy_public);
    assert_eq!(result.created_at, profile.created_at);
    assert_eq!(result.updated_at, profile.updated_at);
}

#[test]
fn test_multiple_users_profiles() {
    let env = Env::default();
    let contract_id = env.register(UserProfileContract, {});
    let client = UserProfileContractClient::new(&env, &contract_id);

    let user1_address = Address::generate(&env);
    let user2_address = Address::generate(&env);

    let profile1 = create_test_profile(&env, user1_address.clone());
    let mut profile2 = create_test_profile(&env, user2_address.clone());
    profile2.name = String::from_str(&env, "Jane Smith");
    profile2.email = Some(String::from_str(&env, "jane.smith@example.com"));

    // Save both profiles to storage
    env.as_contract(&contract_id, || {
        save_profile_to_storage(&env, &profile1);
        save_profile_to_storage(&env, &profile2);
    });

    // Test getting both profiles
    let result1 = client.get_user_profile(&user1_address);
    let result2 = client.get_user_profile(&user2_address);

    assert_eq!(result1, profile1);
    assert_eq!(result2, profile2);
    assert_ne!(result1, result2);
}

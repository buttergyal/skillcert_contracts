// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::UserProfile;
use crate::{UserManagement, UserManagementClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_create_user_profile_integration() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);
    let user: Address = Address::generate(&env);

    let profile = UserProfile {
        full_name: String::from_str(&env, "Alice Johnson"),
        contact_email: String::from_str(&env, "alice@example.com"),
        profession: Some(String::from_str(&env, "Data Scientist")),
        country: Some(String::from_str(&env, "United States")),
        purpose: Some(String::from_str(&env, "Learn machine learning")),
    };

    // Mock authentication
    env.mock_all_auths();

    let created_profile = client.create_user_profile(&user, &profile);

    // Verify the returned profile
    assert_eq!(created_profile.full_name, profile.full_name);
    assert_eq!(created_profile.contact_email, profile.contact_email);
    assert_eq!(created_profile.profession, profile.profession);
    assert_eq!(created_profile.country, profile.country);
    assert_eq!(created_profile.purpose, profile.purpose);
}

#[test]
fn test_get_user_by_id_self_access() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);
    let user: Address = Address::generate(&env);

    // First create a profile
    let profile = UserProfile {
        full_name: String::from_str(&env, "Bob Wilson"),
        contact_email: String::from_str(&env, "bob@example.com"),
        profession: Some(String::from_str(&env, "Software Engineer")),
        country: Some(String::from_str(&env, "Canada")),
        purpose: Some(String::from_str(&env, "Improve coding skills")),
    };

    env.mock_all_auths();

    client.create_user_profile(&user, &profile);

    // User retrieves their own profile (self-access)
    let retrieved_profile = client.get_user_by_id(&user, &user);

    assert_eq!(retrieved_profile.full_name, profile.full_name);
    assert_eq!(retrieved_profile.contact_email, profile.contact_email);
    assert_eq!(retrieved_profile.profession, profile.profession);
    assert_eq!(retrieved_profile.country, profile.country);
    assert_eq!(retrieved_profile.purpose, profile.purpose);
}

#[test]
fn test_get_user_by_id_admin_access() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);
    let user: Address = Address::generate(&env);
    let admin: Address = Address::generate(&env);

    env.mock_all_auths();

    // Initialize system with admin
    client.initialize_system(&admin, &admin, &None);

    // First create a profile
    let profile = UserProfile {
        full_name: String::from_str(&env, "Bob Wilson"),
        contact_email: String::from_str(&env, "bob@example.com"),
        profession: Some(String::from_str(&env, "Software Engineer")),
        country: Some(String::from_str(&env, "Canada")),
        purpose: Some(String::from_str(&env, "Improve coding skills")),
    };

    client.create_user_profile(&user, &profile);

    // Admin retrieves user's profile
    let retrieved_profile = client.get_user_by_id(&admin, &user);

    assert_eq!(retrieved_profile.full_name, profile.full_name);
    assert_eq!(retrieved_profile.contact_email, profile.contact_email);
    assert_eq!(retrieved_profile.profession, profile.profession);
    assert_eq!(retrieved_profile.country, profile.country);
    assert_eq!(retrieved_profile.purpose, profile.purpose);
}

#[test]
fn test_list_all_users_basic() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);

    // Initialize system first
    let super_admin: Address = Address::generate(&env);
    env.mock_all_auths();
    client.initialize_system(&super_admin, &super_admin, &None);

    // Create some test users
    let test_data = [
        ("John Doe", "john@example.com", "Engineer"),
        ("Jane Smith", "jane@example.com", "Scientist"),
        ("Bob Johnson", "bob@example.com", "Teacher"),
    ];

    for (name, email, profession) in test_data.iter() {
        let user: Address = Address::generate(&env);
        let profile = UserProfile {
            full_name: String::from_str(&env, name),
            contact_email: String::from_str(&env, email),
            profession: Some(String::from_str(&env, profession)),
            country: Some(String::from_str(&env, "United States")),
            purpose: Some(String::from_str(&env, "Learn new skills")),
        };

        client.create_user_profile(&user, &profile);
    }

    // Test basic listing
    let users = client.list_all_users(
        &super_admin,
        &0,    // page
        &10,   // page_size
        &None, // role_filter
        &None, // country_filter
        &None, // status_filter
    );

    assert_eq!(users.len(), 3);
}

#[test]
fn test_delete_user() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);
    let user: Address = Address::generate(&env);

    // Create a profile first
    let profile = UserProfile {
        full_name: String::from_str(&env, "Test User"),
        contact_email: String::from_str(&env, "test@example.com"),
        profession: Some(String::from_str(&env, "Tester")),
        country: Some(String::from_str(&env, "United States")),
        purpose: Some(String::from_str(&env, "Learn testing")),
    };

    env.mock_all_auths();

    client.create_user_profile(&user, &profile);

    // Delete the user (self-deletion)
    client.delete_user(&user, &user);

    // Note: The actual deletion logic would need to be tested based on the implementation
    // This test verifies the function can be called without panicking
}

#[test]
fn test_admin_functionality() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);

    let super_admin: Address = Address::generate(&env);
    let new_admin: Address = Address::generate(&env);

    env.mock_all_auths();

    // Initialize system
    let config = client.initialize_system(&super_admin, &super_admin, &None);
    assert_eq!(config.super_admin, super_admin);
    assert!(config.initialized);

    // Add new admin
    client.add_admin(&super_admin, &new_admin);

    // Verify admin was added
    let admins = client.get_admins(&super_admin);
    assert!(admins.contains(&new_admin));
    assert!(admins.contains(&super_admin));

    // Check admin status
    assert!(client.is_admin(&super_admin));
    assert!(client.is_admin(&new_admin));

    // Remove admin
    client.remove_admin(&super_admin, &new_admin);

    // Verify admin was removed
    let admins_after_removal = client.get_admins(&super_admin);
    assert!(!admins_after_removal.contains(&new_admin));
    assert!(admins_after_removal.contains(&super_admin)); // Super admin should remain
}

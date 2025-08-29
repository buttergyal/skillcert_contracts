use crate::{UserManagement, UserManagementClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};

#[test]
fn test_save_profile_integration() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);
    let user: Address = Address::generate(&env);

    let name: String = String::from_str(&env, "Alice");
    let lastname: String = String::from_str(&env, "Johnson");
    let email: String = String::from_str(&env, "alice@example.com");
    let password: String = String::from_str(&env, "securepassword123");
    let confirm_password: String = String::from_str(&env, "securepassword123");
    let specialization: String = String::from_str(&env, "Data Science");
    let languages = Vec::from_array(&env, [String::from_str(&env, "English")]);
    let teaching_categories = Vec::from_array(&env, [String::from_str(&env, "Analytics")]);

    // Mock authentication
    env.mock_all_auths();

    let profile = client.save_profile(
        &name, &lastname, &email, &password, &confirm_password,
        &specialization, &languages, &teaching_categories, &user
    );

    // Verify the returned profile
    assert_eq!(profile.name, name);
    assert_eq!(profile.lastname, lastname);
    assert_eq!(profile.email, email);
    assert_eq!(profile.specialization, specialization);
    assert_eq!(profile.languages, languages);
    assert_eq!(profile.teaching_categories, teaching_categories);
    assert_eq!(profile.user, user);
}

#[test]
fn test_get_user_by_id_self_access() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);
    let user: Address = Address::generate(&env);

    // First create a profile
    let name: String = String::from_str(&env, "Bob");
    let lastname: String = String::from_str(&env, "Wilson");
    let email: String = String::from_str(&env, "bob@example.com");
    let password: String = String::from_str(&env, "password123");
    let confirm_password: String = String::from_str(&env, "password123");
    let specialization: String = String::from_str(&env, "Software Engineering");
    let languages = Vec::from_array(&env, [String::from_str(&env, "Spanish")]);
    let teaching_categories = Vec::from_array(&env, [String::from_str(&env, "Programming")]);

    env.mock_all_auths();

    client.save_profile(
        &name, &lastname, &email, &password, &confirm_password,
        &specialization, &languages, &teaching_categories, &user
    );

    // User retrieves their own profile (self-access)
    let retrieved_profile = client.get_user_by_id(&user, &user);

    assert_eq!(retrieved_profile.name, name);
    assert_eq!(retrieved_profile.lastname, lastname);
    assert_eq!(retrieved_profile.email, email);
    assert_eq!(retrieved_profile.specialization, specialization);
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
    let name: String = String::from_str(&env, "Bob");
    let lastname: String = String::from_str(&env, "Wilson");
    let email: String = String::from_str(&env, "bob@example.com");
    let password: String = String::from_str(&env, "password123");
    let confirm_password: String = String::from_str(&env, "password123");
    let specialization: String = String::from_str(&env, "Software Engineering");
    let languages = Vec::from_array(&env, [String::from_str(&env, "Spanish")]);
    let teaching_categories = Vec::from_array(&env, [String::from_str(&env, "Programming")]);

    client.save_profile(
        &name, &lastname, &email, &password, &confirm_password,
        &specialization, &languages, &teaching_categories, &user
    );

    // Admin retrieves user's profile
    let retrieved_profile = client.get_user_by_id(&admin, &user);

    assert_eq!(retrieved_profile.name, name);
    assert_eq!(retrieved_profile.lastname, lastname);
    assert_eq!(retrieved_profile.email, email);
    assert_eq!(retrieved_profile.specialization, specialization);
}

/// Helper function to create test profile data
fn create_test_profile(
    env: &Env,
    name: &str,
    lastname: &str,
    email: &str,
    _specialization: &str,
    languages: Vec<String>,
    teaching_categories: Vec<String>,
) -> (String, String, String, String, String, Vec<String>, Vec<String>) {
    (
        String::from_str(env, name),
        String::from_str(env, lastname),
        String::from_str(env, email),
        String::from_str(env, "password123"),
        String::from_str(env, "password123"),
        languages,
        teaching_categories,
    )
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
        ("John", "Doe", "john@example.com", "Engineering"),
        ("Jane", "Smith", "jane@example.com", "Science"),
        ("Bob", "Johnson", "bob@example.com", "Mathematics"),
    ];

    for (name, lastname, email, specialization) in test_data.iter() {
        let user: Address = Address::generate(&env);
        let languages = Vec::from_array(&env, [String::from_str(&env, "English")]);
        let categories = Vec::from_array(&env, [String::from_str(&env, specialization)]);
        
        let (name_str, lastname_str, email_str, password, confirm_password, _, _) = 
            create_test_profile(&env, name, lastname, email, specialization, languages.clone(), categories.clone());
        
        client.save_profile(
            &name_str, &lastname_str, &email_str, &password, &confirm_password,
            &String::from_str(&env, specialization), &languages, &categories, &user
        );
    }

    // Test basic listing
    let users = client.list_all_users(
        &super_admin,
        &0,    // page
        &10,   // page_size
        &None, // role_filter
        &None, // country_filter (removed from new schema)
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
    let name: String = String::from_str(&env, "Test");
    let lastname: String = String::from_str(&env, "User");
    let email: String = String::from_str(&env, "test@example.com");
    let password: String = String::from_str(&env, "password123");
    let confirm_password: String = String::from_str(&env, "password123");
    let specialization: String = String::from_str(&env, "Testing");
    let languages = Vec::from_array(&env, [String::from_str(&env, "English")]);
    let teaching_categories = Vec::from_array(&env, [String::from_str(&env, "QA")]);

    env.mock_all_auths();

    client.save_profile(
        &name, &lastname, &email, &password, &confirm_password,
        &specialization, &languages, &teaching_categories, &user
    );

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
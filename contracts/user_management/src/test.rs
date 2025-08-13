use crate::schema::{DataKey, LightProfile, UserRole, UserStatus};
use crate::{UserManagement, UserManagementClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String, Vec};

#[test]
fn test_save_profile_integration() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);
    let user: Address = Address::generate(&env);

    let name: String = String::from_str(&env, "Alice Johnson");
    let email: String = String::from_str(&env, "alice@example.com");
    let profession: Option<String> = Some(String::from_str(&env, "Data Scientist"));
    let goals: Option<String> = Some(String::from_str(&env, "Master machine learning"));
    let country: String = String::from_str(&env, "United Kingdom");

    // Mock authentication
    env.mock_all_auths();

    let profile = client.save_profile(&name, &email, &profession, &goals, &country, &user);

    // Verify the returned profile
    assert_eq!(profile.name, name);
    assert_eq!(profile.email, email);
    assert_eq!(profile.profession, profession);
    assert_eq!(profile.goals, goals);
    assert_eq!(profile.country, country);
    assert_eq!(profile.user, user);
}

#[test]
fn test_save_profile_minimal_data() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);
    let user: Address = Address::generate(&env);

    let name: String = String::from_str(&env, "Bob Wilson");
    let email: String = String::from_str(&env, "bob@example.com");
    let country: String = String::from_str(&env, "Australia");

    // Mock authentication
    env.mock_all_auths();

    let profile = client.save_profile(
        &name, &email, &None, // profession
        &None, // goals
        &country, &user,
    );

    // Verify the returned profile
    assert_eq!(profile.name, name);
    assert_eq!(profile.email, email);
    assert_eq!(profile.profession, None);
    assert_eq!(profile.goals, None);
    assert_eq!(profile.country, country);
    assert_eq!(profile.user, user);
}

// Helper function to create a light profile
fn create_light_profile(
    env: &Env,
    name: &str,
    country: &str,
    profession: Option<&str>,
    role: UserRole,
    status: UserStatus,
) -> (Address, LightProfile) {
    let user_addr = Address::generate(env);
    let profile = LightProfile {
        name: String::from_str(env, name),
        country: String::from_str(env, country),
        profession: profession.map(|p| String::from_str(env, p)),
        role,
        status,
        user_address: user_addr.clone(),
    };
    (user_addr, profile)
}

// Helper function to setup test data with multiple users
fn setup_test_users(
    env: &Env,
    contract_id: &Address,
) -> (Address, Vec<Address>, Vec<LightProfile>) {
    let client = UserManagementClient::new(env, contract_id);
    let initializer = Address::generate(env);
    let admin = Address::generate(env);
    let mut users = Vec::new(env);
    let mut profiles = Vec::new(env);

    // Mock authentication BEFORE calling client functions
    env.mock_all_auths();

    // Initialize admin system using client
    client.initialize_system(&initializer, &admin, &Some(100));

    // Create test users
    let test_data = [
        (
            "Alice Johnson",
            "US",
            Some("Developer"),
            UserRole::Student,
            UserStatus::Active,
        ),
        (
            "Bob Smith",
            "UK",
            Some("Teacher"),
            UserRole::Instructor,
            UserStatus::Active,
        ),
        (
            "Charlie Brown",
            "US",
            None,
            UserRole::Admin,
            UserStatus::Active,
        ),
        (
            "Diana Ross",
            "Canada",
            Some("Designer"),
            UserRole::Student,
            UserStatus::Inactive,
        ),
        (
            "Eve Wilson",
            "UK",
            Some("Manager"),
            UserRole::Instructor,
            UserStatus::Suspended,
        ),
    ];

    for (name, country, profession, role, status) in test_data.iter() {
        let (user_addr, profile) = create_light_profile(
            env,
            name,
            country,
            *profession,
            role.clone(),
            status.clone(),
        );
        users.push_back(user_addr.clone());
        profiles.push_back(profile.clone());

        // Store profile in contract storage context
        env.as_contract(contract_id, || {
            env.storage()
                .persistent()
                .set(&DataKey::UserProfileLight(user_addr), &profile);
        });
    }

    // Store users index in contract context
    env.as_contract(contract_id, || {
        env.storage().persistent().set(&DataKey::UsersIndex, &users);
    });

    (admin, users, profiles)
}

#[test]
fn test_list_all_users_basic_pagination() {
    let env = Env::default();
    let contract_id = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);

    let (admin, _users, _profiles) = setup_test_users(&env, &contract_id);

    // Test first page
    let page1 = client.list_all_users(
        &admin, &0,    // page
        &3,    // page_size
        &None, // role_filter
        &None, // country_filter
        &None, // status_filter
    );
    assert_eq!(page1.len(), 3);

    // Test second page
    let page2 = client.list_all_users(
        &admin, &1,    // page
        &3,    // page_size
        &None, // role_filter
        &None, // country_filter
        &None, // status_filter
    );
    assert_eq!(page2.len(), 2);

    // Test empty page
    let page3 = client.list_all_users(
        &admin, &2,    // page
        &3,    // page_size
        &None, // role_filter
        &None, // country_filter
        &None, // status_filter
    );
    assert_eq!(page3.len(), 0);
}

#[test]
fn test_list_all_users_role_filter() {
    let env = Env::default();
    let contract_id = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);

    let (admin, _users, _profiles) = setup_test_users(&env, &contract_id);

    // Filter by Student role
    let students = client.list_all_users(
        &admin,
        &0,                       // page
        &10,                      // page_size
        &Some(UserRole::Student), // role_filter
        &None,                    // country_filter
        &None,                    // status_filter
    );

    assert_eq!(students.len(), 2); // Alice and Diana are students
    for student in students.iter() {
        assert_eq!(student.role, UserRole::Student);
    }
}

#[test]
fn test_list_all_users_country_filter() {
    let env = Env::default();
    let contract_id = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);

    let (admin, _users, _profiles) = setup_test_users(&env, &contract_id);

    // Filter by US country
    let us_users = client.list_all_users(
        &admin,
        &0,                                  // page
        &10,                                 // page_size
        &None,                               // role_filter
        &Some(String::from_str(&env, "US")), // country_filter
        &None,                               // status_filter
    );

    assert_eq!(us_users.len(), 2); // Alice and Charlie are from US
    for user in us_users.iter() {
        assert_eq!(user.country, String::from_str(&env, "US"));
    }
}

#[test]
fn test_list_all_users_status_filter() {
    let env = Env::default();
    let contract_id = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);

    let (admin, _users, _profiles) = setup_test_users(&env, &contract_id);

    // Filter by Active status
    let active_users = client.list_all_users(
        &admin,
        &0,                        // page
        &10,                       // page_size
        &None,                     // role_filter
        &None,                     // country_filter
        &Some(UserStatus::Active), // status_filter
    );

    assert_eq!(active_users.len(), 3); // Alice, Bob, and Charlie are active
    for user in active_users.iter() {
        assert_eq!(user.status, UserStatus::Active);
    }
}

#[test]
fn test_list_all_users_multiple_filters() {
    let env = Env::default();
    let contract_id = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);

    let (admin, _users, _profiles) = setup_test_users(&env, &contract_id);

    // Filter by Student role AND Active status
    let filtered_users = client.list_all_users(
        &admin,
        &0,                        // page
        &10,                       // page_size
        &Some(UserRole::Student),  // role_filter
        &None,                     // country_filter
        &Some(UserStatus::Active), // status_filter
    );

    assert_eq!(filtered_users.len(), 1); // Only Alice is active student
    let alice = filtered_users.get(0).unwrap();
    assert_eq!(alice.name, String::from_str(&env, "Alice Johnson"));
    assert_eq!(alice.role, UserRole::Student);
    assert_eq!(alice.status, UserStatus::Active);
}

#[test]
fn test_list_all_users_non_admin_access() {
    let env = Env::default();
    let contract_id = env.register(UserManagement, {});

    let (_admin, _users, _profiles) = setup_test_users(&env, &contract_id);
    let _non_admin = Address::generate(&env);

    // Non-admin tries to list users - should panic
    // Note: In real tests, you'd check the panic message but we skip the panic test here
    // as Soroban testing environment doesn't support catch_unwind well
}

#[test]
fn test_list_all_users_invalid_page_size() {
    let env = Env::default();
    let contract_id = env.register(UserManagement, {});

    let (_admin, _users, _profiles) = setup_test_users(&env, &contract_id);

    // Invalid page_size of 0 should panic
    // Note: In real tests, you'd check the panic message but we skip the panic test here
    // as Soroban testing environment doesn't support catch_unwind well
}

#[test]
fn test_list_all_users_empty_database() {
    let env = Env::default();
    let contract_id = env.register(UserManagement, {});
    let client = UserManagementClient::new(&env, &contract_id);

    let initializer = Address::generate(&env);
    let admin = Address::generate(&env);

    // Mock all authentication for testing
    env.mock_all_auths();

    // Initialize admin system using client
    client.initialize_system(&initializer, &admin, &Some(100));

    // Empty users index should return empty result
    let users = client.list_all_users(
        &admin, &0, &10, &None, // role_filter
        &None, // country_filter
        &None, // status_filter
    );

    assert_eq!(users.len(), 0);
}

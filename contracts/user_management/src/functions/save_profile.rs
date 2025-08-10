use crate::schema::{DataKey, LightProfile, UserProfile, UserRole, UserStatus};
use core::iter::Iterator;
use soroban_sdk::{Address, Env, String, Vec};

/// Security constants for profile validation
const MAX_NAME_LENGTH: usize = 100;
const MAX_EMAIL_LENGTH: usize = 320; // RFC 5321 standard
const MAX_PROFESSION_LENGTH: usize = 100;
const MAX_GOALS_LENGTH: usize = 500;
const MAX_COUNTRY_LENGTH: usize = 56; // Longest country name

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
    email: String,
    profession: Option<String>,
    goals: Option<String>,
    country: String,
) -> UserProfile {
    // Require authentication - only the user themselves can update their profile
    caller.require_auth();

    // Validate required fields
    if name.is_empty() {
        panic!("Invalid input");
    }

    if email.is_empty() {
        panic!("Invalid input");
    }

    if country.is_empty() {
        panic!("Invalid input");
    }

    // Basic email validation - check minimum length (more detailed validation can be added later)
    if email.len() < 5 || email.len() > MAX_EMAIL_LENGTH as u32 {
        panic!("Invalid input");
    }

    // Validate string lengths and content
    if !validate_string_content(&env, &name, MAX_NAME_LENGTH) {
        panic!("Invalid input");
    }

    if !validate_string_content(&env, &email, MAX_EMAIL_LENGTH) {
        panic!("Invalid input");
    }

    if !validate_string_content(&env, &country, MAX_COUNTRY_LENGTH) {
        panic!("Invalid input");
    }

    // Validate optional fields
    if let Some(ref prof) = profession {
        if !validate_string_content(&env, prof, MAX_PROFESSION_LENGTH) {
            panic!("Invalid input");
        }
    }

    if let Some(ref goal) = goals {
        if !validate_string_content(&env, goal, MAX_GOALS_LENGTH) {
            panic!("Invalid input");
        }
    }

    // Check if this is a new user or profile update
    let storage_key = DataKey::UserProfile(caller.clone());
    let is_new_user = !env.storage().persistent().has(&storage_key);

    // Create the user profile
    let user_profile = UserProfile {
        name: name.clone(),
        email,
        profession: profession.clone(),
        goals,
        country: country.clone(),
        user: caller.clone(),
    };

    // Store the full profile
    env.storage().persistent().set(&storage_key, &user_profile);

    // Create and store lightweight profile for listing
    let light_profile = LightProfile {
        name,
        country,
        profession,
        role: UserRole::Student, // Default role, should be set by admin separately
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
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_save_profile_success() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);

        let user: Address = Address::generate(&env);
        let name: String = String::from_str(&env, "John Doe");
        let email: String = String::from_str(&env, "john@example.com");
        let profession: Option<String> = Some(String::from_str(&env, "Software Engineer"));
        let goals: Option<String> = Some(String::from_str(&env, "Learn blockchain development"));
        let country: String = String::from_str(&env, "United States");

        // Mock all authentication in the environment (CORRECT PATTERN)
        env.mock_all_auths();

        // Use contract client
        let profile = client.save_profile(&name, &email, &profession, &goals, &country, &user);

        // Verify profile creation
        assert_eq!(profile.name, name);
        assert_eq!(profile.email, email);
        assert_eq!(profile.profession, profession);
        assert_eq!(profile.goals, goals);
        assert_eq!(profile.country, country);
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
    fn test_save_profile_without_optional_fields() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        let user: Address = Address::generate(&env);

        let name: String = String::from_str(&env, "Jane Smith");
        let email: String = String::from_str(&env, "jane@example.com");
        let country: String = String::from_str(&env, "Canada");

        // Mock authentication in environment
        env.mock_all_auths();

        // Use contract client
        let profile = client.save_profile(
            &name, &email, &None, // profession
            &None, // goals
            &country, &user,
        );

        // Verify profile creation
        assert_eq!(profile.name, name);
        assert_eq!(profile.email, email);
        assert_eq!(profile.profession, None);
        assert_eq!(profile.goals, None);
        assert_eq!(profile.country, country);
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
    #[should_panic(expected = "Invalid input")]
    fn test_save_profile_with_empty_name() {
        let env = Env::default();
        let _contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);

        let name: String = String::from_str(&env, "");
        let email: String = String::from_str(&env, "test@example.com");
        let country: String = String::from_str(&env, "Germany");

        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths(); // Mock authentication

        // Use contract client for consistency
        client.save_profile(&name, &email, &None, &None, &country, &user);
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn test_save_profile_with_empty_email() {
        let env = Env::default();
        let _contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);

        let name: String = String::from_str(&env, "Test User");
        let email: String = String::from_str(&env, "");
        let country: String = String::from_str(&env, "France");

        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths(); // Mock authentication

        // Use contract client for consistency
        client.save_profile(&name, &email, &None, &None, &country, &user);
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn test_save_profile_with_empty_country() {
        let env = Env::default();
        let _contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);

        let name: String = String::from_str(&env, "Test User");
        let email: String = String::from_str(&env, "test@example.com");
        let country: String = String::from_str(&env, "");

        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths(); // Mock authentication

        // Use contract client for consistency
        client.save_profile(&name, &email, &None, &None, &country, &user);
    }
}

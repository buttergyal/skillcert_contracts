use soroban_sdk::{Address, Env, String};
use crate::schema::{UserProfile, DataKey};

pub fn user_management_save_profile(
    env: Env,
    name: String,
    email: String,
    profession: Option<String>,
    goals: Option<String>,
    country: String,
    user: Address,
) -> UserProfile {
    
    // Validate required fields
    if name.is_empty() {
        panic!("User profile error: Name cannot be empty");
    }
    
    if email.is_empty() {
        panic!("User profile error: Email cannot be empty");
    }
    
    if country.is_empty() {
        panic!("User profile error: Country cannot be empty");
    }
    
    // Create the user profile
    let user_profile = UserProfile {
        name,
        email,
        profession,
        goals,
        country,
        user: user.clone(),
    };
    
    // Store the profile on-chain using the unique key
    let storage_key = DataKey::UserProfile(user);
    env.storage().persistent().set(&storage_key, &user_profile);
    
    user_profile
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Address, String, Env};
    use crate::schema::{UserProfile, DataKey};
    use crate::UserManagement;
    
    #[test]
    fn test_save_profile_success() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let name: String = String::from_str(&env, "John Doe");
        let email: String = String::from_str(&env, "john@example.com");
        let profession: Option<String> = Some(String::from_str(&env, "Software Engineer"));
        let goals: Option<String> = Some(String::from_str(&env, "Learn blockchain development"));
        let country: String = String::from_str(&env, "United States");
        
        env.as_contract(&contract_id, || {
            let profile = user_management_save_profile(
                env.clone(),
                name.clone(),
                email.clone(),
                profession.clone(),
                goals.clone(),
                country.clone(),
                user.clone(),
            );
            
            // Verify profile creation
            assert_eq!(profile.name, name);
            assert_eq!(profile.email, email);
            assert_eq!(profile.profession, profession);
            assert_eq!(profile.goals, goals);
            assert_eq!(profile.country, country);
            assert_eq!(profile.user, user);
            
            // Verify storage
            let storage_key = DataKey::UserProfile(user);
            let stored_profile: Option<UserProfile> = env.storage().persistent().get(&storage_key);
            let stored = stored_profile.expect("Profile should be stored");
            assert_eq!(stored, profile);
        });
    }
    
    #[test]
    fn test_save_profile_without_optional_fields() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let name: String = String::from_str(&env, "Jane Smith");
        let email: String = String::from_str(&env, "jane@example.com");
        let country: String = String::from_str(&env, "Canada");
        
        env.as_contract(&contract_id, || {
            let profile = user_management_save_profile(
                env.clone(),
                name.clone(),
                email.clone(),
                None, // profession
                None, // goals
                country.clone(),
                user.clone(),
            );
            
            // Verify profile creation
            assert_eq!(profile.name, name);
            assert_eq!(profile.email, email);
            assert_eq!(profile.profession, None);
            assert_eq!(profile.goals, None);
            assert_eq!(profile.country, country);
            assert_eq!(profile.user, user);
        });
    }
    
    #[test]
    #[should_panic(expected = "User profile error: Name cannot be empty")]
    fn test_save_profile_with_empty_name() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let name: String = String::from_str(&env, "");
        let email: String = String::from_str(&env, "test@example.com");
        let country: String = String::from_str(&env, "Germany");
        
        env.as_contract(&contract_id, || {
            user_management_save_profile(
                env.clone(),
                name,
                email,
                None,
                None,
                country,
                user,
            );
        });
    }
    
    #[test]
    #[should_panic(expected = "User profile error: Email cannot be empty")]
    fn test_save_profile_with_empty_email() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let name: String = String::from_str(&env, "Test User");
        let email: String = String::from_str(&env, "");
        let country: String = String::from_str(&env, "France");
        
        env.as_contract(&contract_id, || {
            user_management_save_profile(
                env.clone(),
                name,
                email,
                None,
                None,
                country,
                user,
            );
        });
    }
    
    #[test]
    #[should_panic(expected = "User profile error: Country cannot be empty")]
    fn test_save_profile_with_empty_country() {
        let env = Env::default();
        let contract_id: Address = env.register(UserManagement, {});
        let user: Address = Address::generate(&env);
        
        let name: String = String::from_str(&env, "Test User");
        let email: String = String::from_str(&env, "test@example.com");
        let country: String = String::from_str(&env, "");
        
        env.as_contract(&contract_id, || {
            user_management_save_profile(
                env.clone(),
                name,
                email,
                None,
                None,
                country,
                user,
            );
        });
    }
} 
use soroban_sdk::{Address, String, Env};
use crate::schema::UserProfile;
use crate::UserManagement;

#[test]
fn test_save_profile_integration() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let user: Address = Address::generate(&env);
    
    let name: String = String::from_str(&env, "Alice Johnson");
    let email: String = String::from_str(&env, "alice@example.com");
    let profession: Option<String> = Some(String::from_str(&env, "Data Scientist"));
    let goals: Option<String> = Some(String::from_str(&env, "Master machine learning"));
    let country: String = String::from_str(&env, "United Kingdom");
    
    env.as_contract(&contract_id, || {
        let profile = UserManagement::save_profile(
            env.clone(),
            name.clone(),
            email.clone(),
            profession.clone(),
            goals.clone(),
            country.clone(),
            user.clone(),
        );
        
        // Verify the returned profile
        assert_eq!(profile.name, name);
        assert_eq!(profile.email, email);
        assert_eq!(profile.profession, profession);
        assert_eq!(profile.goals, goals);
        assert_eq!(profile.country, country);
        assert_eq!(profile.user, user);
    });
}

#[test]
fn test_save_profile_minimal_data() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let user: Address = Address::generate(&env);
    
    let name: String = String::from_str(&env, "Bob Wilson");
    let email: String = String::from_str(&env, "bob@example.com");
    let country: String = String::from_str(&env, "Australia");
    
    env.as_contract(&contract_id, || {
        let profile = UserManagement::save_profile(
            env.clone(),
            name.clone(),
            email.clone(),
            None, // profession
            None, // goals
            country.clone(),
            user.clone(),
        );
        
        // Verify the returned profile
        assert_eq!(profile.name, name);
        assert_eq!(profile.email, email);
        assert_eq!(profile.profession, None);
        assert_eq!(profile.goals, None);
        assert_eq!(profile.country, country);
        assert_eq!(profile.user, user);
    });
} 
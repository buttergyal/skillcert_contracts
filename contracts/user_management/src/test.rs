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

    use soroban_sdk::{Address, Env, String};
use crate::{UserManagement, schema::{UserProfile, DataKey}};

#[test]
fn test_get_user_by_id_owner_and_admin() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let owner: Address = Address::generate(&env);
    let admin: Address = Address::generate(&env);
    let stranger: Address = Address::generate(&env);

    // Creamos un perfil
    let profile = UserProfile {
        name: String::from_str(&env, "Charlie Brown"),
        email: String::from_str(&env, "charlie@example.com"),
        profession: None,
        goals: None,
        country: String::from_str(&env, "USA"),
        user: owner.clone(),
    };

    // Guardamos perfil directamente en storage para el owner
    env.storage().set(&DataKey::UserProfile(owner.clone()), &profile);
    // Marcamos admin en storage
    env.storage().set(&DataKey::Admin(admin.clone()), &true);

    env.as_contract(&contract_id, || {
        // Owner accede a su perfil
        let p1 = UserManagement::get_user_by_id(env.clone(), owner.clone(), owner.clone());
        assert_eq!(p1.name, profile.name);
        assert_eq!(p1.email, profile.email);

        // Admin accede al perfil del owner
        let p2 = UserManagement::get_user_by_id(env.clone(), admin.clone(), owner.clone());
        assert_eq!(p2.name, profile.name);

        // Stranger intenta acceder y debe fallar con panic
        let result = std::panic::catch_unwind(|| {
            UserManagement::get_user_by_id(env.clone(), stranger.clone(), owner.clone());
        });
        assert!(result.is_err());
    });
}

#[test]
fn test_get_user_by_id_not_found() {
    let env = Env::default();
    let contract_id: Address = env.register(UserManagement, {});
    let admin: Address = Address::generate(&env);
    let missing_user: Address = Address::generate(&env);

    // Marcamos admin en storage
    env.storage().set(&DataKey::Admin(admin.clone()), &true);

    env.as_contract(&contract_id, || {
        // Intentamos obtener perfil que no existe, debe fallar con panic
        let result = std::panic::catch_unwind(|| {
            UserManagement::get_user_by_id(env.clone(), admin.clone(), missing_user.clone());
        });
        assert!(result.is_err());
    });
}

} 
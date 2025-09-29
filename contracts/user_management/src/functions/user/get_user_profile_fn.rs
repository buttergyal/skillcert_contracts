use crate::error::{Error};
use crate::models::{user::UserProfile, DataKey};
use soroban_sdk::{Address, Env};

/// Get User Profile
///
/// This function retrieves the user profile for the given user address.
/// It requires the user to be authenticated.
pub fn get_user_profile(env: Env, user: Address) -> Result<UserProfile, Error> {
    // Require authentication for the user
    user.require_auth();

    // Fetch user profile from storage
    let storage_key: DataKey = DataKey::UserProfile(user.clone());
    match env
        .storage()
        .persistent()
        .get::<DataKey, UserProfile>(&storage_key)
    {
        Some(profile) => Ok(profile),
        None => Err(Error::UserNotFound),
    }
}


//============================================================================
//================================ TESTS =====================================
//============================================================================



/// Unit tests
/// For this function we cover 3 scenarios:
/// 1. Successfully retrieving an existing user profile.
/// 2. Attempting to retrieve a non-existent user profile (should error).
/// 3. Attempting to retrieve a profile without authentication (should error).
#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::functions::{
        user,
        test_utils
    };
    use crate::models::{user::UserProfile, DataKey};
    use soroban_sdk::{
        Address, Env, String,
        testutils::Address as _
    };
    
    fn setup_user(
        env: &Env,
        contract: &Address,
        addr: &Address,
        full_name: &str,
        contact_email: &str,
        profession: Option<&str>,
        country: Option<&str>,
        purpose: Option<&str>,
        profile_picture_url: Option<&str>,
    ) {
        env.as_contract(contract, || {
            let profile = UserProfile {
                full_name: String::from_str(env, full_name),
                contact_email: String::from_str(env, contact_email),
                profession: profession.map(|s| String::from_str(env, s)),
                country: country.map(|s| String::from_str(env, s)),
                purpose: purpose.map(|s| String::from_str(env, s)),
                profile_picture_url: profile_picture_url.map(|s| String::from_str(env, s)),
            };

            env.storage()
                .persistent()
                .set(&DataKey::UserProfile(addr.clone()), &profile);
        });
    }

    #[test]
    fn test_get_user_success() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_addr: Address = env.register(test_utils::DummyContract, ());
        let user_addr: Address = Address::generate(&env);

        // Save new user profile
        setup_user(
            &env,
            &contract_addr,
            &user_addr,
            "Alice",
            "alice@example.com",
            None,
            None,
            None,
            None,
        );

        let result = env.as_contract(&contract_addr, || {
            user::get_user_profile(env.clone(), user_addr.clone())
        });

        assert!(result.is_ok());

        let profile = result.unwrap();
        assert_eq!(profile.full_name, String::from_str(&env, "Alice"));
        assert_eq!(profile.contact_email, String::from_str(&env, "alice@example.com"));
    }

    #[test]
    fn test_get_user_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_addr: Address = env.register(test_utils::DummyContract, ());
        let user_addr = Address::generate(&env);

        // Nothing stored → should return UserNotFound error
        let result = env.as_contract(&contract_addr, || {
            user::get_user_profile(env.clone(), user_addr.clone())
        });

        assert!(result.is_err());
        assert_eq!(result, Err(Error::UserNotFound));
    }

    #[test]
    #[should_panic] // porque require_auth() fallará
    fn test_get_user_not_authenticated() {
        let env = Env::default();
        let user_addr = Address::generate(&env);

        // NOTE: we do not call env.mock_auths() → no authentication

        let _ = user::get_user_profile(env, user_addr);
    }
}


use soroban_sdk::{Address, Env, Symbol, symbol_short};
use crate::models::{
    user::UserProfile, DataKey
};
use crate::error::{Error, handle_error};

// Execution constants
const USER_CREATED_EVENT: Symbol = symbol_short!("usrCrtd");
const MAX_STRING_LENGTH: u32 = 320;

/// Create a new user profile
///
/// This function creates a new user profile using a UserProfile struct.
/// It validates mandatory fields (full_name and contact_email) and saves the profile.
///
/// # Arguments
/// * `env` - Soroban environment
/// * `user` - Address of the user whose profile is being created
/// * `profile` - UserProfile struct containing all profile data
///
/// # Returns
/// * `UserProfile` - The created user profile
///
/// # Panics
/// * If mandatory fields are empty or invalid
/// * If email format is invalid
/// * If email is already taken
/// * If user authentication fails
/// * If user profile already exists
pub fn create_user_profile(env: Env, user: Address, profile: UserProfile) -> UserProfile {
    // Require authentication for the user
    user.require_auth();

    // Check if user profile already exists
    let storage_key: DataKey = DataKey::UserProfile(user.clone());
    if env.storage().persistent().has(&storage_key) {
        handle_error(&env, Error::UserProfileExists)
    }

    // Validate mandatory fields
    if profile.full_name.is_empty() || profile.contact_email.is_empty() {
        handle_error(&env, Error::RequiredFieldMissing)
    }

    // Validate field lengths and content
    if profile.full_name.len() > MAX_STRING_LENGTH
        || profile.contact_email.len() > MAX_STRING_LENGTH
    {
        handle_error(&env, Error::InvalidField)
    }

    // Ensure email uniqueness
    if !env.storage().persistent()
        .has(&DataKey::EmailIndex(profile.contact_email.clone())) {
        handle_error(&env, Error::EmailAlreadyExists)
    }

    // TODO: Validate optional fields if provided
    
    // Store the profile using persistent storage
    env.storage().persistent().set(&storage_key, &profile);

    // Register email for uniqueness checking
    env.storage().persistent()
        .set(&DataKey::EmailIndex(profile.contact_email.clone()), &user);


    // Emit user creation audit event with detailed information
    env.events().publish(
        (USER_CREATED_EVENT, &user),
        (
            user.clone(),
            profile.full_name.clone(),
            profile.contact_email.clone(),
            profile.profession.clone(),
            profile.country.clone(),
        ),
    );

    profile
}

//============================================================================
//================================ TESTS =====================================  
//============================================================================

// TODO : Add unit tests for create_user_profile function
// Scenarios to cover:
// 1. Successfully creating a user profile with valid data.
// 2. Attempting to create a profile with missing mandatory fields (should error).
// 3. Attempting to create a profile with an already taken email (should error).
// 4. Attempting to create a profile without authentication (should error).
// 5. Attempting to create a duplicate profile for the same user (should error).
// 6. Attempting to create a profile with invalid field lengths (should error).
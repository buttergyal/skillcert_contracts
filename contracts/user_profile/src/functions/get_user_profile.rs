use soroban_sdk::{Env, Address, Symbol};

use crate::schema::UserProfile;

pub fn user_profile_get_user_profile(env: &Env, user_address: Address) -> UserProfile {
    // Create the storage key for the user profile
    let key = Symbol::new(env, "profile");
    
    // Get the user profile from storage
    let profile: UserProfile = env
        .storage()
        .instance()
        .get(&(key, user_address.clone()))
        .expect("User profile not found");
    
    profile
}

// Function to get user profile with privacy check
// Returns profile only if it's public or if the requester is the profile owner
pub fn user_profile_get_user_profile_with_privacy(
    env: &Env, 
    user_address: Address, 
    requester_address: Address
) -> UserProfile {
    // Create the storage key for the user profile
    let key = Symbol::new(env, "profile");
    
    // Get the user profile from storage
    let mut profile: UserProfile = env
        .storage()
        .instance()
        .get(&(key, user_address.clone()))
        .expect("User profile not found");
    
    // Check privacy settings
    // If profile is not public and requester is not the profile owner, hide email
    if !profile.privacy_public && requester_address != user_address {
        profile.email = None;
    }
    
    profile
}

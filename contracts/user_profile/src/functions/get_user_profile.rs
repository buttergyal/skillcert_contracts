// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Address, Env, Symbol};

use crate::schema::UserProfile;

 validate-input-parameter
pub fn user_profile_get_user_profile(env: &Env, user_address: Address) -> UserProfile {
    // Input validation
    // If Address type supports is_empty or similar, add check. Otherwise, skip.
    // For demonstration, assume Address cannot be empty.

pub fn get_user_profile(env: &Env, user_address: Address) -> UserProfile {
    let key = Symbol::new(env, "profile");
    let storage_key = (key, user_address.clone());
    
    // Try temporary storage first for frequently accessed profiles
    if let Some(profile) = env.storage().temporary().get(&storage_key) {
        return profile;
    }

    // Get from instance storage if not cached
    let profile: UserProfile = env
        .storage()
        .instance()
        .get(&storage_key)
        .expect("User profile not found");

    // Cache in temporary storage for subsequent requests
    env.storage().temporary().set(&storage_key, &profile);
    // Cache for 15 minutes
    env.storage().temporary().extend_ttl(&storage_key, 0, 900);

    profile
}

// Function to get user profile with privacy check
// Returns profile only if it's public or if the requester is the profile owner
pub fn get_user_profile_with_privacy(
    env: &Env,
    user_address: Address,
    requester_address: Address,
) -> UserProfile {
    // Reuse the optimized get_user_profile function
    let mut profile = get_user_profile(env, user_address.clone());

    // Apply privacy filters without additional storage reads
    if !profile.privacy_public && requester_address != user_address {
        profile.email = None;
        // Add more privacy filters as needed
        profile.phone = None;
        profile.address_details = None;
    }

    profile
}

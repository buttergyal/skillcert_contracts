// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Env, Address, Symbol};

use crate::schema::UserProfile;
use crate::error::{Error, handle_error};

pub fn user_profile_get_user_profile(env: &Env, user_address: Address) -> UserProfile {
    // Create the storage key for the user profile
    let key = Symbol::new(env, "profile");
    
    // Get the user profile from storage with proper error handling
    match env
        .storage()
        .instance()
        .get::<(Symbol, Address), UserProfile>(&(key, user_address.clone()))
    {
        Some(profile) => profile,
        None => handle_error(env, Error::UserProfileNotFound),
    }
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
    
    // Get the user profile from storage with proper error handling
    let mut profile: UserProfile = match env
        .storage()
        .instance()
        .get::<(Symbol, Address), UserProfile>(&(key, user_address.clone()))
    {
        Some(profile) => profile,
        None => handle_error(env, Error::UserProfileNotFound),
    };
    
    // Check privacy settings
    // If profile is not public and requester is not the profile owner, hide email
    if !profile.privacy_public && requester_address != user_address {
        profile.email = None;
    }
    
    profile
}

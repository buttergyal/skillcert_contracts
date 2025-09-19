// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{AdminConfig, DataKey, UserProfile, UserRole, UserStatus};
use soroban_sdk::{Address, Env, Map, String, Symbol, Vec};

const USER_PROFILE_KEY: Symbol = symbol_short!("user_profile");
const EMAIL_INDEX_KEY: Symbol = symbol_short!("email_index");
const USER_TTL: u32 = 26_298_000; // ~10 months
const USER_BUMP: u32 = 518_400; // ~6 days

/// Efficiently retrieve a user profile with caching
pub fn get_user_profile(
    env: &Env,
    user_address: &Address,
) -> Option<UserProfile> {
    // Check temporary storage first
    let temp_key = (USER_PROFILE_KEY, user_address.clone());
    if let Some(profile) = env.storage().temporary().get(&temp_key) {
        return Some(profile);
    }

    // If not in temporary storage, check persistent storage
    let profile = env.storage().persistent().get(&temp_key);
    
    // Cache the profile in temporary storage if found
    if let Some(ref p) = profile {
        env.storage().temporary().set(&temp_key, p);
    }

    profile
}

/// Save user profile with proper TTL management and email indexing
pub fn save_user_profile(
    env: &Env,
    profile: &UserProfile,
) {
    let user_key = (USER_PROFILE_KEY, profile.address.clone());
    let email_key = (EMAIL_INDEX_KEY, profile.email.clone());

    // Update email index
    env.storage().persistent().set(&email_key, &profile.address);
    env.storage().persistent().extend_ttl(&email_key, USER_BUMP, USER_TTL);

    // Save profile
    env.storage().persistent().set(&user_key, profile);
    env.storage().persistent().extend_ttl(&user_key, USER_BUMP, USER_TTL);

    // Update cache
    env.storage().temporary().set(&user_key, profile);
}

/// Check if email is already registered
pub fn is_email_registered(
    env: &Env,
    email: &String,
    exclude_user: Option<&Address>,
) -> bool {
    let email_key = (EMAIL_INDEX_KEY, email.clone());
    if let Some(existing_user) = env.storage().persistent().get::<_, Address>(&email_key) {
        if let Some(exclude) = exclude_user {
            return &existing_user != exclude;
        }
        true
    } else {
        false
    }
}

/// Get filtered users with pagination support
pub fn get_filtered_users(
    env: &Env,
    page: u32,
    page_size: u32,
    role_filter: Option<UserRole>,
    country_filter: Option<String>,
    status_filter: Option<UserStatus>,
) -> Vec<UserProfile> {
    let mut users = Vec::new(env);
    let mut user_map = Map::new(env);

    // Collect all users matching filters
    env.storage().persistent().find(
        USER_PROFILE_KEY,
        |_, profile: UserProfile| {
            if matches_filters(&profile, &role_filter, &country_filter, &status_filter) {
                user_map.set(profile.address.clone(), profile);
            }
        },
    );

    // Apply pagination
    let start = page * page_size;
    let end = start + page_size;
    let mut current = 0;

    for (_, profile) in user_map.iter() {
        if current >= start && current < end {
            users.push_back(profile);
        }
        current += 1;
        if current >= end {
            break;
        }
    }

    users
}

fn matches_filters(
    profile: &UserProfile,
    role_filter: &Option<UserRole>,
    country_filter: &Option<String>,
    status_filter: &Option<UserStatus>,
) -> bool {
    if let Some(role) = role_filter {
        if &profile.role != role {
            return false;
        }
    }
    if let Some(country) = country_filter {
        if &profile.country != country {
            return false;
        }
    }
    if let Some(status) = status_filter {
        if &profile.status != status {
            return false;
        }
    }
    true
}
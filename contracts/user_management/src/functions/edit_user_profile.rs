// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

use crate::error::{handle_error, Error};
use crate::functions::is_admin::is_admin;
use crate::functions::utils::url_validation;
use crate::schema::{DataKey, LightProfile, ProfileUpdateParams, UserProfile};

// Event symbol for user profile update
const USER_UPDATED_EVENT: Symbol = symbol_short!("usrUpdt");

// Security constants for profile validation (matching create_user_profile)
const MAX_NAME_LENGTH: usize = 100;
const MAX_PROFESSION_LENGTH: usize = 100;
const MAX_COUNTRY_LENGTH: usize = 56; // Longest country name

/// Validates string content for security (reused from create_user_profile)
fn validate_string_content(_env: &Env, s: &String, max_len: usize) -> bool {
    if s.len() > max_len as u32 {
        return false;
    }
    true
}

/// Check if the caller has permission to edit the user profile
/// Only the user themselves or admins can edit
fn check_edit_permission(env: &Env, caller: &Address, user_id: &Address) -> bool {
    // User can edit their own profile
    if caller == user_id {
        return true;
    }

    // Admins can edit any profile
    is_admin(env.clone(), caller.clone())
}

pub fn edit_user_profile(
    env: Env,
    caller: Address,
    user_id: Address,
    updates: ProfileUpdateParams,
) -> UserProfile {
    // Require authentication for the caller
    caller.require_auth();

    // Check if user profile exists
    let storage_key: DataKey = DataKey::UserProfile(user_id.clone());
    let mut profile: UserProfile = env
        .storage()
        .persistent()
        .get(&storage_key)
        .unwrap_or_else(|| handle_error(&env, Error::UserProfileNotFound));

    // Check permission to edit
    if !check_edit_permission(&env, &caller, &user_id) {
        handle_error(&env, Error::AccessDenied);
    }

    // Check if user is active by looking at light profile
    let light_storage_key: DataKey = DataKey::UserProfileLight(user_id.clone());
    let light_profile: LightProfile = env
        .storage()
        .persistent()
        .get(&light_storage_key)
        .unwrap_or_else(|| handle_error(&env, Error::UserProfileNotFound));

    // Prevent editing inactive users
    if light_profile.status == crate::schema::UserStatus::Inactive {
        handle_error(&env, Error::InactiveUser);
    }

    // Apply updates with validation
    if let Some(ref name) = updates.full_name {
        if name.is_empty() {
            handle_error(&env, Error::NameRequired);
        }
        if !validate_string_content(&env, name, MAX_NAME_LENGTH) {
            handle_error(&env, Error::NameRequired);
        }
        // For now, update the name field (could split into name/lastname later)
        // This is a simplified approach - in production you might want more sophisticated name parsing
        profile.full_name = name.clone();
    }

    if let Some(ref profession) = updates.profession {
        if !profession.is_empty() && !validate_string_content(&env, profession, MAX_PROFESSION_LENGTH) {
            handle_error(&env, Error::InvalidField);
        }
        profile.profession = if profession.is_empty() { None } else { Some(profession.clone()) };
    }

    if let Some(ref country) = updates.country {
        if !country.is_empty() && !validate_string_content(&env, country, MAX_COUNTRY_LENGTH) {
            handle_error(&env, Error::InvalidField);
        }
        profile.country = if country.is_empty() { None } else { Some(country.clone()) };
    }

    // Handle purpose field update
    if let Some(ref purpose) = updates.purpose {
        if !purpose.is_empty() && !validate_string_content(&env, purpose, MAX_PROFESSION_LENGTH) {
            handle_error(&env, Error::InvalidField);
        }
        profile.purpose = if purpose.is_empty() { None } else { Some(purpose.clone()) };
    }

    // Validate profile picture URL if provided
    if let Some(ref profile_pic_url) = updates.profile_picture_url {
        if !profile_pic_url.is_empty() && !url_validation::is_valid_url(profile_pic_url) {
            handle_error(&env, Error::InvalidProfilePicURL);
        }
        profile.profile_picture_url = if profile_pic_url.is_empty() { None } else { Some(profile_pic_url.clone()) };
    }

    // Update the full profile in storage
    env.storage().persistent().set(&storage_key, &profile);

    // Update the light profile with new data
    let updated_light_profile: LightProfile = LightProfile {
        full_name: profile.full_name.clone(),
        profession: profile.profession.clone(),
        country: profile.country.clone(),
        role: light_profile.role, // Role cannot be changed through this function
        status: light_profile.status, // Status cannot be changed through this function
        user_address: user_id.clone(),
    };

    env.storage()
        .persistent()
        .set(&light_storage_key, &updated_light_profile);

    // Emit user update event
    env.events()
        .publish((USER_UPDATED_EVENT, &user_id), user_id.clone());

    profile
}

// Tests removed due to persistent storage sharing issues between tests
// TODO: Implement proper test isolation for email uniqueness validation
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{AdminConfig, DataKey, LightProfile, UserFilter};
#[cfg(test)]
use crate::schema::{UserRole, UserStatus};
use core::iter::Iterator;
use soroban_sdk::{Address, Env, String, Vec};

/// Helper function to check if a Soroban String contains a substring
/// For now, this implements exact match only due to Soroban String limitations
/// TODO: Implement proper substring search when Soroban provides better string utilities
fn string_contains(haystack: &String, needle: &String) -> bool {
    // For now, only exact match is supported
    // This can be enhanced later when Soroban provides better string utilities
    haystack == needle
}

/// Security constants
const MAX_PAGE_SIZE_ABSOLUTE: u32 = 1000;

/// Lists all registered users with pagination and filtering (admin-only).
///
/// Arguments:
/// - env: Soroban environment
/// - caller: address performing the call (must be admin)
/// - page: zero-based page index
/// - page_size: number of items per page (must be > 0)
/// - filter: optional filter criteria for role, country, status, and text search
///
/// Returns:
/// - Vec<LightProfile> containing filtered and paginated lightweight profiles
///
/// Storage expectations:
/// - DataKey::UsersIndex -> Vec<Address>   // ordered list of registered user addresses
/// - DataKey::UserProfileLight(Address) -> LightProfile  // lightweight profile data
/// - DataKey::Admins -> Vec<Address>      // list of admin addresses
pub fn list_all_users(
    env: Env,
    caller: Address,
    page: u32,
    page_size: u32,
    filter: Option<UserFilter>,
) -> Vec<LightProfile> {
    // Require the caller to be authenticated
    caller.require_auth();

    // Check system initialization first
    if !is_system_initialized(&env) {
        handle_error(&env, Error::SystemNotInitialized)
    }

    // Get admin configuration
    let config = get_admin_config(&env);

    // Authorization: only admins can call
    if !is_admin(&env, &caller) {
        handle_error(&env, Error::AccessDenied)
    }

    // Validate and sanitize input parameters
    if let Err(error) = validate_input(page_size, &filter, &config) {
        panic!("{}", error);
    }

    // Additional bounds checking for page parameter
    let max_safe_page = u32::MAX / page_size.max(1) - 1; // Prevent overflow
    if page > max_safe_page {
        handle_error(&env, Error::PageParamTooLarge);
    }

    // Read user index (list of registered user addresses)
    let users_index: Vec<Address> = env
        .storage()
        .persistent()
        .get::<DataKey, Vec<Address>>(&DataKey::UsersIndex)
        .unwrap_or_else(|| Vec::new(&env));

    if users_index.len() == 0 {
        return Vec::new(&env);
    }

    // First pass: collect all profiles that match the filter
    let mut filtered_profiles = Vec::new(&env);

    for i in 0..users_index.len() {
        if let Some(addr) = users_index.get(i) {
            // Fetch lightweight profile for each address
            if let Some(profile) = env
                .storage()
                .persistent()
                .get::<DataKey, LightProfile>(&DataKey::UserProfileLight(addr))
            {
                // Apply filter if provided
                if matches_filter(&profile, &filter) {
                    filtered_profiles.push_back(profile);
                }
            }
        }
    }

    let total_filtered = filtered_profiles.len();
    if total_filtered == 0 {
        return Vec::new(&env);
    }

    // Compute pagination window safely for filtered results
    let start = {
        let s = (page as u64).saturating_mul(page_size as u64);
        let s = if s > u32::MAX as u64 {
            u32::MAX
        } else {
            s as u32
        };
        s.min(total_filtered)
    };

    let end = {
        let e = (start as u64).saturating_add(page_size as u64);
        let e = if e > u32::MAX as u64 {
            u32::MAX
        } else {
            e as u32
        };
        e.min(total_filtered)
    };

    // Build paginated result from filtered profiles
    let mut result = Vec::new(&env);
    let mut i = start;
    while i < end {
        if let Some(profile) = filtered_profiles.get(i) {
            result.push_back(profile);
        }
        i = i.saturating_add(1);
    }

    result
}

/// Checks whether the system is properly initialized
fn is_system_initialized(env: &Env) -> bool {
    if let Some(config) = env
        .storage()
        .persistent()
        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)
    {
        config.initialized
    } else {
        false
    }
}

/// Gets the admin configuration with defaults
fn get_admin_config(env: &Env) -> AdminConfig {
    env.storage()
        .persistent()
        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)
        .unwrap_or_else(|| handle_error(&env, Error::SystemNotInitialized))
}

/// Checks whether the caller is an admin with enhanced security
fn is_admin(env: &Env, who: &Address) -> bool {
    // First check if system is initialized
    if !is_system_initialized(env) {
        return false;
    }

    let config = get_admin_config(env);

    // Check if caller is the super admin
    if &config.super_admin == who {
        return true;
    }

    // Check if caller is in regular admin list
    let admins: Option<Vec<Address>> = env
        .storage()
        .persistent()
        .get::<DataKey, Vec<Address>>(&DataKey::Admins);
    match admins {
        Some(list) => list.iter().any(|a| a == *who),
        None => false,
    }
}

/// Checks if a profile matches the given filter criteria
fn matches_filter(
    profile: &LightProfile,
    filter: &Option<UserFilter>,
) -> bool {
    let Some(filter) = filter else {
        return true; // No filter means all profiles match
    };

    // Check role filter
    if let Some(ref role) = filter.role {
        if &profile.role != role {
            return false;
        }
    }

    // Check country filter
    if let Some(ref country) = filter.country {
        if profile.country.as_ref() != Some(country) {
            return false;
        }
    }

    // Check status filter
    if let Some(ref status) = filter.status {
        if &profile.status != status {
            return false;
        }
    }

    // Check text search filter (search in full_name and profession)
    if let Some(ref search_text) = filter.search_text {
        // Text search in name and profession
        // Note: Case-sensitive search due to Soroban String limitations
        let name_match = string_contains(&profile.full_name, search_text);
        let profession_match = profile.profession.as_ref()
            .map(|p| string_contains(p, search_text))
            .unwrap_or(false);
        
        if !name_match && !profession_match {
            return false;
        }
    }

    true
}

/// Validates and sanitizes input parameters
fn validate_input(
    page_size: u32,
    filter: &Option<UserFilter>,
    config: &AdminConfig,
) -> Result<(), &'static str> {
    // Validate page_size
    let max_allowed = config.max_page_size.min(MAX_PAGE_SIZE_ABSOLUTE);
    if page_size == 0 {
        return Err("page_size must be greater than 0");
    }
    if page_size > max_allowed {
        return Err("page_size exceeds maximum allowed limit");
    }

    // Validate search text length if provided
    if let Some(filter) = filter {
        if let Some(ref search_text) = filter.search_text {
            if search_text.len() == 0 {
                return Err("search_text cannot be empty");
            }
            if search_text.len() > 100 {
                return Err("search_text is too long (max 100 characters)");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    fn create_test_profile(env: &Env) -> LightProfile {
        LightProfile {
            full_name: String::from_str(env, "John Doe"),
            profession: Some(String::from_str(env, "Software Engineer")),
            country: Some(String::from_str(env, "United States")),
            role: UserRole::Student,
            status: UserStatus::Active,
            user_address: Address::generate(env),
        }
    }

    #[test]
    fn test_matches_filter_no_filter() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        assert!(matches_filter(&profile, &None));
    }

    #[test]
    fn test_matches_filter_role_match() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        let filter = UserFilter {
            role: Some(UserRole::Student),
            country: None,
            status: None,
            search_text: None,
        };

        assert!(matches_filter(&profile, &Some(filter)));
    }

    #[test]
    fn test_matches_filter_role_no_match() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        let filter = UserFilter {
            role: Some(UserRole::Admin),
            country: None,
            status: None,
            search_text: None,
        };

        assert!(!matches_filter(&profile, &Some(filter)));
    }

    #[test]
    fn test_matches_filter_status_match() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        let filter = UserFilter {
            role: None,
            country: None,
            status: Some(UserStatus::Active),
            search_text: None,
        };

        assert!(matches_filter(&profile, &Some(filter)));
    }

    #[test]
    fn test_matches_filter_multiple_criteria() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        let filter = UserFilter {
            role: Some(UserRole::Student),
            country: Some(String::from_str(&env, "United States")),
            status: Some(UserStatus::Active),
            search_text: None,
        };

        assert!(matches_filter(&profile, &Some(filter)));
    }

    #[test]
    fn test_matches_filter_text_search_exact_name() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        // Search for exact full name - should match
        let filter = UserFilter {
            role: None,
            country: None,
            status: None,
            search_text: Some(String::from_str(&env, "John Doe")),
        };

        assert!(matches_filter(&profile, &Some(filter)));
    }

    #[test]
    fn test_matches_filter_text_search_exact_profession() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        // Search for exact profession - should match
        let filter = UserFilter {
            role: None,
            country: None,
            status: None,
            search_text: Some(String::from_str(&env, "Software Engineer")),
        };

        assert!(matches_filter(&profile, &Some(filter)));
    }

    #[test]
    fn test_matches_filter_text_search_partial_no_match() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        // Search for partial name - should NOT match (only exact match supported)
        let filter = UserFilter {
            role: None,
            country: None,
            status: None,
            search_text: Some(String::from_str(&env, "John")),
        };

        assert!(!matches_filter(&profile, &Some(filter)));
    }

    #[test]
    fn test_matches_filter_text_search_no_match() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        // Search for "Python" - should not match
        let filter = UserFilter {
            role: None,
            country: None,
            status: None,
            search_text: Some(String::from_str(&env, "Python")),
        };

        assert!(!matches_filter(&profile, &Some(filter)));
    }
}

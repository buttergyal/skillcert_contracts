use crate::schema::{AdminConfig, DataKey, LightProfile, UserRole, UserStatus};
use core::iter::Iterator;
use soroban_sdk::{Address, Env, String, Vec};

/// Security constants
const MAX_PAGE_SIZE_ABSOLUTE: u32 = 1000;

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
        .unwrap_or_else(|| panic!("System not initialized"))
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
    role_filter: &Option<UserRole>,
    _country_filter: &Option<String>, // Deprecated but kept for API compatibility
    status_filter: &Option<UserStatus>,
) -> bool {
    // Check role filter
    if let Some(ref role) = role_filter {
        if &profile.role != role {
            return false;
        }
    }

    // Note: Country filter was removed from the new schema as LightProfile
    // no longer has a 'country' field. The parameter is kept for backward compatibility
    // but the actual filtering is disabled.

    // Check status filter
    if let Some(ref status) = status_filter {
        if &profile.status != status {
            return false;
        }
    }

    true
}

/// Validates and sanitizes input parameters
fn validate_input(
    page_size: u32,
    _country_filter: &Option<String>, // Deprecated but kept for API compatibility
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

    // Country filter validation is no longer needed as it's deprecated
    // but we keep the parameter for backward compatibility

    Ok(())
}

/// Lists all registered users with pagination and filtering (admin-only).
///
/// Arguments:
/// - env: Soroban environment
/// - caller: address performing the call (must be admin)
/// - page: zero-based page index
/// - page_size: number of items per page (must be > 0)
/// - filter: optional filter criteria for role, country, and status
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
    role_filter: Option<UserRole>,
    country_filter: Option<String>,
    status_filter: Option<UserStatus>,
) -> Vec<LightProfile> {
    // Require the caller to be authenticated
    caller.require_auth();

    // Check system initialization first
    if !is_system_initialized(&env) {
        panic!("System not initialized");
    }

    // Get admin configuration
    let config = get_admin_config(&env);

    // Authorization: only admins can call
    if !is_admin(&env, &caller) {
        panic!("Access denied");
    }

    // Validate and sanitize input parameters
    if let Err(error) = validate_input(page_size, &country_filter, &config) {
        panic!("{}", error);
    }

    // Additional bounds checking for page parameter
    let max_safe_page = u32::MAX / page_size.max(1) - 1; // Prevent overflow
    if page > max_safe_page {
        panic!("Page parameter too large");
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
                if matches_filter(&profile, &role_filter, &country_filter, &status_filter) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String, Vec};

    fn create_test_profile(env: &Env) -> LightProfile {
        LightProfile {
            name: String::from_str(env, "John"),
            lastname: String::from_str(env, "Doe"),
            specialization: String::from_str(env, "Software Engineering"),
            languages: Vec::from_array(env, [String::from_str(env, "English")]),
            teaching_categories: Vec::from_array(env, [String::from_str(env, "Programming")]),
            role: UserRole::Student,
            status: UserStatus::Active,
            user_address: Address::generate(env),
        }
    }

    #[test]
    fn test_matches_filter_no_filter() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        assert!(matches_filter(&profile, &None, &None, &None));
    }

    #[test]
    fn test_matches_filter_role_match() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        assert!(matches_filter(
            &profile,
            &Some(UserRole::Student),
            &None,
            &None
        ));
    }

    #[test]
    fn test_matches_filter_role_no_match() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        assert!(!matches_filter(
            &profile,
            &Some(UserRole::Admin),
            &None,
            &None
        ));
    }

    #[test]
    fn test_matches_filter_status_match() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        assert!(matches_filter(
            &profile,
            &None,
            &None,
            &Some(UserStatus::Active)
        ));
    }

    #[test]
    fn test_matches_filter_multiple_criteria() {
        let env = Env::default();
        let profile = create_test_profile(&env);

        assert!(matches_filter(
            &profile,
            &Some(UserRole::Student),
            &None, // Country filter is deprecated but kept for compatibility
            &Some(UserStatus::Active)
        ));
    }
}
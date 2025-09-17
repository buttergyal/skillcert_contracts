// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{AdminConfig, DataKey, LightProfile, UserProfile, UserStatus, DEFAULT_MAX_PAGE_SIZE};
use core::iter::Iterator;
use soroban_sdk::{symbol_short, Address, Env, Symbol};

// Event symbol for user deactivation
const EVT_USER_DEACTIVATED: Symbol = symbol_short!("usr_deact");

/// Delete (deactivate) a user account
///
/// This function performs a soft delete by marking the user as inactive instead of
/// permanently removing their data. Only admins or the user themselves can trigger deletion.
///
/// # Arguments
/// * `env` - Soroban environment
/// * `caller` - Address performing the deletion (must be admin or the user themselves)
/// * `user_id` - Address of the user to be deactivated
///
/// # Panics
/// * If caller authentication fails
/// * If user doesn't exist
/// * If caller is neither admin nor the user themselves
/// * If user is already inactive
///
/// # Events
/// Emits a user deactivation event upon successful deletion
pub fn delete_user(env: Env, caller: Address, user_id: Address) -> () {
    // Require authentication for the caller
    caller.require_auth();

    // DEPENDENCY: Validate that user exists (using user existence validation)
    let _user_profile = validate_user_exists(&env, &user_id)
        .unwrap_or_else(|_| handle_error(&env, Error::UserNotFound));

    // Authorization: only admin or the user themselves can trigger deletion
    let is_caller_admin = is_admin(&env, &caller);
    let is_self_deletion = caller == user_id;

    if !is_caller_admin && !is_self_deletion {
        handle_error(&env, Error::AccessDenied)
    }

    // Check current user status from light profile
    let light_profile_key = DataKey::UserProfileLight(user_id.clone());
    let mut light_profile: LightProfile = env
        .storage()
        .persistent()
        .get(&light_profile_key)
        .unwrap_or_else(|| handle_error(&env, Error::UserProfileNotFound));

    // Check if user is already inactive
    if light_profile.status == UserStatus::Inactive {
        handle_error(&env, Error::InactiveUser)
    }

    // Perform soft delete: mark user as inactive
    light_profile.status = UserStatus::Inactive;

    // Update the light profile with new status
    env.storage()
        .persistent()
        .set(&light_profile_key, &light_profile);

    // Note: We keep the full UserProfile intact for potential future reactivation
    // Only the status in LightProfile is changed to Inactive

    // Emits a user deactivation event upon successful deletion.
    env.events()
        .publish((EVT_USER_DEACTIVATED, &caller), user_id.clone());
}

/// Check if the caller is an admin
fn is_admin(env: &Env, who: &Address) -> bool {
    // Check if system is initialized
    let config: Option<AdminConfig> = env.storage().persistent().get(&DataKey::AdminConfig);
    match config {
        Some(cfg) if cfg.initialized => {
            // Check if caller is super admin
            if &cfg.super_admin == who {
                return true;
            }

            // Check regular admin list
            let admins: Option<soroban_sdk::Vec<Address>> =
                env.storage()
                    .persistent()
                    .get::<DataKey, soroban_sdk::Vec<Address>>(&DataKey::Admins);
            match admins {
                Some(list) => list.iter().any(|a| a == *who),
                None => false,
            }
        }
        _ => false,
    }
}

/// Validate that a user exists by checking their profile
fn validate_user_exists(env: &Env, user_id: &Address) -> Result<UserProfile, ()> {
    env.storage()
        .persistent()
        .get::<DataKey, UserProfile>(&DataKey::UserProfile(user_id.clone()))
        .ok_or(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{LightProfile, UserProfile, UserRole, UserStatus};
    use crate::{UserManagement, UserManagementClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn setup_test_env() -> (Env, Address, UserManagementClient<'static>) {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        (env, contract_id, client)
    }

    fn create_test_user(env: &Env, contract_id: &Address, user: &Address) -> UserProfile {
        // Create user profile directly in storage for testing
        let user_profile = UserProfile {
            full_name: String::from_str(env, "Test User"),
            contact_email: String::from_str(env, "test@example.com"),
            profession: Some(String::from_str(env, "Software Tester")),
            country: Some(String::from_str(env, "United States")),
            purpose: Some(String::from_str(env, "Learn testing methodologies")),
        };

        let light_profile = LightProfile {
            full_name: String::from_str(env, "Test User"),
            profession: Some(String::from_str(env, "Software Tester")),
            country: Some(String::from_str(env, "United States")),
            role: UserRole::Student,
            status: UserStatus::Active,
            user_address: user.clone(),
        };

        env.as_contract(contract_id, || {
            env.storage()
                .persistent()
                .set(&DataKey::UserProfile(user.clone()), &user_profile);
            env.storage()
                .persistent()
                .set(&DataKey::UserProfileLight(user.clone()), &light_profile);
        });

        user_profile
    }

    fn setup_admin(env: &Env, contract_id: &Address, admin: &Address) {
        env.as_contract(contract_id, || {
            let config = AdminConfig {
                initialized: true,
                super_admin: admin.clone(),
                max_page_size: DEFAULT_MAX_PAGE_SIZE,
                total_user_count: 0,
            };
            env.storage()
                .persistent()
                .set(&DataKey::AdminConfig, &config);
        });
    }

    #[test]
    fn test_delete_user_by_admin_success() {
        let (env, contract_id, client) = setup_test_env();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        // Setup admin and user
        setup_admin(&env, &contract_id, &admin);
        create_test_user(&env, &contract_id, &user);

        env.mock_all_auths();

        // Admin deletes user
        client.delete_user(&admin, &user);

        // Verify user is marked as inactive
        env.as_contract(&contract_id, || {
            let light_profile: LightProfile = env
                .storage()
                .persistent()
                .get(&DataKey::UserProfileLight(user.clone()))
                .expect("Light profile should exist");

            assert_eq!(light_profile.status, UserStatus::Inactive);

            // Verify full profile still exists (soft delete)
            let full_profile: UserProfile = env
                .storage()
                .persistent()
                .get(&DataKey::UserProfile(user.clone()))
                .expect("Full profile should still exist");

            // Profile should still exist with the same data
            assert_eq!(full_profile.full_name, String::from_str(&env, "Test User"));
        });
    }

    #[test]
    fn test_delete_user_by_self_success() {
        let (env, contract_id, _client) = setup_test_env();
        let user = Address::generate(&env);

        // Create user
        create_test_user(&env, &contract_id, &user);

        env.mock_all_auths();

        // User deletes themselves
        env.as_contract(&contract_id, || {
            delete_user(env.clone(), user.clone(), user.clone());
        });

        // Verify user is marked as inactive
        env.as_contract(&contract_id, || {
            let light_profile: LightProfile = env
                .storage()
                .persistent()
                .get(&DataKey::UserProfileLight(user.clone()))
                .expect("Light profile should exist");

            assert_eq!(light_profile.status, UserStatus::Inactive);
        });
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #4)")]
    fn test_delete_user_unauthorized() {
        let (env, contract_id, client) = setup_test_env();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        // Create both users
        create_test_user(&env, &contract_id, &user1);
        create_test_user(&env, &contract_id, &user2);

        env.mock_all_auths();

        // user1 tries to delete user2 (should fail)
        client.delete_user(&user1, &user2);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #20)")]
    fn test_delete_nonexistent_user() {
        let (env, contract_id, client) = setup_test_env();
        let admin = Address::generate(&env);
        let nonexistent_user = Address::generate(&env);

        setup_admin(&env, &contract_id, &admin);

        env.mock_all_auths();

        // Try to delete non-existent user
        client.delete_user(&admin, &nonexistent_user);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #22)")]
    fn test_delete_already_inactive_user() {
        let (env, contract_id, client) = setup_test_env();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        setup_admin(&env, &contract_id, &admin);
        create_test_user(&env, &contract_id, &user);

        // First, deactivate the user
        env.as_contract(&contract_id, || {
            let mut light_profile: LightProfile = env
                .storage()
                .persistent()
                .get(&DataKey::UserProfileLight(user.clone()))
                .expect("Light profile should exist");

            light_profile.status = UserStatus::Inactive;
            env.storage()
                .persistent()
                .set(&DataKey::UserProfileLight(user.clone()), &light_profile);
        });

        env.mock_all_auths();

        // Try to delete already inactive user
        client.delete_user(&admin, &user);
    }
}

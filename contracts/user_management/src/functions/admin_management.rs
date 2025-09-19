// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{
    AdminConfig, DataKey, ABSOLUTE_MAX_PAGE_SIZE, DEFAULT_MAX_PAGE_SIZE, MAX_ADMINS,
};
use core::iter::Iterator;
use soroban_sdk::{Address, Env, Vec};

/// Initialize the admin system - can only be called once
pub fn initialize_system(
    env: Env,
    initializer: Address,
    super_admin: Address,
    max_page_size: Option<u32>,
) -> AdminConfig {
    initializer.require_auth();

    // Check if system is already initialized
    if let Some(existing_config) = env
        .storage()
        .persistent()
        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)
    {
        if existing_config.initialized {
            handle_error(&env, Error::AlreadInitialized)
        }
    }

    // Validate max_page_size
    let validated_max_page_size = match max_page_size {
        Some(size) => {
            if size == 0 || size > ABSOLUTE_MAX_PAGE_SIZE {
                handle_error(&env, Error::InvalidMaxPageSize)
            }
            size
        }
        // TODO: Make page size configurable through contract configuration
        None => DEFAULT_MAX_PAGE_SIZE, // Default
    };

    let config = AdminConfig {
        initialized: true,
        super_admin,
        max_page_size: validated_max_page_size,
        total_user_count: 0,
    };

    // Store the configuration
    env.storage()
        .persistent()
        .set(&DataKey::AdminConfig, &config);

    // Initialize empty admin list (super_admin is checked separately)
    let empty_admins: Vec<Address> = Vec::new(&env);
    env.storage()
        .persistent()
        .set(&DataKey::Admins, &empty_admins);

    config
}

/// Add a new admin (super admin only)
pub fn add_admin(env: Env, caller: Address, new_admin: Address) {
    caller.require_auth();

    let config = env
        .storage()
        .persistent()
        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)
        .unwrap_or_else(|| handle_error(&env, Error::SystemNotInitialized));

    if !config.initialized {
        handle_error(&env, Error::SystemNotInitialized)
    }

    // Only super admin can add admins
    if caller != config.super_admin {
        handle_error(&env, Error::AccessDenied)
    }

    // Prevent adding super admin to regular admin list
    if new_admin == config.super_admin {
        handle_error(&env, Error::SuperAdminNotRegular)
    }

    let mut admins: Vec<Address> = env
        .storage()
        .persistent()
        .get::<DataKey, Vec<Address>>(&DataKey::Admins)
        .unwrap_or_else(|| Vec::new(&env));

    // Check if admin already exists
    if admins.iter().any(|a| a == new_admin) {
        handle_error(&env, Error::OperationFailed) // Don't disclose admin status
    }

    // Limit number of admins for security
    if (admins.len() as u32) >= MAX_ADMINS {
        handle_error(&env, Error::MaxAdminsReached)
    }

    admins.push_back(new_admin);
    env.storage().persistent().set(&DataKey::Admins, &admins);
}

/// Remove an admin (super admin only)
pub fn remove_admin(env: Env, caller: Address, admin_to_remove: Address) {
    caller.require_auth();

    let config = env
        .storage()
        .persistent()
        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)
        .unwrap_or_else(|| handle_error(&env, Error::SystemNotInitialized));

    if !config.initialized {
        handle_error(&env, Error::SystemNotInitialized)
    }

    // Only super admin can remove admins
    if caller != config.super_admin {
        handle_error(&env, Error::AccessDenied)
    }

    // Cannot remove super admin
    if admin_to_remove == config.super_admin {
        handle_error(&env, Error::CannotRemoveSuperAdmin)
    }

    let admins: Vec<Address> = env
        .storage()
        .persistent()
        .get::<DataKey, Vec<Address>>(&DataKey::Admins)
        .unwrap_or_else(|| Vec::new(&env));

    // Find and remove the admin
    let initial_len = admins.len();
    let mut new_admins = Vec::new(&env);

    for admin in admins.iter() {
        if admin != admin_to_remove {
            new_admins.push_back(admin);
        }
    }

    if new_admins.len() == initial_len {
        handle_error(&env, Error::AccessDenied) // Don't disclose admin existence
    }

    env.storage()
        .persistent()
        .set(&DataKey::Admins, &new_admins);
}

/// Get list of all admins (admin only)
pub fn get_admins(env: Env, caller: Address) -> Vec<Address> {
    caller.require_auth();

    let config = env
        .storage()
        .persistent()
        .get::<DataKey, AdminConfig>(&DataKey::AdminConfig)
        .unwrap_or_else(|| handle_error(&env, Error::SystemNotInitialized));

    if !config.initialized {
        handle_error(&env, Error::SystemNotInitialized);
    }

    // Check if caller is an admin (including super admin)
    let is_super_admin = caller == config.super_admin;
    let regular_admins: Vec<Address> = env
        .storage()
        .persistent()
        .get::<DataKey, Vec<Address>>(&DataKey::Admins)
        .unwrap_or_else(|| Vec::new(&env));
    let is_regular_admin = regular_admins.iter().any(|a| a == caller);

    if !is_super_admin && !is_regular_admin {
        handle_error(&env, Error::AccessDenied)
    }

    // Return all admins including super admin
    let mut all_admins = Vec::new(&env);
    all_admins.push_back(config.super_admin);

    for admin in regular_admins.iter() {
        all_admins.push_back(admin);
    }

    all_admins
}

/// Check if system is initialized
pub fn is_system_initialized(env: Env) -> bool {
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

#[cfg(test)]
mod tests {
    use crate::{UserManagement, UserManagementClient};
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_initialize_system() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();

        let initializer = Address::generate(&env);
        let super_admin = Address::generate(&env);

        const TEST_MAX_PAGE_SIZE: u32 = 50;
        let config =
            client.initialize_system(&initializer, &super_admin, &Some(TEST_MAX_PAGE_SIZE));

        assert!(config.initialized);
        assert_eq!(config.super_admin, super_admin);
        assert_eq!(config.max_page_size, TEST_MAX_PAGE_SIZE);
        assert_eq!(config.total_user_count, 0);

        assert!(client.is_system_initialized());
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #1)")]
    fn test_cannot_initialize_twice() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();

        let initializer = Address::generate(&env);
        let super_admin = Address::generate(&env);

        const TEST_MAX_PAGE_SIZE: u32 = 50;
        client.initialize_system(&initializer, &super_admin, &Some(TEST_MAX_PAGE_SIZE));
        client.initialize_system(&initializer, &super_admin, &Some(TEST_MAX_PAGE_SIZE));
    }

    #[test]
    fn test_add_remove_admin() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();

        let initializer = Address::generate(&env);
        let super_admin = Address::generate(&env);
        let new_admin = Address::generate(&env);

        client.initialize_system(&initializer, &super_admin, &None);

        // Add admin
        client.add_admin(&super_admin, &new_admin);

        let admins = client.get_admins(&super_admin);
        assert_eq!(admins.len(), 2); // super_admin + new_admin

        // Remove admin
        client.remove_admin(&super_admin, &new_admin);

        let admins = client.get_admins(&super_admin);
        assert_eq!(admins.len(), 1); // only super_admin
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #4)")]
    fn test_non_super_admin_cannot_add_admin() {
        let env = Env::default();
        let contract_id = env.register(UserManagement, {});
        let client = UserManagementClient::new(&env, &contract_id);
        env.mock_all_auths();

        let initializer = Address::generate(&env);
        let super_admin = Address::generate(&env);
        let regular_admin = Address::generate(&env);
        let new_admin = Address::generate(&env);

        client.initialize_system(&initializer, &super_admin, &None);
        client.add_admin(&super_admin, &regular_admin);

        // Regular admin tries to add another admin
        client.add_admin(&regular_admin, &new_admin);
    }
}

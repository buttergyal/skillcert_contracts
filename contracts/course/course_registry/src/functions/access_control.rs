// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{Course, DataKey};
use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

const KEY_USER_MGMT_ADDR: &str = "user_mgmt_addr";
const KEY_OWNER: &str = "owner";

/// Check if a user is an admin by querying the user management contract
pub fn is_admin(env: &Env, who: &Address) -> bool {
    // Get user management contract address
    let user_mgmt_addr: Option<Address> = env
        .storage()
        .instance()
        .get(&(KEY_USER_MGMT_ADDR,));

    match user_mgmt_addr {
        Some(addr) => {
            // Cross-contract call to check admin status
            env.invoke_contract(
                &addr,
                &Symbol::new(&env, "is_admin"),
                (who.clone(),).into_val(&env),
            )
        }
        None => false // If user management contract isn't configured, no admins
    }
}

/// Check if a user is the creator of a specific course
pub fn is_course_creator(env: &Env, course_id: &String, who: &Address) -> bool {
    let key = (symbol_short!("course"), course_id.clone());
    
    match env.storage().persistent().get::<_, Course>(&key) {
        Some(course) => course.creator == *who,
        None => false
    }
}

/// Require that the caller has proper authorization for course management
/// Authorization is granted if the caller is:
/// 1. The course creator
/// 2. An admin
pub fn require_course_management_auth(env: &Env, caller: &Address, course_id: &String) {
    // Always require basic authentication
    caller.require_auth();
    
    // Check if caller is course creator or admin
    if !is_course_creator(env, course_id, caller) && !is_admin(env, caller) {
        handle_error(env, Error::Unauthorized)
    }
}

/// Initialize access control settings
pub fn initialize(env: &Env, owner: &Address, user_mgmt_addr: &Address) {
    if env.storage().instance().has(&(KEY_OWNER,)) {
        handle_error(env, Error::AlreadyInitialized)
    }

    // Store contract owner and user management contract address
    env.storage().instance().set(&(KEY_OWNER,), owner);
    env.storage()
        .instance()
        .set(&(KEY_USER_MGMT_ADDR,), user_mgmt_addr);
}

/// Update the user management contract address
/// Only the contract owner can perform this update
pub fn update_user_mgmt_address(env: &Env, caller: &Address, new_addr: &Address) {
    caller.require_auth();
    
    // Check if caller is contract owner
    let owner: Address = env
        .storage()
        .instance()
        .get(&(KEY_OWNER,))
        .expect("Contract not initialized");

    if *caller != owner {
        handle_error(env, Error::Unauthorized)
    }

    env.storage()
        .instance()
        .set(&(KEY_USER_MGMT_ADDR,), new_addr);
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Env as _};

    #[test]
    fn test_course_creator_authorization() {
        let env = Env::default();
        let creator = Address::generate(&env);
        let other_user = Address::generate(&env);
        let course_id = String::from_str(&env, "course1");

        // Mock course storage
        let course = Course {
            id: course_id.clone(),
            creator: creator.clone(),
            title: String::from_str(&env, "Test Course"),
            description: String::from_str(&env, "Test Description"),
            price: 1000,
            is_archived: false,
            created_at: 12345,
            updated_at: Some(12346),
            category: None,
            language: None,
            thumbnail_url: None,
            level: None,
            duration_hours: None,
        };

        env.storage().persistent().set(
            &(symbol_short!("course"), course_id.clone()),
            &course
        );

        // Test creator authorization
        assert!(is_course_creator(&env, &course_id, &creator));
        assert!(!is_course_creator(&env, &course_id, &other_user));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #401)")]  // Unauthorized error
    fn test_unauthorized_access() {
        let env = Env::default();
        let unauthorized_user = Address::generate(&env);
        let course_id = String::from_str(&env, "course1");

        env.mock_all_auths();

        // Should panic with unauthorized error
        require_course_management_auth(&env, &unauthorized_user, &course_id);
    }

    #[test]
    fn test_admin_authorization() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user_mgmt = Address::generate(&env);

        // Initialize access control
        initialize(&env, &admin, &user_mgmt);

        // Mock user management contract
        env.register_contract(&user_mgmt, crate::test_user_management::TestUserManagement {});
        
        // Test admin access
        assert!(is_admin(&env, &admin));
    }
}
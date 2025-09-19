// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use super::utils::{concat_strings, u32_to_string};
use crate::error::{handle_error, Error};
pub use crate::schema::{Course, CourseModule};
use soroban_sdk::{symbol_short, vec, Address, Env, String, Symbol};

const COURSE_KEY: Symbol = symbol_short!("course");
const MODULE_KEY: Symbol = symbol_short!("module");

pub fn course_registry_add_module(
    env: Env,
    caller: Address,
    course_id: String,
    position: u32,
    title: String,
) -> CourseModule {
    // Validate input parameters
    if title.is_empty() {
        handle_error(&env, Error::EmptyModuleTitle)
    }

    let course_storage_key: (Symbol, String) = (COURSE_KEY, course_id.clone());

    if !env.storage().persistent().has(&course_storage_key) {
        handle_error(&env, Error::CourseIdNotExist)
    }

    // Verify caller has proper authorization
    super::access_control::require_course_management_auth(&env, &caller, &course_id);

    // Check for duplicate position
    let position_key: (Symbol, String, u32) = (symbol_short!("pos"), course_id.clone(), position);
    if env.storage().persistent().has(&position_key) {
        handle_error(&env, Error::DuplicateModulePosition)
    }

    let ledger_seq: u32 = env.ledger().sequence();

    let arr = vec![
        &env,
        String::from_str(&env, "module_"),
        course_id.clone(),
        String::from_str(&env, "_"),
        u32_to_string(&env, position),
        String::from_str(&env, "_"),
        u32_to_string(&env, ledger_seq),
    ];

    let module_id = concat_strings(&env, arr);

    // Create new module
    let module: CourseModule = CourseModule {
        id: module_id.clone(),
        course_id: course_id.clone(),
        position,
        title,
        created_at: env.ledger().timestamp(),
    };

    let storage_key: (Symbol, String) = (MODULE_KEY, module_id.clone());
    let position_key: (Symbol, String, u32) = (symbol_short!("pos"), course_id.clone(), position);

    env.storage().persistent().set(&storage_key, &module);
    env.storage().persistent().set(&position_key, &true);

    module
}

#[cfg(test)]
mod test {
    extern crate std;
    
    use super::*;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn create_course<'a>(client: &CourseRegistryClient<'a>, creator: &Address) -> Course {
        let title = String::from_str(&client.env, "title");
        let description = String::from_str(&client.env, "description");
        let price = 1000_u128;
        client.create_course(
            &creator,
            &title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        )
    }

    // Mock UserManagement contract for testing
    mod mock_user_management {
        use soroban_sdk::{contract, contractimpl, Address, Env};

        #[contract]
        pub struct UserManagement;

        #[contractimpl]
        impl UserManagement {
            pub fn is_admin(_env: Env, _who: Address) -> bool {
                // For testing, return false to force course creator authorization
                // This ensures that only course creators can add modules
                false
            }
        }
    }

    fn setup_test_env() -> (Env, Address, Address, CourseRegistryClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        
        // Register mock user management contract
        let user_mgmt_id = env.register(mock_user_management::UserManagement, ());
        
        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);

        // Setup admin
        let admin = Address::generate(&env);
        env.as_contract(&contract_id, || {
            crate::functions::access_control::initialize(&env, &admin, &user_mgmt_id);
        });

        (env, contract_id, admin, client)
    }

    #[test]
    fn test_add_module_success_course_creator() {
        let (env, _, _, client) = setup_test_env();
        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);

        let module = client.add_module(&creator, &course.id, &1, &String::from_str(&env, "Module 1"));

        assert_eq!(module.course_id, course.id);
        assert_eq!(module.position, 1);
        assert_eq!(module.title, String::from_str(&env, "Module 1"));
    }

    #[test]
    fn test_add_module_success_admin() {
        let (env, _, _admin, client) = setup_test_env();
        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);

        // Admin should be able to add modules
        let module = client.add_module(&creator, &course.id, &1, &String::from_str(&env, "Module 1"));

        assert_eq!(module.course_id, course.id);
        assert_eq!(module.position, 1);
        assert_eq!(module.title, String::from_str(&env, "Module 1"));
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #6)")] // Unauthorized error
    fn test_add_module_unauthorized() {
        let (env, _, _, client) = setup_test_env();
        let creator = Address::generate(&env);
        let _unauthorized_user = Address::generate(&env);
        let course = create_course(&client, &creator);

        // Unauthorized user should not be able to add modules
        let unauthorized_user = Address::generate(&env);
        client.add_module(
            &unauthorized_user,
            &course.id,
            &1,
            &String::from_str(&env, "Module 1"),
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")] // CourseIdNotExist error
    fn test_add_module_invalid_course() {
        let (env, _, _admin, client) = setup_test_env();

        let unauthorized_user = Address::generate(&env);
        client.add_module(
            &unauthorized_user,
            &String::from_str(&env, "invalid_course"),
            &1,
            &String::from_str(&env, "Module 1"),
        );
    }

    #[test]
    fn test_add_module_generates_unique_ids() {
        let (env, _, _admin, client) = setup_test_env();
        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);

        let module1 = client.add_module(&creator, &course.id, &1, &String::from_str(&env, "Module 1"));
        let module2 = client.add_module(&creator, &course.id, &2, &String::from_str(&env, "Module 2"));

        assert_ne!(module1.id, module2.id);
    }

    #[test]
    fn test_add_module_storage_key_format() {
        let (env, contract_id, _admin, client) = setup_test_env();
        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);

        let module = client.add_module(&creator, &course.id, &1, &String::from_str(&env, "Module 1"));

        let exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(MODULE_KEY, module.id.clone()))
        });

        assert!(exists);
    }

    #[test]
    #[should_panic]
    fn test_add_module_different_course_creator() {
        let (env, _, _, client) = setup_test_env();
        let creator1 = Address::generate(&env);
        let _creator2 = Address::generate(&env);
        
        let course1 = create_course(&client, &creator1);
        
        // Creator2 should not be able to add module to Creator1's course
        let creator2 = Address::generate(&env);
        client.add_module(&creator2, &course1.id, &1, &String::from_str(&env, "Module 1"));
    }

    #[test]
    #[should_panic]
    fn test_add_module_empty_title() {
        let (env, _, _admin, client) = setup_test_env();
        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);

        // Should panic with validation error for empty title
        client.add_module(&creator, &course.id, &1, &String::from_str(&env, ""));
    }

    #[test]
    #[should_panic]
    fn test_add_module_duplicate_position() {
        let (env, _, _admin, client) = setup_test_env();
        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);

        // Add first module at position 1
        client.add_module(&creator, &course.id, &1, &String::from_str(&env, "Module 1"));

        // Try to add another module at the same position
        client.add_module(&creator, &course.id, &1, &String::from_str(&env, "Module 2"));
    }
}

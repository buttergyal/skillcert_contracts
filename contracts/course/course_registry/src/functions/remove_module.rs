// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::CourseModule;
use crate::error::{Error, handle_error};
use soroban_sdk::{symbol_short, Env, String};

pub fn course_registry_remove_module(env: &Env, module_id: String) -> Result<(), &'static str> {
    if module_id.len() == 0 {
        handle_error(&env, Error::EmptyModuleId)
    }

    // Try to get the module data to verify it exists and is a valid CourseModule
    let module: Option<CourseModule> = env
        .storage()
        .persistent()
        .get(&(symbol_short!("module"), module_id.clone()));

    // Validate that the module exists and is a valid CourseModule
    if module.is_none() {
        handle_error(&env, Error::ModuleNotFound)
    }

    // Delete the CourseModule directly from persistent storage using its key.
    env.storage()
        .persistent()
        .remove(&(symbol_short!("module"), module_id.clone()));

    /// Emits an event to indicate the module has been removed.
    env.events().publish((module_id,), "module_removed");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{schema::Course, CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_remove_module_success() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "title"),
            &String::from_str(&env, "description"),
            &1000_u128,
            &Some(String::from_str(&env, "category")),
            &Some(String::from_str(&env, "language")),
            &Some(String::from_str(&env, "thumbnail_url")),
            &None,
            &None,
        );
        let new_module = client.add_module(&course.id, &0, &String::from_str(&env, "Module Title"));

        let exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(symbol_short!("module"), new_module.id.clone()))
        });
        assert!(exists);

        client.remove_module(&new_module.id);
        let exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(symbol_short!("module"), new_module.id.clone()))
        });
        assert!(!exists);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #22)")]
    fn test_remove_module_with_empty_id() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        client.remove_module(&String::from_str(&env, ""));
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #21)")]
    fn test_remove_module_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        client.remove_module(&String::from_str(&env, "non_existent_module"));
    }
}

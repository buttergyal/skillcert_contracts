// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::CourseModule;
use soroban_sdk::{Env, String, Symbol};

pub fn course_registry_list_modules(env: &Env, course_id: String) -> CourseModule {
    if course_id.len() == 0 {
        handle_error(&env, Error::EmptyCourseId)
    }

    let key: Symbol = Symbol::new(env, "module");

    // Get the course from storage
    let module: CourseModule = env
        .storage()
        .persistent()
        .get(&(key, course_id.clone()))
        .expect("Module with the specified ID does not exist");

    module
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CourseRegistry;
    use soroban_sdk::{symbol_short, testutils::Ledger, Address, Env, String};

    const MODULE_KEY: Symbol = symbol_short!("module");

    #[test]
    fn test_course_registry_add_module_storage_key_format() {
        let env: Env = Env::default();
        env.ledger().set_timestamp(100000);

        let contract_id: Address = env.register(CourseRegistry, {});

        // Create a test course first
        let course: CourseModule = CourseModule {
            id: String::from_str(&env, "test_module_123"),
            course_id: String::from_str(&env, "test_course_123"),
            position: 0,
            title: String::from_str(&env, "Introduction to Blockchain"),
            created_at: 0,
        };

        // Set up initial course data and perform test within contract context
        env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .set(&(MODULE_KEY, course.course_id.clone()), &course);
            course_registry_list_modules(&env, course.course_id)
        });
    }

    #[test]
    #[should_panic(expected = "Module with the specified ID does not exist")]
    fn test_add_module_invalid_course() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});

        let course_id: String = String::from_str(&env, "invalid_course");

        env.as_contract(&contract_id, || {
            course_registry_list_modules(&env, course_id);
        });
    }
}

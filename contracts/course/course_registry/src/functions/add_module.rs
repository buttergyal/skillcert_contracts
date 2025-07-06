use soroban_sdk::{Env, Symbol, symbol_short, String};
pub use crate::schema::{Course, CourseModule};

const COURSE_KEY: Symbol = symbol_short!("course");
const MODULE_KEY: Symbol = symbol_short!("module");

pub fn course_registry_add_module(env: Env, course_id: String, position: i32, title: String) -> CourseModule {
    // Verify course exists
    let course_storage_key = (COURSE_KEY, course_id.clone());

    // require!(env.storage().persistent().has(&course_storage_key), "Course with the specified ID does not exist");
        
    if !env.storage().persistent().has(&course_storage_key) {
        panic!("Course with the specified ID does not exist");
    }

    let ledger_seq = env.ledger().sequence();

    let module_id = String::from_str(&env, &format!("module_{}_{:?}_{:?}",
            course_id.to_string(),
            position,
            ledger_seq
        ));
    

    // Create new module
    let module = CourseModule {
        id: module_id.clone(),
        course_id,
        position,
        title,
        created_at: env.ledger().timestamp(),
    };

    let storage_key = (MODULE_KEY, module_id.clone());

    env.storage().persistent().set(&storage_key, &module);

    module
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{log, symbol_short, String};
    use soroban_sdk::{testutils::{Address as _, Ledger as _}, Address, Env};
    use crate::schema::{CourseModule, Course};
    use crate::CourseRegistry;

    #[test]
    fn test_add_module_success() {
        let env = Env::default();
        env.ledger().set_timestamp(100000);
        
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "course_123");
        let position = 1;
        let title = String::from_str(&env, "Introduction Module");

        // Create a test course first
        let course = Course {
            id: course_id.clone(),
            title: String::from_str(&env, "Test Course"),
            description: String::from_str(&env, "Test Description"),
            creator: Address::generate(&env),
            published: true,
        };
        
        // Set up initial course data and perform test within contract context
        env.as_contract(&contract_id, || {
            env.storage().persistent().set(&(COURSE_KEY, course_id.clone()), &course);
            log!(&env, "Stored course in contract storage");
        });

        // Act - Call the function within contract context
        let result = env.as_contract(&contract_id, || {
            log!(&env, "Calling add_module function");
            course_registry_add_module(
                env.clone(),
                course_id.clone(),
                position,
                title.clone(),
            )
        });

        // Assert - Verify the returned module
        assert_eq!(result.course_id, course_id);
        assert_eq!(result.position, position);
        assert_eq!(result.title, title);
        assert_eq!(result.created_at, 100000);
    }

    #[test]
    #[should_panic(expected = "Course with the specified ID does not exist")]
    fn test_add_module_invalid_course() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});

        let course_id = String::from_str(&env, "invalid_course");
        let position = 1;
        let title = String::from_str(&env, "Test Module");

        env.as_contract(&contract_id, || {
            course_registry_add_module(
                env.clone(),
                course_id,
                position,
                title,
            );
        });
    }

    #[test]
    fn test_course_registry_add_module_generates_unique_ids() {
        let env = Env::default();
        env.ledger().set_timestamp(100000);
        
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "course_123");
        let course_id_second = String::from_str(&env, "course_234");
        let position = 1;
        let title = String::from_str(&env, "Introduction Module");
        let title_second = String::from_str(&env, "Second Module");

        // Create a test course first
        let course = Course {
            id: course_id.clone(),
            title: String::from_str(&env, "Test Course"),
            description: String::from_str(&env, "Test Description"),
            creator: Address::generate(&env),
            published: true,
        };
        
        // Set up initial course data and perform test within contract context
        env.as_contract(&contract_id, || {
            env.storage().persistent().set(&(COURSE_KEY, course_id.clone()), &course);
            log!(&env, "Stored course in contract storage");
        });


        env.as_contract(&contract_id, || {
            env.storage().persistent().set(&(COURSE_KEY, course_id_second.clone()), &course);
            log!(&env, "Stored course in contract storage");
        });
        // Act - Call the function within contract context
        let result = env.as_contract(&contract_id, || {
            course_registry_add_module(
                env.clone(),
                course_id.clone(),
                position,
                title.clone(),
            )
        });

        let result_second = env.as_contract(&contract_id, || {
            course_registry_add_module(
                env.clone(),
                course_id_second.clone(),
                2,
                title_second.clone(),
            )
        });

        // Assert - Verify the returned module
        assert_eq!(result.id, String::from_str(&env, "module_course_123_1_0"));
        assert_eq!(result_second.id, String::from_str(&env, "module_course_234_2_0"));
    }

    #[test]
    fn test_course_registry_add_module_storage_key_format() {
        let env = Env::default();
        env.ledger().set_timestamp(100000);
        
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "course_123");
        let position = 1;
        let title = String::from_str(&env, "Introduction Module");

        // Create a test course first
        let course = Course {
            id: course_id.clone(),
            title: String::from_str(&env, "Test Course"),
            description: String::from_str(&env, "Test Description"),
            creator: Address::generate(&env),
            published: true,
        };
        
        // Set up initial course data and perform test within contract context
        env.as_contract(&contract_id, || {
            env.storage().persistent().set(&(COURSE_KEY, course_id.clone()), &course);
            log!(&env, "Stored course in contract storage");
        });

        // Act - Call the function within contract context
        let result = env.as_contract(&contract_id, || {
            course_registry_add_module(
                env.clone(),
                course_id.clone(),
                position,
                title.clone(),
            )
        });

        let expected_storage_key = (MODULE_KEY, result.id.clone());

        env.as_contract(&contract_id, || {
            assert!(env.storage().persistent().has(&expected_storage_key));
        });

        env.as_contract(&contract_id, || {
            let stored_module: CourseModule = env.storage().persistent().get(&expected_storage_key).unwrap();
            assert_eq!(stored_module.id, result.id);
        });
    }

}
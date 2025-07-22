use soroban_sdk::{Env, String, Symbol, symbol_short};
use crate::schema::{Course, CourseModule};

const COURSE_KEY: Symbol = symbol_short!("course");
const MODULE_KEY: Symbol = symbol_short!("module");
const TITLE_KEY: Symbol = symbol_short!("title");

pub fn course_registry_delete_course(env: &Env, course_id: String) -> Result<(), &'static str> {
    if course_id.is_empty() {
        return Err("Course ID cannot be empty");
    }

    let course_storage_key = (COURSE_KEY, course_id.clone());

    if !env.storage().persistent().has(&course_storage_key) {
        return Err("Course not found");
    }

    let course: Course = env
        .storage()
        .persistent()
        .get(&course_storage_key)
        .ok_or("Course not found")?;

    delete_course_modules(env, &course_id);

    let title_key = (TITLE_KEY, String::from_str(env, course.title.to_string().to_lowercase().as_str()));
    env.storage().persistent().remove(&title_key);

    env.storage().persistent().remove(&course_storage_key);

    env.events().publish((course_id,), "course_deleted");

    Ok(())
}

fn delete_course_modules(env: &Env, course_id: &String) {
    // For now, we'll implement a simple approach
    // In a real implementation, you might want to maintain an index of modules per course
    // This is a simplified version that checks a reasonable range of potential module IDs
    for counter in 0..100u32 {
        let potential_module_id = String::from_str(env, &format!("module_{}_{}_0", course_id.to_string(), counter));
        let module_key = (MODULE_KEY, potential_module_id.clone());

        if env.storage().persistent().has(&module_key) {
            let module_opt: Option<CourseModule> = env.storage().persistent().get(&module_key);
            if let Some(module) = module_opt {
                if module.course_id == *course_id {
                    env.storage().persistent().remove(&module_key);
                    env.events().publish((potential_module_id,), "module_deleted");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};
    use crate::schema::{Course, CourseModule};
    use crate::CourseRegistry;

    fn create_test_course(env: &Env, course_id: &str) -> Course {
        Course {
            id: String::from_str(env, course_id),
            title: String::from_str(env, "Test Course"),
            description: String::from_str(env, "Test Description"),
            creator: Address::generate(env),
            price: 1000,
            category: None,
            language: None,
            thumbnail_url: None,
            published: false,
        }
    }

    #[test]
    fn test_delete_course_success() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "test_course_123");
        let course = create_test_course(&env, "test_course_123");

        env.as_contract(&contract_id, || {
            let storage_key = (COURSE_KEY, course_id.clone());
            let title_key = (TITLE_KEY, String::from_str(&env, "test course"));
            env.storage().persistent().set(&storage_key, &course);
            env.storage().persistent().set(&title_key, &true);
        });

        let result = env.as_contract(&contract_id, || {
            course_registry_delete_course(&env, course_id.clone())
        });

        assert!(result.is_ok());

        env.as_contract(&contract_id, || {
            let storage_key = (COURSE_KEY, course_id.clone());
            let title_key = (TITLE_KEY, String::from_str(&env, "test course"));
            assert!(!env.storage().persistent().has(&storage_key));
            assert!(!env.storage().persistent().has(&title_key));
        });
    }

    #[test]
    fn test_delete_course_with_modules() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "123");
        let course = create_test_course(&env, "123");

        let module1 = CourseModule {
            id: String::from_str(&env, "module_123_0_0"),
            course_id: course_id.clone(),
            position: 0,
            title: String::from_str(&env, "Module 1"),
            created_at: 0,
        };

        env.as_contract(&contract_id, || {
            let course_key = (COURSE_KEY, course_id.clone());
            let title_key = (TITLE_KEY, String::from_str(&env, "test course"));
            let module1_key = (MODULE_KEY, module1.id.clone());

            env.storage().persistent().set(&course_key, &course);
            env.storage().persistent().set(&title_key, &true);
            env.storage().persistent().set(&module1_key, &module1);
        });

        let result = env.as_contract(&contract_id, || {
            course_registry_delete_course(&env, course_id.clone())
        });

        assert!(result.is_ok());

        env.as_contract(&contract_id, || {
            let course_key = (COURSE_KEY, course_id.clone());
            let module1_key = (MODULE_KEY, module1.id.clone());

            assert!(!env.storage().persistent().has(&course_key));
            assert!(!env.storage().persistent().has(&module1_key));
        });
    }

    #[test]
    fn test_delete_course_not_found() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "nonexistent");

        let result = env.as_contract(&contract_id, || {
            course_registry_delete_course(&env, course_id)
        });

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Course not found");
    }

    #[test]
    fn test_delete_course_empty_id() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "");

        let result = env.as_contract(&contract_id, || {
            course_registry_delete_course(&env, course_id)
        });

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Course ID cannot be empty");
    }

    #[test]
    fn test_delete_course_preserves_other_courses() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});

        let course1_id = String::from_str(&env, "1");
        let course2_id = String::from_str(&env, "2");
        let course1 = create_test_course(&env, "1");
        let course2 = create_test_course(&env, "2");

        env.as_contract(&contract_id, || {
            env.storage().persistent().set(&(COURSE_KEY, course1_id.clone()), &course1);
            env.storage().persistent().set(&(COURSE_KEY, course2_id.clone()), &course2);
            env.storage().persistent().set(&(TITLE_KEY, String::from_str(&env, "test course")), &true);
        });

        let result = env.as_contract(&contract_id, || {
            course_registry_delete_course(&env, course1_id.clone())
        });

        assert!(result.is_ok());

        env.as_contract(&contract_id, || {
            assert!(!env.storage().persistent().has(&(COURSE_KEY, course1_id)));
            assert!(env.storage().persistent().has(&(COURSE_KEY, course2_id)));
        });
    }
}
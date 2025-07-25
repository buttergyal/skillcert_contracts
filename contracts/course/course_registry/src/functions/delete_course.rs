use soroban_sdk::{Env, String, Symbol, symbol_short, Vec};
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
    let mut modules_to_delete: Vec<String> = Vec::new(env);

    let mut counter = 0u32;
    loop {
        let module_id = format!("module_{}_{}_0", course_id.to_string(), counter);
        let key = (MODULE_KEY, String::from_str(env, &module_id));
        if env.storage().persistent().has(&key) {
            if let Some(module) = env.storage().persistent().get::<_, CourseModule>(&key) {
                if module.course_id == *course_id {
                    modules_to_delete.push_back(String::from_str(env, &module_id));
                }
            }
        } else {
            break;
        }
        counter += 1;
        if counter > 1000 {
            break;
        }
    }

    for id in modules_to_delete.iter() {
        env.storage().persistent().remove(&(MODULE_KEY, id.clone()));
        env.events().publish((id.clone(),), "module_deleted");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env};
    use crate::schema::{Course, CourseModule};
    use crate::CourseRegistry;

    fn create_test_course(env: &Env, id: &str) -> Course {
        Course {
            id: String::from_str(env, id),
            title: String::from_str(env, "Test Course"),
            description: String::from_str(env, "Test Description"),
            creator: Address::generate(env),
            published: false,
        }
    }

    #[test]
    fn test_delete_course_success() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "course_1");
        let course = create_test_course(&env, "course_1");

        env.as_contract(&contract_id, || {
            env.storage().persistent().set(&(COURSE_KEY, course_id.clone()), &course);
            env.storage().persistent().set(&(TITLE_KEY, String::from_str(&env, "test course")), &true);
        });

        let result = env.as_contract(&contract_id, || {
            course_registry_delete_course(&env, course_id.clone())
        });

        assert!(result.is_ok());

        env.as_contract(&contract_id, || {
            assert!(!env.storage().persistent().has(&(COURSE_KEY, course_id.clone())));
            assert!(!env.storage().persistent().has(&(TITLE_KEY, String::from_str(&env, "test course"))));
        });
    }

    #[test]
    fn test_delete_course_with_modules() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "abc");
        let course = create_test_course(&env, "abc");

        let module = CourseModule {
            id: String::from_str(&env, "module_abc_0_0"),
            course_id: course_id.clone(),
            position: 0,
            title: String::from_str(&env, "Intro"),
            created_at: 0,
        };

        env.as_contract(&contract_id, || {
            env.storage().persistent().set(&(COURSE_KEY, course_id.clone()), &course);
            env.storage().persistent().set(&(TITLE_KEY, String::from_str(&env, "test course")), &true);
            env.storage().persistent().set(&(MODULE_KEY, module.id.clone()), &module);
        });

        let result = env.as_contract(&contract_id, || {
            course_registry_delete_course(&env, course_id.clone())
        });

        assert!(result.is_ok());

        env.as_contract(&contract_id, || {
            assert!(!env.storage().persistent().has(&(COURSE_KEY, course_id.clone())));
            assert!(!env.storage().persistent().has(&(MODULE_KEY, module.id.clone())));
        });
    }

    #[test]
    fn test_delete_course_not_found() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});
        let course_id = String::from_str(&env, "not_found");

        let result = env.as_contract(&contract_id, || {
            course_registry_delete_course(&env, course_id)
        });

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

        assert_eq!(result.unwrap_err(), "Course ID cannot be empty");
    }

    #[test]
    fn test_delete_course_preserves_others() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, {});

        let id1 = String::from_str(&env, "keep");
        let id2 = String::from_str(&env, "remove");

        let c1 = create_test_course(&env, "keep");
        let c2 = create_test_course(&env, "remove");

        env.as_contract(&contract_id, || {
            env.storage().persistent().set(&(COURSE_KEY, id1.clone()), &c1);
            env.storage().persistent().set(&(COURSE_KEY, id2.clone()), &c2);
            env.storage().persistent().set(&(TITLE_KEY, String::from_str(&env, "test course")), &true);
        });

        env.as_contract(&contract_id, || {
            course_registry_delete_course(&env, id2.clone()).unwrap();
        });

        env.as_contract(&contract_id, || {
            assert!(!env.storage().persistent().has(&(COURSE_KEY, id2.clone())));
            assert!(env.storage().persistent().has(&(COURSE_KEY, id1.clone())));
        });
    }
}

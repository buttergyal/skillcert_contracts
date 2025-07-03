


use soroban_sdk::{symbol_short, Address, Env, String, Symbol};
use crate::schema::{Course, };

const COURSE_KEY: Symbol = symbol_short!("course");
const TITLE_KEY: Symbol = symbol_short!("title");
const COURSE_ID: Symbol = symbol_short!("course");

pub fn course_registry_create_course(env: Env, title: String, description: String) {

    let caller: Address = env.current_contract_address();

    let storage_key: (Symbol, String) = (COURSE_KEY, title.clone());

    if env.storage().persistent().has(&storage_key) {
        panic!("Course with this ID already exists");
    }

    // create a new course
    let new_course: Course = Course {
        id: title.clone(),
        title,
        description,
        creator: caller,
        published: false,
    };

    // save to the storage
    env.storage().persistent().set(&storage_key, &new_course);
}




fn generate_course_id(env: &Env) -> u128 {
    let current_id: u128 = env.storage().persistent()
        .get(&COURSE_ID)
        .unwrap_or(0);
    let new_id = current_id + 1;
    env.storage().persistent().set(&COURSE_ID, &new_id);
    new_id
}


#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{log, Address, String, Env};
    use crate::schema::{ Course};
    use crate::CourseRegistry;
    
    #[test]
    fn test_generate_course_id() {
        let env = Env::default();

        let contract_id: Address = env.register(CourseRegistry, {});
        env.as_contract(&contract_id, || {
            generate_course_id(&env);
            let id: u128 = generate_course_id(&env);
            assert_eq!(id, 2);
        });
    }

    #[test]
    fn test_add_module_success() {
        let env = Env::default();

        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");


        // Act - Call the function within contract context
        env.as_contract(&contract_id, || {
            log!(&env, "Calling add_module function");
            course_registry_create_course(
                env.clone(),
                title.clone(),
                description.clone(),
            )
        });

        // try get the course out
        let storage_key: (Symbol, String) = (COURSE_KEY, title.clone());
        let stored_course: Option<Course> = env
            .as_contract(&contract_id, || {
                env.storage().persistent().get(&storage_key)
            });

        let course = stored_course.expect("Course should be stored");

        assert_eq!(course.title, title);
        assert_eq!(course.description, description);
        assert!(!course.published);
    }
    
    
    
}
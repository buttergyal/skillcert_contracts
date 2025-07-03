use alloc::string::ToString;
use soroban_sdk::{symbol_short, Address, Env, String, Symbol};
use crate::schema::{Course, };

const COURSE_KEY: Symbol = symbol_short!("course");
const TITLE_KEY: Symbol = symbol_short!("title");
const COURSE_ID: Symbol = symbol_short!("course");

pub fn course_registry_create_course(env: Env, title: String, description: String) {

    let caller: Address = env.current_contract_address();
    
    // ensure the title is not empty 
    if title.is_empty() { 
        panic!("Course error: Course Title cannot be empty");
    }
    
    // to avoid duplicate title, 
    let title_key: (Symbol, String) = (TITLE_KEY, String::from_str(&env, title.to_string().to_lowercase().as_str()));
    
    if env.storage().persistent().has(&title_key) { 
        panic!("Course error: Course Title already exists");
    }
    
    // generate the unique id
    let id: u128 = generate_course_id(&env);
    let converted_id: String = String::from_str(&env, id.to_string().as_str());
    

    let storage_key: (Symbol, String) = (COURSE_KEY, converted_id.clone());

    if env.storage().persistent().has(&storage_key) {
        panic!("Course with this ID already exists");
    }

    // create a new course
    let new_course: Course = Course {
        id: converted_id.clone()    ,
        title,
        description,
        creator: caller,
        published: false,
    };

    // save to the storage
    env.storage().persistent().set(&storage_key, &new_course);
    env.storage().persistent().set(&title_key, &true);
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
    use soroban_sdk::{ Address, String, Env};
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

        env.as_contract(&contract_id, || {
            course_registry_create_course(env.clone(), title.clone(), description.clone());
            // Verify course storage
            let storage_key: (Symbol, String) = (COURSE_KEY, String::from_str(&env, "1"));
            let stored_course: Option<Course> = env.storage().persistent().get(&storage_key);
            let course = stored_course.expect("Course should be stored");
            assert_eq!(course.title, title);
            assert_eq!(course.description, description);
            assert_eq!(course.id, String::from_str(&env, "1"));
            assert!(!course.published);
        });
        
    }   
    
    #[test]
    fn test_add_module_success_multiple() {
        let env: Env = Env::default();
        
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");

        let another_course_title: String = String::from_str(&env, "another title");
        let another_course_description: String = String::from_str(&env, "another description");
        
        env.as_contract(&contract_id, || {
            course_registry_create_course(env.clone(), title.clone(), description.clone());
            
            //create a second course
            course_registry_create_course(env.clone(), another_course_title.clone(), another_course_description.clone());
            
            let storage_key: (Symbol, String) = (COURSE_KEY, String::from_str(&env, "2"));
            
            let stored_course: Option<Course> = env.storage().persistent().get(&storage_key);
            
            let course: Course = stored_course.expect("Course should be stored");
            
            assert_eq!(course.title, another_course_title);
            assert_eq!(course.description, another_course_description);
            assert_eq!(course.id, String::from_str(&env, "2"));
            
        });
    }
    
    #[test]
    #[should_panic(expected = "Course error: Course Title already exists")]
    fn test_cannot_create_courses_with_duplicate_title() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");
        let another_description: String = String::from_str(&env, "another description");
        
        env.as_contract(&contract_id, || {
            course_registry_create_course(env.clone(), title.clone(), description.clone());
            
            // create another course with the same title
            course_registry_create_course(env.clone(), title.clone(), another_description.clone());
        })
    }
}
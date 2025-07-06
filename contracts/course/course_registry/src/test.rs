#[cfg(test)]
mod tests {
    use soroban_sdk::{Address, Env, String, Symbol, testutils::Address as _};
    use crate::schema::Course;
    use crate::functions::get_course::course_registry_get_course;

    #[test]
    fn test_get_course_success() {
        let env = Env::default();
        
        // Create test data
        let course_id = String::from_str(&env, "course_123");
        let title = String::from_str(&env, "Test Course");
        let description = String::from_str(&env, "A test course description");
        let creator = Address::generate(&env);
        let published = true;
        
        let course = Course {
            id: course_id.clone(),
            title: title.clone(),
            description: description.clone(),
            creator: creator.clone(),
            published,
        };
        
        // Set up contract environment
        let contract_id = env.register_contract(None, crate::CourseRegistry);
        
        // Store the course in the contract's storage
        let key = Symbol::new(&env, "course");
        env.as_contract(&contract_id, || {
            env.storage().instance().set(&(key, course_id.clone()), &course);
        });
        
        // Test the get_course function
        let retrieved_course = env.as_contract(&contract_id, || {
            course_registry_get_course(&env, course_id)
        });
        
        // Verify the retrieved course matches the stored course
        assert_eq!(retrieved_course.id, course.id);
        assert_eq!(retrieved_course.title, course.title);
        assert_eq!(retrieved_course.description, course.description);
        assert_eq!(retrieved_course.creator, course.creator);
        assert_eq!(retrieved_course.published, course.published);
    }
    
    #[test]
    #[should_panic(expected = "Course not found")]
    fn test_get_course_not_found() {
        let env = Env::default();
        let non_existent_course_id = String::from_str(&env, "non_existent_course");
        
        // Set up contract environment
        let contract_id = env.register_contract(None, crate::CourseRegistry);
        
        // This should panic with "Course not found"
        env.as_contract(&contract_id, || {
            course_registry_get_course(&env, non_existent_course_id)
        });
    }
    
    #[test]
    fn test_get_course_with_different_course_id() {
        let env = Env::default();
        
        // Create test data for a different course
        let course_id = String::from_str(&env, "advanced_course_456");
        let title = String::from_str(&env, "Advanced Course");
        let description = String::from_str(&env, "An advanced course description");
        let creator = Address::generate(&env);
        let published = false;
        
        let course = Course {
            id: course_id.clone(),
            title: title.clone(),
            description: description.clone(),
            creator: creator.clone(),
            published,
        };
        
        // Set up contract environment
        let contract_id = env.register_contract(None, crate::CourseRegistry);
        
        // Store the course in the contract's storage
        let key = Symbol::new(&env, "course");
        env.as_contract(&contract_id, || {
            env.storage().instance().set(&(key, course_id.clone()), &course);
        });
        
        // Test the get_course function
        let retrieved_course = env.as_contract(&contract_id, || {
            course_registry_get_course(&env, course_id)
        });
        
        // Verify the retrieved course matches the stored course
        assert_eq!(retrieved_course.id, course.id);
        assert_eq!(retrieved_course.title, course.title);
        assert_eq!(retrieved_course.description, course.description);
        assert_eq!(retrieved_course.creator, course.creator);
        assert_eq!(retrieved_course.published, course.published);
    }
}

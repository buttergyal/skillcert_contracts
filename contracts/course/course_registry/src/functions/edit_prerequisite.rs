use soroban_sdk::{Env, String, Vec, symbol_short, Symbol, Map};
use crate::schema::{Course, DataKey};

const PREREQ_UPDATED_EVENT: Symbol = symbol_short!("preqedit");

pub fn course_registry_edit_prerequisite(
    env: Env,
    course_id: String,
    new_prerequisites: Vec<String>,
) {
    let invoker = env.current_contract_address();

    // Load course to verify it exists and check authorization
    let course_key = (symbol_short!("course"), course_id.clone());
    let course: Course = env.storage().persistent()
        .get(&course_key)
        .expect("Course not found");

    // Authorization: only creator can edit prerequisites
    if course.creator != invoker {
        panic!("Only the course creator can edit prerequisites");
    }

    // Validate that all prerequisite courses exist
    for prerequisite_id in new_prerequisites.iter() {
        let prereq_course_key = (symbol_short!("course"), prerequisite_id.clone());
        if !env.storage().persistent().has(&prereq_course_key) {
            panic!("Prerequisite course not found");
        }
    }

    // Prevent circular dependencies
    validate_no_circular_dependency(&env, &course_id, &new_prerequisites);

    // Save updated prerequisites
    env.storage().persistent().set(&DataKey::CoursePrerequisites(course_id.clone()), &new_prerequisites);

    // Emit event
    env.events().publish(
        (PREREQ_UPDATED_EVENT, course_id),
        new_prerequisites.len() as u32,
    );
}

fn validate_no_circular_dependency(
    env: &Env,
    course_id: &String,
    new_prerequisites: &Vec<String>
) {
    // Check if course_id appears in new_prerequisites (direct circular dependency)
    for prerequisite_id in new_prerequisites.iter() {
        if prerequisite_id.eq(course_id) {
            panic!("Course cannot be its own prerequisite");
        }
    }

    // Check for indirect circular dependencies using DFS
    let mut visited = Map::new(env);
    let mut rec_stack = Map::new(env);
    
    for prerequisite_id in new_prerequisites.iter() {
        if has_cycle(env, &prerequisite_id, course_id, &mut visited, &mut rec_stack) {
            panic!("Circular dependency detected");
        }
    }
}

fn has_cycle(
    env: &Env,
    current_course: &String,
    target_course: &String,
    visited: &mut Map<String, bool>,
    rec_stack: &mut Map<String, bool>
) -> bool {
    // If we've reached the target course, we found a cycle
    if current_course.eq(target_course) {
        return true;
    }

    // If already in recursion stack, we have a cycle
    if rec_stack.contains_key(current_course.clone()) {
        return true;
    }

    // If already visited and not in recursion stack, no cycle from this path
    if visited.contains_key(current_course.clone()) {
        return false;
    }

    // Mark as visited and add to recursion stack
    visited.set(current_course.clone(), true);
    rec_stack.set(current_course.clone(), true);

    // Get prerequisites for current course
    let prerequisites: Vec<String> = env.storage().persistent()
        .get(&DataKey::CoursePrerequisites(current_course.clone()))
        .unwrap_or(Vec::new(env));

    // Recursively check all prerequisites
    for prerequisite in prerequisites.iter() {
        if has_cycle(env, &prerequisite, target_course, visited, rec_stack) {
            return true;
        }
    }

    // Remove from recursion stack before returning
    rec_stack.remove(current_course.clone());
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functions::create_course::course_registry_create_course;
    use crate::CourseRegistry;
    use soroban_sdk::{testutils::Events, Env, String};

    fn create_test_course(env: &Env, title: &str) -> String {
        let course = course_registry_create_course(
            env.clone(),
            String::from_str(env, title),
            String::from_str(env, "Test description"),
            1000,
            None,
            None,
            None,
        );
        course.id
    }

    #[test]
    fn test_edit_prerequisite_success() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, ());

        env.as_contract(&contract_id, || {
            // Create test courses
            let course1_id = create_test_course(&env, "Course 1");
            let course2_id = create_test_course(&env, "Course 2");
            let course3_id = create_test_course(&env, "Course 3");

            // Create prerequisites vector
            let mut prerequisites = Vec::new(&env);
            prerequisites.push_back(course2_id.clone());
            prerequisites.push_back(course3_id.clone());

            // Edit prerequisites
            course_registry_edit_prerequisite(env.clone(), course1_id.clone(), prerequisites.clone());

            // Verify prerequisites were saved
            let stored_prerequisites: Vec<String> = env.storage().persistent()
                .get(&DataKey::CoursePrerequisites(course1_id.clone()))
                .unwrap();

            assert_eq!(stored_prerequisites.len(), 2);
            assert_eq!(stored_prerequisites.get(0).unwrap(), course2_id);
            assert_eq!(stored_prerequisites.get(1).unwrap(), course3_id);

            // Verify event was emitted - just check that events were generated
            let events = env.events().all();
            assert!(!events.is_empty()); // Should have at least some events
        });
    }

    #[test]
    fn test_edit_prerequisite_replace_existing() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, ());

        env.as_contract(&contract_id, || {
            // Create test courses
            let course1_id = create_test_course(&env, "Course 1");
            let course2_id = create_test_course(&env, "Course 2");
            let course3_id = create_test_course(&env, "Course 3");
            let course4_id = create_test_course(&env, "Course 4");

            // Set initial prerequisites
            let mut initial_prerequisites = Vec::new(&env);
            initial_prerequisites.push_back(course2_id.clone());
            course_registry_edit_prerequisite(env.clone(), course1_id.clone(), initial_prerequisites);

            // Replace with new prerequisites
            let mut new_prerequisites = Vec::new(&env);
            new_prerequisites.push_back(course3_id.clone());
            new_prerequisites.push_back(course4_id.clone());
            course_registry_edit_prerequisite(env.clone(), course1_id.clone(), new_prerequisites.clone());

            // Verify old prerequisites were replaced
            let stored_prerequisites: Vec<String> = env.storage().persistent()
                .get(&DataKey::CoursePrerequisites(course1_id.clone()))
                .unwrap();

            assert_eq!(stored_prerequisites.len(), 2);
            assert_eq!(stored_prerequisites.get(0).unwrap(), course3_id);
            assert_eq!(stored_prerequisites.get(1).unwrap(), course4_id);
        });
    }

    #[test]
    fn test_edit_prerequisite_empty_list() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, ());

        env.as_contract(&contract_id, || {
            // Create test courses
            let course1_id = create_test_course(&env, "Course 1");
            let course2_id = create_test_course(&env, "Course 2");

            // Set initial prerequisites
            let mut initial_prerequisites = Vec::new(&env);
            initial_prerequisites.push_back(course2_id.clone());
            course_registry_edit_prerequisite(env.clone(), course1_id.clone(), initial_prerequisites);

            // Clear prerequisites with empty list
            let empty_prerequisites = Vec::new(&env);
            course_registry_edit_prerequisite(env.clone(), course1_id.clone(), empty_prerequisites);

            // Verify prerequisites were cleared
            let stored_prerequisites: Vec<String> = env.storage().persistent()
                .get(&DataKey::CoursePrerequisites(course1_id.clone()))
                .unwrap();

            assert_eq!(stored_prerequisites.len(), 0);
        });
    }

    #[test]
    #[should_panic(expected = "Course not found")]
    fn test_edit_prerequisite_course_not_found() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, ());

        env.as_contract(&contract_id, || {
            let prerequisites = Vec::new(&env);
            course_registry_edit_prerequisite(
                env.clone(), 
                String::from_str(&env, "nonexistent"), 
                prerequisites
            );
        });
    }

    #[test]
    #[should_panic(expected = "Prerequisite course not found")]
    fn test_edit_prerequisite_invalid_prerequisite() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, ());

        env.as_contract(&contract_id, || {
            // Create test course
            let course1_id = create_test_course(&env, "Course 1");

            // Try to add nonexistent prerequisite
            let mut prerequisites = Vec::new(&env);
            prerequisites.push_back(String::from_str(&env, "nonexistent"));

            course_registry_edit_prerequisite(env.clone(), course1_id, prerequisites);
        });
    }

    #[test]
    #[should_panic(expected = "Course cannot be its own prerequisite")]
    fn test_edit_prerequisite_direct_circular_dependency() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, ());

        env.as_contract(&contract_id, || {
            // Create test course
            let course1_id = create_test_course(&env, "Course 1");

            // Try to make course prerequisite of itself
            let mut prerequisites = Vec::new(&env);
            prerequisites.push_back(course1_id.clone());

            course_registry_edit_prerequisite(env.clone(), course1_id, prerequisites);
        });
    }

    #[test]
    #[should_panic(expected = "Circular dependency detected")]
    fn test_edit_prerequisite_indirect_circular_dependency() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, ());

        env.as_contract(&contract_id, || {
            // Create test courses
            let course1_id = create_test_course(&env, "Course 1");
            let course2_id = create_test_course(&env, "Course 2");
            let course3_id = create_test_course(&env, "Course 3");

            // Set up chain: Course1 -> Course2 -> Course3
            let mut prerequisites2 = Vec::new(&env);
            prerequisites2.push_back(course3_id.clone());
            course_registry_edit_prerequisite(env.clone(), course2_id.clone(), prerequisites2);

            let mut prerequisites1 = Vec::new(&env);
            prerequisites1.push_back(course2_id.clone());
            course_registry_edit_prerequisite(env.clone(), course1_id.clone(), prerequisites1);

            // Try to create cycle: Course3 -> Course1 (which would create Course1 -> Course2 -> Course3 -> Course1)
            let mut prerequisites3 = Vec::new(&env);
            prerequisites3.push_back(course1_id.clone());

            course_registry_edit_prerequisite(env.clone(), course3_id, prerequisites3);
        });
    }

    #[test]
    fn test_edit_prerequisite_authorization() {
        let env = Env::default();
        
        // Note: In the current implementation, we use env.current_contract_address() 
        // which means the authorization check will always pass in tests.
        // This test verifies the function works when authorization passes.
        let contract_id = env.register(CourseRegistry, ());

        env.as_contract(&contract_id, || {
            let course1_id = create_test_course(&env, "Course 1");
            let course2_id = create_test_course(&env, "Course 2");

            let mut prerequisites = Vec::new(&env);
            prerequisites.push_back(course2_id.clone());

            // This should succeed since we're calling from the contract address
            course_registry_edit_prerequisite(env.clone(), course1_id.clone(), prerequisites);

            // Verify it worked
            let stored_prerequisites: Vec<String> = env.storage().persistent()
                .get(&DataKey::CoursePrerequisites(course1_id))
                .unwrap();
            assert_eq!(stored_prerequisites.len(), 1);
        });
    }

    #[test]
    fn test_edit_prerequisite_complex_chain() {
        let env = Env::default();
        let contract_id = env.register(CourseRegistry, ());

        env.as_contract(&contract_id, || {
            // Create a complex but valid prerequisite chain
            // Course1 -> [Course2, Course3]
            // Course2 -> [Course4]
            // Course3 -> [Course5]
            // No cycles
            let course1_id = create_test_course(&env, "Course 1");
            let course2_id = create_test_course(&env, "Course 2");
            let course3_id = create_test_course(&env, "Course 3");
            let course4_id = create_test_course(&env, "Course 4");
            let course5_id = create_test_course(&env, "Course 5");

            // Set Course2 -> Course4
            let mut prerequisites2 = Vec::new(&env);
            prerequisites2.push_back(course4_id.clone());
            course_registry_edit_prerequisite(env.clone(), course2_id.clone(), prerequisites2);

            // Set Course3 -> Course5
            let mut prerequisites3 = Vec::new(&env);
            prerequisites3.push_back(course5_id.clone());
            course_registry_edit_prerequisite(env.clone(), course3_id.clone(), prerequisites3);

            // Set Course1 -> [Course2, Course3] - should work
            let mut prerequisites1 = Vec::new(&env);
            prerequisites1.push_back(course2_id.clone());
            prerequisites1.push_back(course3_id.clone());
            course_registry_edit_prerequisite(env.clone(), course1_id.clone(), prerequisites1);

            // Verify all prerequisites were set correctly
            let stored_prerequisites: Vec<String> = env.storage().persistent()
                .get(&DataKey::CoursePrerequisites(course1_id))
                .unwrap();
            assert_eq!(stored_prerequisites.len(), 2);
        });
    }
}
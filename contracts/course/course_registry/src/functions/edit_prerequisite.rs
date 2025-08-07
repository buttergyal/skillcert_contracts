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
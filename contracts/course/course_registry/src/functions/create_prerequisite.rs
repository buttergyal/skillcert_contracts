use crate::schema::{Course, DataKey};
use soroban_sdk::{symbol_short, Address, Env, Map, String, Symbol, Vec};

const PREREQ_CREATED_EVENT: Symbol = symbol_short!("prereqAdd");

pub fn course_registry_add_prerequisite(
    env: Env,
    creator: Address,
    course_id: String,
    prerequisites: Vec<String>,
) {
    creator.require_auth();

    let course_key: (Symbol, String) = (symbol_short!("course"), course_id.clone());
    let course: Course = env
        .storage()
        .persistent()
        .get(&course_key)
        .expect("Course not found");

    if course.creator != creator {
        panic!("Only the course creator can edit prerequisites");
    }

    for prerequisite_id in prerequisites.iter() {
        let prereq_course_key: (Symbol, String) = (symbol_short!("course"), prerequisite_id.clone());
        if !env.storage().persistent().has(&prereq_course_key) {
            panic!("Prerequisite course not found");
        }
    }

    validate_no_circular_dependency(&env, &course_id, &prerequisites);

    env.storage().persistent().set(
        &DataKey::CoursePrerequisites(course_id.clone()),
        &prerequisites,
    );

    env.events().publish(
        (PREREQ_CREATED_EVENT, course_id),
        prerequisites.len() as u32,
    );

}

fn validate_no_circular_dependency(env: &Env, course_id: &String, new_prerequisites: &Vec<String>) {
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
        if has_cycle(
            env,
            &prerequisite_id,
            course_id,
            &mut visited,
            &mut rec_stack,
        ) {
            panic!("Circular dependency detected");
        }
    }
}

fn has_cycle(
    env: &Env,
    current_course: &String,
    target_course: &String,
    visited: &mut Map<String, bool>,
    rec_stack: &mut Map<String, bool>,
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
    let prerequisites: Vec<String> = env
        .storage()
        .persistent()
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

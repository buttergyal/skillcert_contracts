use soroban_sdk::{Env, Address, String, Vec, symbol_short, Symbol};
use crate::schema::{Course, DataKey};

const PREREQ_REMOVED_EVENT: Symbol = symbol_short!("prereq_removed");

pub fn course_registry_remove_prerequisite(
    env: Env,
    course_id: String,
    prerequisite_course_id: String,
) {
    let invoker = env.invoker();

    // Load course
    let course_key = (symbol_short!("course"), course_id.clone());
    let course: Course = env.storage().persistent()
        .get(&course_key)
        .expect("Course not found");

    // Authorization: only creator can remove prerequisites
    if course.creator != invoker {
        panic!("Only the course creator can remove prerequisites");
    }

    // Load current list of prerequisites
    let mut prerequisites: Vec<String> = env.storage().persistent()
        .get(&DataKey::CoursePrerequisites(course_id.clone()))
        .unwrap_or(Vec::new(&env));

    // Find and remove the prerequisite
    let index = prerequisites.iter().position(|id| id == &prerequisite_course_id);

    match index {
        Some(i) => {
            prerequisites.remove(i);
        },
        None => {
            panic!("Prerequisite not found in the list");
        }
    }

    // Save updated prerequisites
    env.storage().persistent().set(&DataKey::CoursePrerequisites(course_id.clone()), &prerequisites);

    // Emit event
    env.events().publish(
        (PREREQ_REMOVED_EVENT, course_id),
        prerequisite_course_id,
    );
}

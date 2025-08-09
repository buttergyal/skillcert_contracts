use crate::schema::{Course, CourseGoal, DataKey};
use soroban_sdk::{symbol_short, Env, String, Symbol};

const GOAL_EDITED_EVENT: Symbol = symbol_short!("goaledit");

pub fn course_registry_edit_goal(
    env: Env,
    course_id: String,
    goal_id: String,
    new_content: String,
) -> CourseGoal {
    // Validate input
    if new_content.is_empty() {
        panic!("New goal content cannot be empty");
    }

    let invoker = env.current_contract_address();

    // Load course
    let storage_key = (symbol_short!("course"), course_id.clone());
    let course: Course = env
        .storage()
        .persistent()
        .get(&storage_key)
        .expect("Course not found");

    // Only creator can edit goal (or later: check admin)
    if course.creator != invoker {
        panic!("Only the course creator can edit goals");
    }

    let goal_key = DataKey::CourseGoal(course_id.clone(), goal_id.clone());
    let mut goal: CourseGoal = env
        .storage()
        .persistent()
        .get(&goal_key)
        .expect("Goal not found");

    // Update goal content
    goal.content = new_content.clone();

    // Save updated goal
    env.storage().persistent().set(&goal_key, &goal);

    // Emit event
    env.events().publish(
        (GOAL_EDITED_EVENT, course_id.clone(), goal_id.clone()),
        new_content.clone(),
    );

    goal
}

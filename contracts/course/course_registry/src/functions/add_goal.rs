use crate::functions::utils;
use crate::schema::{Course, CourseGoal, DataKey};
use soroban_sdk::{symbol_short, Env, String, Symbol, Vec};
// use functions::utils::generate_unique_id;

const GOAL_ADDED_EVENT: Symbol = symbol_short!("goaladd");

pub fn course_registry_add_goal(env: Env, course_id: String, content: String) -> CourseGoal {
    // Validate input
    if content.is_empty() {
        panic!("Goal content cannot be empty");
    }

    let invoker = env.current_contract_address();

    // Load course
    let storage_key = (symbol_short!("course"), course_id.clone());
    let course: Course = env
        .storage()
        .persistent()
        .get(&storage_key)
        .expect("Course not found");

    // Only creator can add goal (or later: check admin)
    if course.creator != invoker {
        panic!("Only the course creator can add goals");
    }

    // Generate a unique goal ID
    let goal_id = utils::generate_unique_id(&env);

    // Load or initialize goal list
    let mut goals: Vec<CourseGoal> = env
        .storage()
        .persistent()
        .get(&DataKey::CourseGoal(course_id.clone(), goal_id.clone()))
        .unwrap_or(Vec::new(&env));

    // Create new goal
    let goal = CourseGoal {
        course_id: course_id.clone(),
        goal_id: goal_id.clone(),
        content: content.clone(),
        created_by: invoker,
        created_at: env.ledger().timestamp(),
    };

    goals.push_back(goal.clone());

    // Save updated goal list
    env.storage().persistent().set(
        &DataKey::CourseGoal(course_id.clone(), goal_id.clone()),
        &goals,
    );

    // Emit event
    env.events().publish(
        (GOAL_ADDED_EVENT, course_id.clone(), goal_id.clone()),
        content.clone(),
    );

    goal
}

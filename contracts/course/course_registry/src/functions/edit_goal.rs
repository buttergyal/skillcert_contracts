// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use super::is_course_creator::is_course_creator;
use crate::error::{handle_error, Error};
use crate::schema::{Course, CourseGoal, DataKey};
use soroban_sdk::{symbol_short, Address, Env, String, Symbol};
// use soroban_sdk::testutils::Logs;

const GOAL_EDITED_EVENT: Symbol = symbol_short!("goaledit");

pub fn edit_goal(
    env: Env,
    creator: Address,
    course_id: String,
    goal_id: String,
    new_content: String,
) -> CourseGoal {
    creator.require_auth();
    // Validate input
    if course_id.is_empty() {
        handle_error(&env, Error::InvalidInput)
    }
    if goal_id.is_empty() {
        handle_error(&env, Error::InvalidInput)
    }
    if new_content.is_empty() {
        handle_error(&env, Error::EmptyNewGoalContent)
    }

    // Load course
    let storage_key = (symbol_short!("course"), course_id.clone());
    let course: Course = env
        .storage()
        .persistent()
        .get(&storage_key)
        .expect("Course not found");

    // Only creator can edit goal (or later: check admin)
    if !is_course_creator(&env, course.id.clone(), creator) {
        handle_error(&env, Error::Unauthorized)
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

#[cfg(test)]
mod test {
    use crate::schema::Course;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn setup_course_and_goal(
        env: &Env,
        client: &CourseRegistryClient,
        creator: &Address,
    ) -> (Course, String) {
        let course: Course = client.create_course(
            creator,
            &String::from_str(env, "Test Course"),
            &String::from_str(env, "Test Description"),
            &1000_u128,
            &Some(String::from_str(env, "category")),
            &Some(String::from_str(env, "language")),
            &Some(String::from_str(env, "thumbnail_url")),
            &None,
            &None,
        );

        let goal_content = String::from_str(env, "Learn the basics of Rust");
        let goal = client.add_goal(creator, &course.id, &goal_content);

        (course, goal.goal_id)
    }

    #[test]
    fn test_edit_goal_success() {
        let env = Env::default();
        env.mock_all_auths();
        let creator: Address = Address::generate(&env);
        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        // Setup course and goal, this will create a new Course and CourseGoal in storage
        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Test Course"),
            &String::from_str(&env, "Test Description"),
            &1000_u128,
            &Some(String::from_str(&env, "category")),
            &Some(String::from_str(&env, "language")),
            &Some(String::from_str(&env, "thumbnail_url")),
            &None,
            &None,
        );
        let goal_content = String::from_str(&env, "Learn the basics of Rust");
        // The `add_goal` function should return the newly created CourseGoal
        let goal = client.add_goal(&creator, &course.id, &goal_content);
        let updated_content = String::from_str(&env, "Master advanced Rust");
        // Use the 'goal.id' which is a Soroban String representing the UUID
        let edited_goal = client.edit_goal(&creator, &course.id, &goal.goal_id, &updated_content);
        assert_eq!(edited_goal.content, updated_content);
        assert_eq!(edited_goal.course_id, course.id);
        assert_eq!(edited_goal.created_by, creator);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #6)")]
    fn test_edit_goal_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();
        let creator: Address = Address::generate(&env);
        let impostor: Address = Address::generate(&env);

        let contract_id = env.register(CourseRegistry, {});

        let client = CourseRegistryClient::new(&env, &contract_id);

        let (course, goal_id) = setup_course_and_goal(&env, &client, &creator);

        let updated_content = String::from_str(&env, "Updated content");
        client.edit_goal(&impostor, &course.id, &goal_id, &updated_content);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #18)")]
    fn test_edit_goal_empty_content() {
        let env = Env::default();
        env.mock_all_auths();
        let creator: Address = Address::generate(&env);

        let contract_id = env.register(CourseRegistry, {});

        let client = CourseRegistryClient::new(&env, &contract_id);

        let (course, goal_id) = setup_course_and_goal(&env, &client, &creator);

        client.edit_goal(&creator, &course.id, &goal_id, &String::from_str(&env, ""));
    }

    #[test]
    #[should_panic(expected = "Course not found")]
    fn test_edit_goal_course_not_found() {
        let env = Env::default();
        env.mock_all_auths();
        let creator: Address = Address::generate(&env);

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        client.edit_goal(
            &creator,
            &String::from_str(&env, "nonexistent_course"),
            &String::from_str(&env, "goal1"),
            &String::from_str(&env, "Some content"),
        );
    }

    #[test]
    #[should_panic(expected = "Goal not found")]
    fn test_edit_goal_goal_not_found() {
        let env = Env::default();
        env.mock_all_auths();
        let creator: Address = Address::generate(&env);

        let contract_id = env.register(CourseRegistry, {});

        let client = CourseRegistryClient::new(&env, &contract_id);

        let (course, _goal_id) = setup_course_and_goal(&env, &client, &creator);

        client.edit_goal(
            &creator,
            &course.id,
            &String::from_str(&env, "nonexistent_goal"),
            &String::from_str(&env, "Some content"),
        );
    }
}

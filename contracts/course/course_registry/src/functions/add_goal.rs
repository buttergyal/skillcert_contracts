// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

use crate::error::{handle_error, Error};
use crate::functions::utils::{self, trim};
use crate::schema::{Course, CourseGoal, DataKey};

const COURSE_KEY: Symbol = symbol_short!("course");

const GOAL_ADDED_EVENT: Symbol = symbol_short!("goalAdded");

pub fn add_goal(env: Env, creator: Address, course_id: String, content: String) -> CourseGoal {
    creator.require_auth();
    
    // Validate input parameters
    if course_id.is_empty() {
        handle_error(&env, Error::EmptyCourseId);
    }
    
    // Validate goal content - prevent empty or whitespace-only content
    if content.is_empty() || trim(&env, &content).is_empty() {
        handle_error(&env, Error::EmptyGoalContent);
    }
    
    // Check string lengths to prevent extremely long values
    if course_id.len() > 100 {
        handle_error(&env, Error::InvalidCourseId);
    }
    
    if content.len() > 1000 {
        handle_error(&env, Error::InvalidGoalContent);
    }

    // Load course
    let storage_key: (Symbol, String) = (COURSE_KEY, course_id.clone());
    let course: Course = env
        .storage()
        .persistent()
        .get(&storage_key)
        .expect("Course not found");

    // Only creator can add goal (or later: check admin)
    if course.creator != creator {
        handle_error(&env, Error::OnlyCreatorCanAddGoals)
    }

    // Generate a unique goal ID
    let goal_id = utils::generate_unique_id(&env);

    // Create new goal
    let goal: CourseGoal = CourseGoal {
        course_id: course_id.clone(),
        goal_id: goal_id.clone(),
        content: content.clone(),
        created_by: creator.clone(),
        created_at: env.ledger().timestamp(),
    };

    // Save the new goal directly
    env.storage().persistent().set(
        &DataKey::CourseGoal(course_id.clone(), goal_id.clone()),
        &goal,
    );

    // Emit event
    env.events().publish(
        (GOAL_ADDED_EVENT, course_id.clone(), goal_id.clone()),
        content.clone(),
    );

    goal
}

#[cfg(test)]
mod test {
    use crate::schema::Course;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_add_goal_success() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

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
        let goal = client.add_goal(&creator, &course.id, &goal_content);

        assert_eq!(goal.course_id, course.id);
        assert_eq!(goal.content, goal_content);
        assert_eq!(goal.created_by, creator);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #1)")]
    fn test_add_goal_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let impostor: Address = Address::generate(&env);

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
        client.add_goal(&impostor, &course.id, &goal_content);
    }

    #[test]
    #[should_panic(expected = "Course not found")]
    fn test_add_goal_course_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let fake_course_id = String::from_str(&env, "nonexistent_course");

        let goal_content = String::from_str(&env, "Learn the basics of Rust");
        client.add_goal(&creator, &fake_course_id, &goal_content);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #2)")]
    fn test_add_goal_empty_content() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

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

        let goal_content = String::from_str(&env, "");
        client.add_goal(&creator, &course.id, &goal_content);
    }

    #[test]
    fn test_add_multiple_goals() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

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

        let goal_content1 = String::from_str(&env, "Learn the basics of Rust");
        let goal1 = client.add_goal(&creator, &course.id, &goal_content1);

        let goal_content2 = String::from_str(&env, "Understand ownership and borrowing");
        let goal2 = client.add_goal(&creator, &course.id, &goal_content2);

        assert_eq!(goal1.course_id, course.id);
        assert_eq!(goal1.content, goal_content1);
        assert_eq!(goal1.created_by, creator);

        assert_eq!(goal2.course_id, course.id);
        assert_eq!(goal2.content, goal_content2);
        assert_eq!(goal2.created_by, creator);

        assert!(goal2.created_at >= goal1.created_at);
    }
}

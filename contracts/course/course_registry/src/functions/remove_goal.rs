use crate::schema::{Course, CourseGoal, DataKey};
use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

const GOAL_REMOVED_EVENT: Symbol = symbol_short!("goalrem");

pub fn course_registry_remove_goal(
    env: Env,
    caller: Address,
    course_id: String,
    goal_id: String,
) -> () {
    caller.require_auth();
    
    // Validate input
    if goal_id.is_empty() {
        panic!("Goal ID cannot be empty");
    }

    // Load course to verify it exists and check permissions
    let storage_key = (symbol_short!("course"), course_id.clone());
    let course: Course = env
        .storage()
        .persistent()
        .get(&storage_key)
        .expect("Course not found");

    // Only course creator or authorized admin can remove goals
    if course.creator != caller {
        // TODO: Add admin check when admin management is implemented
        panic!("Only the course creator can remove goals");
    }

    // Check if the goal exists
    let goal_storage_key = DataKey::CourseGoal(course_id.clone(), goal_id.clone());
    let goal: CourseGoal = env
        .storage()
        .persistent()
        .get(&goal_storage_key)
        .expect("Goal not found");

    // Verify the goal belongs to the specified course
    if goal.course_id != course_id {
        panic!("Goal does not belong to the specified course");
    }

    // Remove the goal from storage
    env.storage().persistent().remove(&goal_storage_key);

    // Emit event for successful goal removal
    env.events().publish(
        (GOAL_REMOVED_EVENT, course_id.clone(), goal_id.clone()),
        goal.content.clone(),
    );
}

#[cfg(test)]
mod test {
    use crate::schema::Course;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_remove_goal_success() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        // Create a course
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

        // Add a goal first
        let goal_content = String::from_str(&env, "Learn the basics of Rust");
        let goal = client.add_goal(&creator, &course.id, &goal_content);

        // Remove the goal
        client.remove_goal(&creator, &course.id, &goal.goal_id);
    }

    #[test]
    #[should_panic(expected = "Only the course creator can remove goals")]
    fn test_remove_goal_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let impostor: Address = Address::generate(&env);

        // Create a course
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

        // Add a goal
        let goal_content = String::from_str(&env, "Learn the basics of Rust");
        let goal = client.add_goal(&creator, &course.id, &goal_content);

        // Try to remove the goal as an impostor
        client.remove_goal(&impostor, &course.id, &goal.goal_id);
    }

    #[test]
    #[should_panic(expected = "Course not found")]
    fn test_remove_goal_course_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let fake_course_id = String::from_str(&env, "nonexistent_course");
        let fake_goal_id = String::from_str(&env, "fake_goal_id");

        client.remove_goal(&creator, &fake_course_id, &fake_goal_id);
    }

    #[test]
    #[should_panic(expected = "Goal not found")]
    fn test_remove_goal_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        // Create a course
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

        let fake_goal_id = String::from_str(&env, "nonexistent_goal");
        client.remove_goal(&creator, &course.id, &fake_goal_id);
    }

    #[test]
    #[should_panic(expected = "Goal ID cannot be empty")]
    fn test_remove_goal_empty_goal_id() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        // Create a course
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

        let empty_goal_id = String::from_str(&env, "");
        client.remove_goal(&creator, &course.id, &empty_goal_id);
    }

    #[test]
    fn test_remove_multiple_goals() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        // Create a course
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

        // Add multiple goals
        let goal_content1 = String::from_str(&env, "Learn the basics of Rust");
        let goal1 = client.add_goal(&creator, &course.id, &goal_content1);

        let goal_content2 = String::from_str(&env, "Understand ownership and borrowing");
        let goal2 = client.add_goal(&creator, &course.id, &goal_content2);

        let goal_content3 = String::from_str(&env, "Master error handling");
        let goal3 = client.add_goal(&creator, &course.id, &goal_content3);

        // Remove goals in different order
        client.remove_goal(&creator, &course.id, &goal2.goal_id); // Remove middle goal
        client.remove_goal(&creator, &course.id, &goal1.goal_id); // Remove first goal
        client.remove_goal(&creator, &course.id, &goal3.goal_id); // Remove last goal

        // All goals should be removed successfully
    }
}

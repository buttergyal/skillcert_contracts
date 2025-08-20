use soroban_sdk::{Env, String, Symbol};

use crate::schema::Course;

pub fn course_registry_get_course(env: &Env, course_id: String) -> Course {
    // Create the storage key for the course
    let key = Symbol::new(env, "course");

    // Get the course from storage
    let course: Course = env
        .storage()
        .persistent()
        .get(&(key, course_id.clone()))
        .expect("Course not found");

    match course.is_archived {
        true => panic!("Course is archived"),
        false => course,
    }
}

#[cfg(test)]
mod test {
    use crate::{schema::Course, CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};
    
    fn create_course<'a>(client: &CourseRegistryClient<'a>, creator: &Address) -> Course {
        let title = String::from_str(&client.env, "title");
        let description = String::from_str(&client.env, "description");
        let price = 1000_u128;
        client.create_course(&creator, &title, &description, &price, &None, &None, &None, &None, &None)
    }

    #[test]
    fn test_get_course_success() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);
        let fetched_course = client.get_course(&course.id);
        assert_eq!(fetched_course, course);
    }

    #[test]
    #[should_panic(expected = "Course not found")]
    fn test_get_course_not_found() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        client.get_course(&String::from_str(&env, "1"));
    }

    #[test]
    #[should_panic(expected = "Course is archived")]
    fn test_get_archived_course() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);
        client.archive_course(&creator, &course.id);
        client.get_course(&course.id);
    }
}
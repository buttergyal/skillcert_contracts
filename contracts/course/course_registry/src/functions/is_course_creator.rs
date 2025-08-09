use crate::schema::Course;
use soroban_sdk::{symbol_short, Address, Env, String, Symbol};
const COURSE_KEY: Symbol = symbol_short!("course");

pub fn is_course_creator(env: &Env, course_id: String, user: Address) -> bool {
    let key = (COURSE_KEY, course_id.clone());
    let course: Course = env.storage().persistent().get(&key).unwrap();

    if course.creator == user {
        return true;
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::Course;
    use crate::CourseRegistry;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    #[test]
    fn test_is_cource_creator_success() {
        let env = Env::default();

        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 1000;
        let category: Option<String> = Some(String::from_str(&env, "Programming"));
        let language: Option<String> = Some(String::from_str(&env, "English"));
        let thumbnail_url: Option<String> =
            Some(String::from_str(&env, "https://example.com/thumb.jpg"));

        env.as_contract(&contract_id, || {
            CourseRegistry::create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                category.clone(),
                language.clone(),
                thumbnail_url.clone(),
            );
            // Verify course storage
            let storage_key: (Symbol, String) = (COURSE_KEY, String::from_str(&env, "1"));
            let stored_course: Option<Course> = env.storage().persistent().get(&storage_key);
            let course = stored_course.expect("Course should be stored");
            let id: String = String::from_str(&env, "1");
            let caller = env.current_contract_address();
            let is_creator = CourseRegistry::is_course_creator(&env, id.clone(), caller.clone());

            assert_eq!(course.id, id);
            assert!(is_creator);
        });
    }

    #[test]
    fn test_is_cource_creator_fail() {
        let env = Env::default();

        let contract_id: Address = env.register(CourseRegistry, {});
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 1000;
        let category: Option<String> = Some(String::from_str(&env, "Programming"));
        let language: Option<String> = Some(String::from_str(&env, "English"));
        let thumbnail_url: Option<String> =
            Some(String::from_str(&env, "https://example.com/thumb.jpg"));

        env.as_contract(&contract_id, || {
            CourseRegistry::create_course(
                env.clone(),
                title.clone(),
                description.clone(),
                price,
                category.clone(),
                language.clone(),
                thumbnail_url.clone(),
            );
            // Verify course storage
            let storage_key: (Symbol, String) = (COURSE_KEY, String::from_str(&env, "1"));
            let stored_course: Option<Course> = env.storage().persistent().get(&storage_key);
            let course = stored_course.expect("Course should be stored");
            let id: String = String::from_str(&env, "1");
            let not_caller = Address::generate(&env);

            let is_creator =
                CourseRegistry::is_course_creator(&env, id.clone(), not_caller.clone());

            assert_eq!(course.id, id);
            assert!(!is_creator);
        });
    }
}

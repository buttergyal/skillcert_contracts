// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use super::utils::u32_to_string;
use crate::schema::Course;
use soroban_sdk::{symbol_short, Address, Env, Symbol, Vec, String};

const COURSE_KEY: Symbol = symbol_short!("course");

pub fn get_courses_by_instructor(env: &Env, instructor: Address) -> Vec<Course> {
    let mut results: Vec<Course> = Vec::new(env);
    let mut id: u128 = 1;

    loop {
        let course_id: String = u32_to_string(env, id as u32);
        let key: (Symbol, String) = (COURSE_KEY, course_id.clone());

        if !env.storage().persistent().has(&key) {
            break;
        }

        let course: Course = env.storage().persistent().get(&key).unwrap();

        if course.creator == instructor && !course.is_archived {
            results.push_back(course);
        }

        id += 1;
        if id > crate::schema::MAX_LOOP_GUARD as u128 {
            break; // safety limit
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn create_course<'a>(
        client: &CourseRegistryClient<'a>,
        creator: &Address,
        title: &str,
    ) -> Course {
        let title = String::from_str(&client.env, title);
        let description = String::from_str(&client.env, "description");
        let price = 1000_u128;
        client.create_course(
            &creator,
            &title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        )
    }

    #[test]
    fn test_get_courses_by_instructor() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let instructor1 = Address::generate(&env);
        let instructor2 = Address::generate(&env);

        let course1 = create_course(&client, &instructor1, "course1");
        let course2 = create_course(&client, &instructor2, "course2");
        let course3 = create_course(&client, &instructor1, "course3");

        let instructor1_courses = client.get_courses_by_instructor(&instructor1);
        assert_eq!(instructor1_courses.len(), 2);
        assert_eq!(instructor1_courses.get(0).unwrap(), course1);
        assert_eq!(instructor1_courses.get(1).unwrap(), course3);

        let instructor2_courses = client.get_courses_by_instructor(&instructor2);
        assert_eq!(instructor2_courses.len(), 1);
        assert_eq!(instructor2_courses.get(0).unwrap(), course2);
    }

    #[test]
    fn test_get_courses_by_instructor_no_courses() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let instructor = Address::generate(&env);

        let courses = client.get_courses_by_instructor(&instructor);
        assert_eq!(courses.len(), 0);
    }

    #[test]
    fn test_get_courses_by_instructor_with_archived() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let instructor = Address::generate(&env);

        let course1 = create_course(&client, &instructor, "course1");
        let course2 = create_course(&client, &instructor, "course2");

        client.archive_course(&instructor, &course2.id);
        let courses = client.get_courses_by_instructor(&instructor);
        assert_eq!(courses.len(), 1);
        assert_eq!(courses.get(0).unwrap(), course1);
    }
}

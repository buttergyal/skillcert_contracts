// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{symbol_short, Address, Env, String, Symbol, Vec};

use crate::error::{handle_error, Error};
use crate::schema::{Course, CourseLevel};
use super::utils::{to_lowercase, trim, u32_to_string};

const COURSE_KEY: Symbol = symbol_short!("course");
const TITLE_KEY: Symbol = symbol_short!("title");
const COURSE_ID: Symbol = symbol_short!("course");

const CREATE_COURSE_EVENT: Symbol = symbol_short!("crtCourse");
const GENERATE_COURSE_ID_EVENT: Symbol = symbol_short!("genCrsId");

pub fn create_course(
    env: Env,
    creator: Address,
    title: String,
    description: String,
    price: u128,
    category: Option<String>,
    language: Option<String>,
    thumbnail_url: Option<String>,
    level: Option<CourseLevel>,
    duration_hours: Option<u32>,
) -> Course {
    creator.require_auth();

    // ensure the title is not empty and not just whitespace
    let trimmed_title: String = trim(&env, &title);
    if title.is_empty() || trimmed_title.is_empty() {
        handle_error(&env, Error::EmptyCourseTitle);
    }

    // Additional title validation
    if title.len() > 200 {
        handle_error(&env, Error::InvalidTitleLength);
    }

    // Validate description - only check length, allow empty
    if description.len() > 2000 {
        handle_error(&env, Error::InvalidCourseDescription);
    }

    // ensure the price is greater than 0
    if price == 0 {
        handle_error(&env, Error::InvalidPrice);
    }

    // Validate optional parameters
    if let Some(ref cat) = category {
        if cat.is_empty() || cat.len() > 100 {
            handle_error(&env, Error::EmptyCategory);
        }
    }

    if let Some(ref lang) = language {
        if lang.is_empty() || lang.len() > 50 {
            handle_error(&env, Error::InvalidLanguageLength);
        }
    }

    if let Some(ref url) = thumbnail_url {
        if url.is_empty() || url.len() > 500 {
            handle_error(&env, Error::InvalidThumbnailUrlLength);
        }
    }

    if let Some(duration) = duration_hours {
        if duration == 0 || duration > 8760 {
            // 8760 hours = 1 year, reasonable maximum
            handle_error(&env, Error::InvalidDurationValue);
        }
    }

    let lowercase_title: String = to_lowercase(&env, &title);

    // to avoid duplicate title,
    let title_key: (Symbol, String) = (TITLE_KEY, lowercase_title);

    if env.storage().persistent().has(&title_key) {
        handle_error(&env, Error::DuplicateCourseTitle)
    }

    // generate the unique id
    let id: u128 = generate_course_id(&env);
    let converted_id: String = u32_to_string(&env, id as u32);

    let storage_key: (Symbol, String) = (COURSE_KEY, converted_id.clone());

    if env.storage().persistent().has(&storage_key) {
        handle_error(&env, Error::DuplicateCourseId)
    }

    // create a new course
    let new_course: Course = Course {
        id: converted_id.clone(),
        title: title.clone(),
        description: description.clone(),
        creator: creator.clone(),
        price,
        category: category.clone(),
        language: language.clone(),
        thumbnail_url: thumbnail_url.clone(),
        published: false,
        prerequisites: Vec::new(&env),
        is_archived: false,
        level: level.clone(),
        duration_hours,
    };

    // save to the storage
    env.storage().persistent().set(&storage_key, &new_course);
    env.storage().persistent().set(&title_key, &true);

    // emit an event
    env.events()
        .publish((CREATE_COURSE_EVENT,), (converted_id, creator, title, description, price, category, language, thumbnail_url, level, duration_hours));

    new_course
}

pub fn generate_course_id(env: &Env) -> u128 {
    let current_id: u128 = env.storage().persistent().get(&COURSE_ID).unwrap_or(0);
    let new_id: u128 = current_id + 1;
    env.storage().persistent().set(&COURSE_ID, &new_id);

    // emit an event
    env.events()
        .publish((GENERATE_COURSE_ID_EVENT,), new_id);

    new_id
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::schema::Course;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_generate_course_id() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id: Address = env.register(CourseRegistry, {});

        env.as_contract(&contract_id, || {
            generate_course_id(&env);
            let id: u128 = generate_course_id(&env);
            assert_eq!(id, 2);
        });
    }

    #[test]
    fn test_add_course_success() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let title = String::from_str(&env, "title");
        let description = String::from_str(&env, "description");
        let price = 1000_u128;
        let category = Some(String::from_str(&env, "category"));
        let language = Some(String::from_str(&env, "language"));
        let thumbnail_url = Some(String::from_str(&env, "thumbnail_url"));
        let course: Course = client.create_course(
            &creator,
            &title,
            &description,
            &price,
            &category,
            &language,
            &thumbnail_url,
            &None,
            &None,
        );
        let course = client.get_course(&course.id);
        assert_eq!(course.title, title);
        assert_eq!(course.description, description);
        assert_eq!(course.id, String::from_str(&env, "1"));
        assert_eq!(course.price, price);
        assert_eq!(course.category, category);
        assert_eq!(course.language, language);
        assert_eq!(course.thumbnail_url, thumbnail_url);
        assert!(!course.published);
    }

    #[test]
    fn test_add_course_success_multiple() {
        let env: Env = Env::default();
        env.mock_all_auths();

        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = crate::schema::DEFAULT_COURSE_PRICE;

        let another_course_title: String = String::from_str(&env, "another title");
        let another_course_description: String = String::from_str(&env, "another description");
        let another_price: u128 = 2000;

        client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let course2 = client.create_course(
            &Address::generate(&env),
            &another_course_title,
            &another_course_description,
            &another_price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let stored_course = client.get_course(&course2.id);

        assert_eq!(stored_course.title, another_course_title);
        assert_eq!(stored_course.description, another_course_description);
        assert_eq!(stored_course.id, String::from_str(&env, "2"));
        assert_eq!(stored_course.price, another_price);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #10)")]
    fn test_cannot_create_courses_with_duplicate_title() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "title");
        let description: String = String::from_str(&env, "A description");
        let another_description: String = String::from_str(&env, "another description");
        let price: u128 = crate::schema::DEFAULT_COURSE_PRICE;

        client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        client.create_course(
            &Address::generate(&env),
            &title,
            &another_description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #8)")]
    fn test_cannot_create_courses_with_empty_title() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = crate::schema::DEFAULT_COURSE_PRICE;

        client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #9)")]
    fn test_cannot_create_courses_with_zero_price() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "Valid Title");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 0;

        client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #8)")]
    fn test_cannot_create_courses_with_whitespace_only_title() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "   ");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = crate::schema::DEFAULT_COURSE_PRICE;

        client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #10)")]
    fn test_duplicate_title_case_insensitive() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title1: String = String::from_str(&env, "Programming Basics");
        let title2: String = String::from_str(&env, "PROGRAMMING BASICS");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = crate::schema::DEFAULT_COURSE_PRICE;

        client.create_course(
            &Address::generate(&env),
            &title1,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
        client.create_course(
            &Address::generate(&env),
            &title2,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    fn test_create_course_with_long_title() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let long_title: String = String::from_str(&env, "This is a very long course title that contains many words and should still be valid for course creation as long as it is not empty");
        let description: String = String::from_str(&env, "A description");
        let price: u128 = 1500;

        let course = client.create_course(
            &Address::generate(&env),
            &long_title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
        assert_eq!(course.title, long_title);
        assert_eq!(course.price, price);
        assert_eq!(course.id, String::from_str(&env, "1"));
    }

    #[test]
    fn test_create_course_with_special_characters() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "C++ & JavaScript: Advanced Programming!");
        let description: String = String::from_str(
            &env,
            "Learn C++ and JavaScript with special symbols: @#$%^&*()",
        );
        let price: u128 = 2500;

        let course = client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
        assert_eq!(course.title, title);
        assert_eq!(course.description, description);
        assert_eq!(course.price, price);
    }

    #[test]
    fn test_create_course_with_maximum_price() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "Premium Course");
        let description: String = String::from_str(&env, "Most expensive course");
        let max_price: u128 = u128::MAX;

        let course = client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &max_price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
        assert_eq!(course.price, max_price);
        assert_eq!(course.title, title);
    }

    #[test]
    fn test_create_course_with_all_optional_fields() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "Complete Course");
        let description: String = String::from_str(&env, "Course with all fields");
        let price: u128 = 3000;
        let category: Option<String> = Some(String::from_str(&env, "Web Development"));
        let language: Option<String> = Some(String::from_str(&env, "Spanish"));
        let thumbnail_url: Option<String> = Some(String::from_str(
            &env,
            "https://example.com/course-thumbnail.png",
        ));
        let level: Option<CourseLevel> = Some(String::from_str(&env, "Intermediate"));
        let duration_hours: Option<u32> = Some(40);

        let course = client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &category,
            &language,
            &thumbnail_url,
            &level,
            &duration_hours,
        );
        assert_eq!(course.title, title);
        assert_eq!(course.description, description);
        assert_eq!(course.price, price);
        assert_eq!(course.category, category);
        assert_eq!(course.language, language);
        assert_eq!(course.thumbnail_url, thumbnail_url);
        assert_eq!(course.level, level);
        assert_eq!(course.duration_hours, duration_hours);
        assert!(!course.published);
    }

    #[test]
    fn test_create_course_with_partial_optional_fields() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "Partial Course");
        let description: String = String::from_str(&env, "Course with some optional fields");
        let price: u128 = 1800;
        let category: Option<String> = Some(String::from_str(&env, "Data Science"));

        let course = client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &category,
            &None,
            &None,
            &None,
            &None,
        );
        assert_eq!(course.title, title);
        assert_eq!(course.price, price);
        assert_eq!(course.category, category);
        assert_eq!(course.language, None);
        assert_eq!(course.thumbnail_url, None);
    }

    #[test]
    fn test_create_course_empty_description() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "Course with Empty Description");
        let description: String = String::from_str(&env, "");
        let price: u128 = 1200;

        let course = client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
        assert_eq!(course.title, title);
        assert_eq!(course.description, description);
        assert_eq!(course.price, price);
    }

    #[test]
    fn test_create_multiple_courses_sequential_ids() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let price: u128 = crate::schema::DEFAULT_COURSE_PRICE;

        let course1 = client.create_course(
            &Address::generate(&env),
            &String::from_str(&env, "Course One"),
            &String::from_str(&env, "First course"),
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let course2 = client.create_course(
            &Address::generate(&env),
            &String::from_str(&env, "Course Two"),
            &String::from_str(&env, "Second course"),
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let course3 = client.create_course(
            &Address::generate(&env),
            &String::from_str(&env, "Course Three"),
            &String::from_str(&env, "Third course"),
            &price,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        assert_eq!(course1.id, String::from_str(&env, "1"));
        assert_eq!(course2.id, String::from_str(&env, "2"));
        assert_eq!(course3.id, String::from_str(&env, "3"));
    }

    #[test]
    fn test_create_course_with_unicode_characters() {
        let env: Env = Env::default();
        env.mock_all_auths();
        let contract_id: Address = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);
        let title: String = String::from_str(&env, "Programación en Español 🚀");
        let description: String = String::from_str(
            &env,
            "Curso de programación con caracteres especiales: áéíóú ñ",
        );
        let price: u128 = 2000;
        let language: Option<String> = Some(String::from_str(&env, "Español"));

        let course = client.create_course(
            &Address::generate(&env),
            &title,
            &description,
            &price,
            &None,
            &language,
            &None,
            &None,
            &None,
        );
        assert_eq!(course.title, title);
        assert_eq!(course.description, description);
        assert_eq!(course.language, language);
    }
}

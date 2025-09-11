// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::{Course, EditCourseParams};
use crate::error::{Error, handle_error};
use soroban_sdk::{symbol_short, Address, Env, String, Symbol};
use super::utils::{to_lowercase, trim};

const COURSE_KEY: Symbol = symbol_short!("course");
const TITLE_KEY: Symbol = symbol_short!("title");

const EDIT_COURSE_EVENT: Symbol = symbol_short!("editcours");

pub fn course_registry_edit_course(
    env: Env,
    creator: Address,
    course_id: String,
    params: EditCourseParams,
) -> Course {
    creator.require_auth();

    // --- Load existing course ---
    let storage_key: (Symbol, String) = (COURSE_KEY, course_id.clone());
    let mut course: Course = env
        .storage()
        .persistent()
        .get(&storage_key)
        .expect("Course error: Course not found");

    // --- Permission: only creator can edit ---
    if creator != course.creator {
        handle_error(&env, Error::Unauthorized)
    }

    // --- Title update (validate + uniqueness) ---

    if let Some(t) = params.new_title {
        // Clone the string to avoid move issues
        let t_str = t.clone();
        let t_trim = trim(&env, &t_str);
      
        if t_trim.is_empty() {
            handle_error(&env, Error::EmptyCourseTitle)
        }

        // Only check/rotate title index if it's effectively changing (case-insensitive)
        let old_title_lc = to_lowercase(&env, &course.title);
        let new_title_lc = to_lowercase(&env, &t_str);

        if old_title_lc != new_title_lc {
            // uniqueness index key for the *new* title
            let new_title_key: (Symbol, String) =
                (TITLE_KEY, new_title_lc);
            if env.storage().persistent().has(&new_title_key) {
                handle_error(&env, Error::DuplicateCourseTitle)
            }

            // remove old title index and set new one
            let old_title_key: (Symbol, String) =
                (TITLE_KEY, old_title_lc);
            env.storage().persistent().remove(&old_title_key);
            env.storage().persistent().set(&new_title_key, &true);

            course.title = t_trim;
        }
    }

    // --- Description ---
    if let Some(d) = params.new_description {
        course.description = d;
    }

    // --- Price (>0) ---
    if let Some(p) = params.new_price {
        if p == 0 {
            handle_error(&env, Error::InvalidPrice)
        }
        course.price = p;
    }

    // --- Optional fields: category / language / thumbnail ---
    if let Some(cat) = params.new_category {
        course.category = cat; // Some(value) sets; None clears
    }
    if let Some(lang) = params.new_language {
        course.language = lang;
    }
    if let Some(url) = params.new_thumbnail_url {
        course.thumbnail_url = url;
    }

    // --- Published flag ---
    if let Some(p) = params.new_published {
        course.published = p;
    }

    // --- Level field ---
    if let Some(level) = params.new_level {
        course.level = level; // Some(value) sets; None clears
    }

    // --- Duration hours field ---
    if let Some(duration) = params.new_duration_hours {
        course.duration_hours = duration; // Some(value) sets; None clears
    }

    // --- Persist updated course ---
    env.storage().persistent().set(&storage_key, &course);

    // --- Emit event ---
    env.events()
        .publish((EDIT_COURSE_EVENT, course_id.clone()), course.clone());

    course
}

#[cfg(test)]
mod test {
    use crate::schema::{Course, EditCourseParams};
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_edit_course_success() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &Some(String::from_str(&env, "original_category")),
            &Some(String::from_str(&env, "original_language")),
            &Some(String::from_str(&env, "original_thumbnail")),
            &None,
            &None,
        );

        let params = EditCourseParams {
            new_title: Some(String::from_str(&env, "New Title")),
            new_description: Some(String::from_str(&env, "New Description")),
            new_price: Some(2000_u128),
            new_category: Some(Some(String::from_str(&env, "new_category"))),
            new_language: Some(Some(String::from_str(&env, "new_language"))),
            new_thumbnail_url: Some(Some(String::from_str(&env, "new_thumbnail"))),
            new_published: Some(true),
            new_level: None,
            new_duration_hours: None,
        };
        let edited_course = client.edit_course(
            &creator,
            &course.id,
            &params,
        );

        assert_eq!(edited_course.title, String::from_str(&env, "New Title"));
        assert_eq!(
            edited_course.description,
            String::from_str(&env, "New Description")
        );
        assert_eq!(edited_course.price, 2000_u128);
        assert_eq!(
            edited_course.category,
            Some(String::from_str(&env, "new_category"))
        );
        assert_eq!(
            edited_course.language,
            Some(String::from_str(&env, "new_language"))
        );
        assert_eq!(
            edited_course.thumbnail_url,
            Some(String::from_str(&env, "new_thumbnail"))
        );
        assert_eq!(edited_course.published, true);
        assert_eq!(edited_course.creator, creator);

        let retrieved_course = client.get_course(&course.id);
        assert_eq!(retrieved_course.title, String::from_str(&env, "New Title"));
        assert_eq!(
            retrieved_course.description,
            String::from_str(&env, "New Description")
        );
        assert_eq!(retrieved_course.price, 2000_u128);
        assert_eq!(
            retrieved_course.category,
            Some(String::from_str(&env, "new_category"))
        );
        assert_eq!(
            retrieved_course.language,
            Some(String::from_str(&env, "new_language"))
        );
        assert_eq!(
            retrieved_course.thumbnail_url,
            Some(String::from_str(&env, "new_thumbnail"))
        );
        assert_eq!(retrieved_course.published, true);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #6)")]
    fn test_edit_course_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let impostor: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let params = EditCourseParams {
            new_title: Some(String::from_str(&env, "New Title")),
            new_description: None,
            new_price: None,
            new_category: None,
            new_language: None,
            new_thumbnail_url: None,
            new_published: None,
            new_level: None,
            new_duration_hours: None,
        };
        client.edit_course(
            &impostor,
            &course.id,
            &params,
        );
    }

    #[test]
    #[should_panic(expected = "Course error: Course not found")]
    fn test_edit_course_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let fake_course_id = String::from_str(&env, "nonexistent_course");

        let params = EditCourseParams {
            new_title: Some(String::from_str(&env, "New Title")),
            new_description: None,
            new_price: None,
            new_category: None,
            new_language: None,
            new_thumbnail_url: None,
            new_published: None,
            new_level: None,
            new_duration_hours: None,
        };
        client.edit_course(
            &creator,
            &fake_course_id,
            &params,
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #8)")]
    fn test_edit_course_empty_title() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let params = EditCourseParams {
            new_title: Some(String::from_str(&env, "")),
            new_description: None,
            new_price: None,
            new_category: None,
            new_language: None,
            new_thumbnail_url: None,
            new_published: None,
            new_level: None,
            new_duration_hours: None,
        };
        client.edit_course(
            &creator,
            &course.id,
            &params,
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #9)")]
    fn test_edit_course_zero_price() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let params = EditCourseParams {
            new_title: None,
            new_description: None,
            new_price: Some(0_u128),
            new_category: None,
            new_language: None,
            new_thumbnail_url: None,
            new_published: None,
            new_level: None,
            new_duration_hours: None,
        };
        client.edit_course(
            &creator,
            &course.id,
            &params,
        );
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Contract, #10)")]
    fn test_edit_course_duplicate_title() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let _course1: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Course 1"),
            &String::from_str(&env, "Description 1"),
            &1000_u128,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let course2: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Course 2"),
            &String::from_str(&env, "Description 2"),
            &1000_u128,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let params = EditCourseParams {
            new_title: Some(String::from_str(&env, "Course 1")),
            new_description: None,
            new_price: None,
            new_category: None,
            new_language: None,
            new_thumbnail_url: None,
            new_published: None,
            new_level: None,
            new_duration_hours: None,
        };
        client.edit_course(
            &creator,
            &course2.id,
            &params,
        );
    }

    #[test]
    fn test_edit_course_partial_fields() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        // Create a course
        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &Some(String::from_str(&env, "original_category")),
            &Some(String::from_str(&env, "original_language")),
            &Some(String::from_str(&env, "original_thumbnail")),
            &None,
            &None,
        );

        let params = EditCourseParams {
            new_title: Some(String::from_str(&env, "New Title")),
            new_description: None,
            new_price: Some(2000_u128),
            new_category: None,
            new_language: None,
            new_thumbnail_url: None,
            new_published: None,
            new_level: None,
            new_duration_hours: None,
        };
        let edited_course = client.edit_course(
            &creator,
            &course.id,
            &params,
        );

        assert_eq!(edited_course.title, String::from_str(&env, "New Title"));
        assert_eq!(
            edited_course.description,
            String::from_str(&env, "Original Description")
        );
        assert_eq!(edited_course.price, 2000_u128);
        assert_eq!(
            edited_course.category,
            Some(String::from_str(&env, "original_category"))
        );
        assert_eq!(
            edited_course.language,
            Some(String::from_str(&env, "original_language"))
        );
        assert_eq!(
            edited_course.thumbnail_url,
            Some(String::from_str(&env, "original_thumbnail"))
        );
        assert_eq!(edited_course.published, false); // Default value, unchanged
    }

    #[test]
    fn test_edit_course_same_title_no_change() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let params = EditCourseParams {
            new_title: Some(String::from_str(&env, "original title")), // Same title, different case
            new_description: Some(String::from_str(&env, "New Description")),
            new_price: None,
            new_category: None,
            new_language: None,
            new_thumbnail_url: None,
            new_published: None,
            new_level: None,
            new_duration_hours: None,
        };
        let edited_course = client.edit_course(
            &creator,
            &course.id,
            &params,
        );

        assert_eq!(
            edited_course.title,
            String::from_str(&env, "Original Title")
        );
        assert_eq!(
            edited_course.description,
            String::from_str(&env, "New Description")
        );
    }
}
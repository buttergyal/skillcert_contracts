use crate::error::{handle_error, Error};
use crate::functions::utils::u32_to_string;

use crate::schema::{Course, CourseFilters, MAX_EMPTY_CHECKS};
use soroban_sdk::{symbol_short, Env, Symbol, Vec, String};

/// Helper function to check if a Soroban String contains a substring
/// For now, this implements exact match only due to Soroban String limitations
/// TODO: Implement proper substring search when Soroban provides better string utilities
fn string_contains(haystack: &String, needle: &String) -> bool {
    // For now, only exact match is supported
    // This can be enhanced later when Soroban provides better string utilities
    haystack == needle
}

const COURSE_KEY: Symbol = symbol_short!("course");

pub fn list_courses_with_filters(
    env: &Env,
    filters: CourseFilters,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Vec<Course> {
    // Validate pagination parameters to prevent abuse
    if let Some(l) = limit {
        if l > 100 {
            // Prevent excessively large limits
            handle_error(env, Error::InvalidLimitValue)
        }
    }
    if let Some(o) = offset {
        if o > 10000 {
            // Prevent excessively large offsets
            handle_error(env, Error::InvalidOffsetValue)
        }
    }

    let mut results: Vec<Course> = Vec::new(env);
    let mut id: u128 = 1;
    let mut count: u32 = 0;
    let mut matched: u32 = 0;
    let mut empty_checks: u32 = 0;

    let offset_value: u32 = offset.unwrap_or(0);
    let limit_value: u32 = limit.unwrap_or(10); // Reduced default limit for budget

    // Safety check for limit - reduced for budget constraints
    let max_limit: u32 = if limit_value > 20 { 20 } else { limit_value };

    loop {
        // Much more aggressive safety limits for budget
        if id > crate::schema::MAX_SCAN_ID as u128
            || empty_checks > MAX_EMPTY_CHECKS as u32
        {
            break;
        }

        // Use the utility function instead of to_string()
        let course_id: String = u32_to_string(env, id as u32);
        let key: (Symbol, String) = (COURSE_KEY, course_id.clone());

        if !env.storage().persistent().has(&key) {
            empty_checks += 1;
            id += 1;
            continue;
        }

        // Reset empty checks when we find a course
        empty_checks = 0;

        let course: Course = env.storage().persistent().get(&key).unwrap();

        // Skip archived or unpublished courses
        if course.is_archived || !course.published {
            id += 1;
            continue;
        }

        // Apply filters with early exits for performance.
        //
        // - Price range filter (min/max)
        // - Category filter
        // - Level filter
        // - Duration filter (min/max, only if course has duration)
        // - Text search filter (title and description)
        let passes_filters: bool = filters.min_price.map_or(true, |min| course.price >= min)
            && filters.max_price.map_or(true, |max| course.price <= max)
            && filters
                .category
                .as_ref()
                .is_none_or(|cat| course.category.as_ref() == Some(cat))
            && filters
                .level
                .as_ref()
                .map_or(true, |lvl| course.level.as_ref() == Some(lvl))
            && filters.min_duration.map_or(true, |min| {
                course.duration_hours.map_or(false, |d| d >= min)
            })
            && filters.max_duration.map_or(true, |max| {
                course.duration_hours.map_or(false, |d| d <= max)
            })
            && filters.search_text.as_ref().map_or(true, |search| {
                // Text search in title and description
                // Note: Case-sensitive search due to Soroban String limitations
                string_contains(&course.title, search) || string_contains(&course.description, search)
            });

        // If course passes all filters
        if passes_filters {
            // Handle pagination
            if matched >= offset_value {
                if count < max_limit {
                    results.push_back(course);
                    count += 1;
                } else {
                    // We've reached the limit
                    break;
                }
            }
            matched += 1;
        }

        id += 1;
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_empty_list_no_courses() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);

        // Test with no courses - should return empty
        let filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
            search_text: None,
        };

        let results = client.list_courses_with_filters(&filters, &None, &None);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_single_course_no_filters() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);
        let creator = Address::generate(&env);

        // Create one course
        let course = client.create_course(
            &creator,
            &String::from_str(&env, "Test Course"),
            &String::from_str(&env, "Description"),
            &100,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        // Publish the course so it appears in filtered results
        use crate::schema::EditCourseParams;
        let params = EditCourseParams {
            new_title: None,
            new_description: None,
            new_price: None,
            new_category: None,
            new_language: None,
            new_thumbnail_url: None,
            new_published: Some(true),
            new_level: None,
            new_duration_hours: None,
        };
        client.edit_course(&creator, &course.id, &params);

        // No filters - should return the course
        let filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
            search_text: None,
        };

        let results = client.list_courses_with_filters(&filters, &None, &None);
        assert_eq!(results.len(), 1);
        assert_eq!(results.get(0).unwrap().price, 100);
    }

    #[test]
    fn test_price_filter_excludes_course() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);
        let creator = Address::generate(&env);

        // Create one course with price 100
        client.create_course(
            &creator,
            &String::from_str(&env, "Cheap Course"),
            &String::from_str(&env, "Description"),
            &100,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        // Filter for expensive courses - should return empty
        let filters = CourseFilters {
            min_price: Some(crate::schema::FILTER_MIN_PRICE),
            max_price: Some(crate::schema::DEFAULT_COURSE_PRICE),
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
            search_text: None,
        };

        let results = client.list_courses_with_filters(&filters, &None, &None);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_pagination_limit() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);
        let creator = Address::generate(&env);

        // Create one course
        client.create_course(
            &creator,
            &String::from_str(&env, "Course 1"),
            &String::from_str(&env, "Description"),
            &100,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        // Test limit = 0 should return empty
        let filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
            search_text: None,
        };

        let results = client.list_courses_with_filters(&filters, &Some(0), &None);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_text_search_filter() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);
        let creator = Address::generate(&env);

        // Create courses with different titles and descriptions
        let course1 = client.create_course(
            &creator,
            &String::from_str(&env, "Rust Programming"),
            &String::from_str(&env, "Learn Rust language fundamentals"),
            &100,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        let course2 = client.create_course(
            &creator,
            &String::from_str(&env, "JavaScript Basics"),
            &String::from_str(&env, "Introduction to web development"),
            &150,
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        // Publish both courses
        use crate::schema::EditCourseParams;
        let publish_params = EditCourseParams {
            new_title: None,
            new_description: None,
            new_price: None,
            new_category: None,
            new_language: None,
            new_thumbnail_url: None,
            new_published: Some(true),
            new_level: None,
            new_duration_hours: None,
        };
        client.edit_course(&creator, &course1.id, &publish_params);
        client.edit_course(&creator, &course2.id, &publish_params);

        // Search for exact title match - should return only first course
        let exact_title_filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
            search_text: Some(String::from_str(&env, "Rust Programming")),
        };

        let exact_title_results = client.list_courses_with_filters(&exact_title_filters, &None, &None);
        assert_eq!(exact_title_results.len(), 1);
        assert_eq!(exact_title_results.get(0).unwrap().title, String::from_str(&env, "Rust Programming"));

        // Search for exact description match - should return only second course
        let exact_desc_filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
            search_text: Some(String::from_str(&env, "Introduction to web development")),
        };

        let exact_desc_results = client.list_courses_with_filters(&exact_desc_filters, &None, &None);
        assert_eq!(exact_desc_results.len(), 1);
        assert_eq!(exact_desc_results.get(0).unwrap().title, String::from_str(&env, "JavaScript Basics"));

        // Search for non-existent term
        let none_filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
            search_text: Some(String::from_str(&env, "Python")),
        };

        let none_results = client.list_courses_with_filters(&none_filters, &None, &None);
        assert_eq!(none_results.len(), 0);
    }
}

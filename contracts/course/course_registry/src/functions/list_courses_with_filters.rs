use crate::schema::{Course, CourseFilters};
use soroban_sdk::{symbol_short, Env, String, Symbol, Vec};

const COURSE_KEY: Symbol = symbol_short!("course");

pub fn course_registry_list_courses_with_filters(
    env: &Env,
    filters: CourseFilters,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Vec<Course> {
    let mut results: Vec<Course> = Vec::new(env);
    let mut id: u128 = 1;
    let mut count: u32 = 0;
    let mut matched: u32 = 0;
    
    let offset_value = offset.unwrap_or(0);
    let limit_value = limit.unwrap_or(50); // Default limit of 50
    
    // Safety check for limit
    let max_limit = if limit_value > 100 { 100 } else { limit_value };
    
    loop {
        // Safety limit to prevent infinite loop
        if id > 1000 {
            break;
        }
        
        let course_id = String::from_str(env, &id.to_string());
        let key = (COURSE_KEY, course_id.clone());
        
        if !env.storage().persistent().has(&key) {
            id += 1;
            continue;
        }
        
        let course: Course = env.storage().persistent().get(&key).unwrap();
        
        // Skip archived or unpublished courses
        if course.is_archived || !course.published {
            id += 1;
            continue;
        }
        
        // Apply filters
        let mut passes_filters = true;
        
        // Price range filter
        if let Some(min_price) = filters.min_price {
            if course.price < min_price {
                passes_filters = false;
            }
        }
        if passes_filters {
            if let Some(max_price) = filters.max_price {
                if course.price > max_price {
                    passes_filters = false;
                }
            }
        }
        
        // Category filter
        if passes_filters {
            if let Some(ref filter_category) = filters.category {
                match &course.category {
                    Some(course_category) => {
                        if course_category != filter_category {
                            passes_filters = false;
                        }
                    }
                    None => passes_filters = false,
                }
            }
        }
        
        // Level filter
        if passes_filters {
            if let Some(ref filter_level) = filters.level {
                match &course.level {
                    Some(course_level) => {
                        if course_level != filter_level {
                            passes_filters = false;
                        }
                    }
                    None => passes_filters = false,
                }
            }
        }
        
        // Duration filter
        if passes_filters {
            if let Some(min_duration) = filters.min_duration {
                match course.duration_hours {
                    Some(duration) => {
                        if duration < min_duration {
                            passes_filters = false;
                        }
                    }
                    None => passes_filters = false,
                }
            }
        }
        if passes_filters {
            if let Some(max_duration) = filters.max_duration {
                match course.duration_hours {
                    Some(duration) => {
                        if duration > max_duration {
                            passes_filters = false;
                        }
                    }
                    None => passes_filters = false,
                }
            }
        }
        
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
    use crate::schema::CourseLevel;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn create_test_course<'a>(
        client: &CourseRegistryClient<'a>,
        creator: &Address,
        title: &str,
        price: u128,
        category: Option<String>,
        level: Option<CourseLevel>,
        duration_hours: Option<u32>,
    ) -> Course {
        let title = String::from_str(&client.env, title);
        let description = String::from_str(&client.env, "Test course description");
        
        // Create the course first with basic fields
        let mut course = client.create_course(
            &creator,
            &title,
            &description,
            &price,
            &category,
            &None, // language
            &None, // thumbnail_url
            &level,
            &duration_hours,
        );
        
        course
    }

    #[test]
    fn test_filter_by_price_range() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        
        // Create courses with different prices
        create_test_course(&client, &creator, "Cheap Course", 100, None, None, None);
        create_test_course(&client, &creator, "Medium Course", 500, None, None, None);
        create_test_course(&client, &creator, "Expensive Course", 1000, None, None, None);
        
        // Filter for courses between 200 and 600
        let filters = CourseFilters {
            min_price: Some(200),
            max_price: Some(600),
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
        };
        
        let results = client.list_courses_with_filters(&filters, &None, &None);
        assert_eq!(results.len(), 1);
        assert_eq!(results.get(0).unwrap().price, 500);
    }
    
    #[test]
    fn test_filter_by_category() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let programming = Some(String::from_str(&env, "Programming"));
        let design = Some(String::from_str(&env, "Design"));
        
        create_test_course(&client, &creator, "Rust Course", 500, programming.clone(), None, None);
        create_test_course(&client, &creator, "Python Course", 400, programming.clone(), None, None);
        create_test_course(&client, &creator, "UI/UX Course", 600, design.clone(), None, None);
        
        // Filter for Programming category
        let filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: programming.clone(),
            level: None,
            min_duration: None,
            max_duration: None,
        };
        
        let results = client.list_courses_with_filters(&filters, &None, &None);
        assert_eq!(results.len(), 2);
    }
    
    #[test]
    fn test_filter_by_level() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        
        create_test_course(&client, &creator, "Intro Course", 300, None, Some(CourseLevel::Beginner), None);
        create_test_course(&client, &creator, "Advanced Course", 800, None, Some(CourseLevel::Advanced), None);
        create_test_course(&client, &creator, "Mid Course", 500, None, Some(CourseLevel::Intermediate), None);
        
        // Filter for Beginner level
        let filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: None,
            level: Some(CourseLevel::Beginner),
            min_duration: None,
            max_duration: None,
        };
        
        let results = client.list_courses_with_filters(&filters, &None, &None);
        assert_eq!(results.len(), 1);
    }
    
    #[test]
    fn test_filter_by_duration() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        
        create_test_course(&client, &creator, "Short Course", 200, None, None, Some(5));
        create_test_course(&client, &creator, "Medium Course", 300, None, None, Some(20));
        create_test_course(&client, &creator, "Long Course", 400, None, None, Some(40));
        
        // Filter for courses between 10 and 30 hours
        let filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: None,
            level: None,
            min_duration: Some(10),
            max_duration: Some(30),
        };
        
        let results = client.list_courses_with_filters(&filters, &None, &None);
        assert_eq!(results.len(), 1);
        assert_eq!(results.get(0).unwrap().duration_hours, Some(20));
    }
    
    #[test]
    fn test_combined_filters() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let programming = Some(String::from_str(&env, "Programming"));
        
        // Create diverse courses
        create_test_course(&client, &creator, "Perfect Match", 500, programming.clone(), Some(CourseLevel::Intermediate), Some(20));
        create_test_course(&client, &creator, "Wrong Price", 100, programming.clone(), Some(CourseLevel::Intermediate), Some(20));
        create_test_course(&client, &creator, "Wrong Category", 500, None, Some(CourseLevel::Intermediate), Some(20));
        create_test_course(&client, &creator, "Wrong Level", 500, programming.clone(), Some(CourseLevel::Beginner), Some(20));
        
        // Apply multiple filters
        let filters = CourseFilters {
            min_price: Some(400),
            max_price: Some(600),
            category: programming.clone(),
            level: Some(CourseLevel::Intermediate),
            min_duration: Some(15),
            max_duration: Some(25),
        };
        
        let results = client.list_courses_with_filters(&filters, &None, &None);
        assert_eq!(results.len(), 1);
    }
    
    #[test]
    fn test_pagination() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        
        // Create 5 courses
        for i in 0..5 {
            let title = format!("Course {}", i);
            create_test_course(&client, &creator, &title, 500, None, None, None);
        }
        
        // No filters, just pagination
        let filters = CourseFilters {
            min_price: None,
            max_price: None,
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
        };
        
        // Get first 2 courses
        let page1 = client.list_courses_with_filters(&filters, &Some(2), &Some(0));
        assert_eq!(page1.len(), 2);
        
        // Get next 2 courses
        let page2 = client.list_courses_with_filters(&filters, &Some(2), &Some(2));
        assert_eq!(page2.len(), 2);
        
        // Get last course
        let page3 = client.list_courses_with_filters(&filters, &Some(2), &Some(4));
        assert_eq!(page3.len(), 1);
    }
    
    #[test]
    fn test_no_results() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, ());
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        
        create_test_course(&client, &creator, "Course", 500, None, None, None);
        
        // Filter with impossible criteria
        let filters = CourseFilters {
            min_price: Some(1000),
            max_price: Some(2000),
            category: None,
            level: None,
            min_duration: None,
            max_duration: None,
        };
        
        let results = client.list_courses_with_filters(&filters, &None, &None);
        assert_eq!(results.len(), 0);
    }
}
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::{schema::Category, CourseRegistry, CourseRegistryClient};
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, String, Vec};

use crate::{
    functions::{
        get_prerequisites_by_course::get_prerequisites_by_course_id,
        list_categories::course_registry_list_categories,
    },
    schema::Course,
};

#[test]
fn test_remove_module_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CourseRegistry, {});
    let client = CourseRegistryClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let course: Course = client.create_course(
        &creator,
        &String::from_str(&env, "title"),
        &String::from_str(&env, "description"),
        &1000_u128,
        &Some(String::from_str(&env, "category")),
        &Some(String::from_str(&env, "language")),
        &Some(String::from_str(&env, "thumbnail_url")),
        &None,
        &None,
    );
    let new_module = client.add_module(&course.id, &0, &String::from_str(&env, "Module Title"));

    let exists: bool = env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .has(&(symbol_short!("module"), new_module.id.clone()))
    });
    assert!(exists);

    client.remove_module(&new_module.id);
    let exists: bool = env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .has(&(symbol_short!("module"), new_module.id.clone()))
    });
    assert!(!exists);
}

#[test]
fn test_remove_multiple_different_modules() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CourseRegistry, {});
    let client = CourseRegistryClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let course: Course = client.create_course(
        &creator,
        &String::from_str(&env, "title"),
        &String::from_str(&env, "description"),
        &1000_u128,
        &Some(String::from_str(&env, "category")),
        &Some(String::from_str(&env, "language")),
        &Some(String::from_str(&env, "thumbnail_url")),
        &None,
        &None,
    );
    let module1 = client.add_module(&course.id, &0, &String::from_str(&env, "Module 1 Title"));
    let module2 = client.add_module(&course.id, &1, &String::from_str(&env, "Module 2 Title"));

    client.remove_module(&module1.id.clone());
    client.remove_module(&module2.id.clone());

    let module1_exists: bool = env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .has(&(symbol_short!("module"), module1.id.clone()))
    });
    assert!(!module1_exists);

    let module2_exists: bool = env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .has(&(symbol_short!("module"), module2.id.clone()))
    });
    assert!(!module2_exists);
}

#[test]
fn test_remove_module_storage_isolation() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CourseRegistry, {});
    let client = CourseRegistryClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let course: Course = client.create_course(
        &creator,
        &String::from_str(&env, "title"),
        &String::from_str(&env, "description"),
        &1000_u128,
        &Some(String::from_str(&env, "category")),
        &Some(String::from_str(&env, "language")),
        &Some(String::from_str(&env, "thumbnail_url")),
        &None,
        &None,
    );
    let module1 = client.add_module(&course.id, &0, &String::from_str(&env, "Module 1 Title"));
    let module2 = client.add_module(&course.id, &1, &String::from_str(&env, "Module 2 Title"));

    client.remove_module(&module1.id.clone());

    let module1_exists: bool = env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .has(&(symbol_short!("module"), module1.id.clone()))
    });
    assert!(!module1_exists);

    let module2_exists: bool = env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .has(&(symbol_short!("module"), module2.id.clone()))
    });
    assert!(module2_exists);
}

#[test]
fn test_get_course_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CourseRegistry, ());
    let client = CourseRegistryClient::new(&env, &contract_id);

    let creator: Address = Address::generate(&env);
    let course = client.create_course(
        &creator,
        &String::from_str(&env, "Course 1"),
        &String::from_str(&env, "description"),
        &1000,
        &None,
        &None,
        &None,
        &None,
        &None,
    );

    let retrieved = client.get_course(&course.id);

    assert_eq!(retrieved.id, course.id);
    assert_eq!(retrieved.title, course.title);
    assert_eq!(retrieved.description, course.description);
    assert_eq!(retrieved.creator, course.creator);
    assert_eq!(retrieved.published, course.published);
}

#[test]
#[should_panic(expected = "Course not found")]
fn test_get_course_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CourseRegistry, {});
    let client = CourseRegistryClient::new(&env, &contract_id);

    let fake_id = String::from_str(&env, "not_found");
    client.get_course(&fake_id);
}

#[test]
fn test_get_courses_by_instructor_empty() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CourseRegistry, {});
    let client = CourseRegistryClient::new(&env, &contract_id);

    let courses = client.get_courses_by_instructor(&Address::generate(&env));

    assert_eq!(courses.len(), 0);
}

#[test]
fn test_get_courses_by_instructor_found() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CourseRegistry, {});
    let client = CourseRegistryClient::new(&env, &contract_id);

    let creator: Address = Address::generate(&env);
    let course = client.create_course(
        &creator,
        &String::from_str(&env, "Course 1"),
        &String::from_str(&env, "description"),
        &1000,
        &None,
        &None,
        &None,
        &None,
        &None,
    );

    let results = client.get_courses_by_instructor(&creator);

    assert_eq!(results.len(), 1);
    assert_eq!(results.get(0).unwrap().id, course.id);
}

#[test]
fn test_get_prerequisites_by_course_id() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CourseRegistry, {});
    let client = CourseRegistryClient::new(&env, &contract_id);

    let creator: Address = Address::generate(&env);
    let course = client.create_course(
        &creator,
        &String::from_str(&env, "Course 1"),
        &String::from_str(&env, "description"),
        &1000,
        &None,
        &None,
        &None,
        &None,
        &None,
    );

    let prerequisites = env.as_contract(&contract_id, || {
        get_prerequisites_by_course_id(&env, course.id.clone())
    });
    assert!(prerequisites.is_empty());
}

#[test]
fn test_list_categories_counts() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(CourseRegistry, ());
    let client = CourseRegistryClient::new(&env, &contract_id);
    let creator = Address::generate(&env);

    client.create_course(
        &creator,
        &String::from_str(&env, "A"),
        &String::from_str(&env, "d"),
        &10,
        &Some(String::from_str(&env, "Programming")),
        &None,
        &None,
        &None,
        &None,
    );
    client.create_course(
        &creator,
        &String::from_str(&env, "B"),
        &String::from_str(&env, "d"),
        &10,
        &Some(String::from_str(&env, "Data")),
        &None,
        &None,
        &None,
        &None,
    );
    client.create_course(
        &creator,
        &String::from_str(&env, "C"),
        &String::from_str(&env, "d"),
        &10,
        &Some(String::from_str(&env, "Programming")),
        &None,
        &None,
        &None,
        &None,
    );

    // Call the function to list categories
    let cats = client.list_categories();
    assert_eq!(cats.len(), 2); // Should have 2 unique categories

    // Verify counts without assuming order
    let mut prog = 0u128;
    let mut data = 0u128;
    for c in cats.iter() {
        let cname = c.name.clone();
        if cname == String::from_str(&env, "Programming") {
            prog = c.count;
        }
        if cname ==  String::from_str(&env, "Data") {
            data = c.count;
        }
    }
    assert_eq!(prog, 2);
    assert_eq!(data, 1);
}

#[test]
fn test_list_categories_empty() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(CourseRegistry, ());
    // No courses created, should return empty vector
    let cats: Vec<Category> = env.as_contract(&contract_id, || course_registry_list_categories(&env));
    assert_eq!(cats.len(), 0);
}

#[test]
fn test_list_categories_ignores_none() {
    let env: Env = Env::default();
    env.mock_all_auths();
    
    let contract_id: Address = env.register(CourseRegistry, ());
    let client = CourseRegistryClient::new(&env, &contract_id);
    let creator: Address = Address::generate(&env);

    client.create_course(
        &creator,
        &String::from_str(&env, "B"),
        &String::from_str(&env, "d"),
        &10,
        &Some(String::from_str(&env, "Programming")),
        &None,
        &None,
        &None,
        &None,
    );

    let cats = client.list_categories();
    assert_eq!(cats.len(), 1); // Only "Programming" should be returned
    let c = cats.get(0).unwrap();
    assert_eq!(c.name, String::from_str(&env, "Programming"));
    assert_eq!(c.count, 1);
}

#[test]
fn test_list_categories_with_id_gaps() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(CourseRegistry, ());
    let client = CourseRegistryClient::new(&env, &contract_id);
    let creator = Address::generate(&env);

    client.create_course(
        &creator,
        &String::from_str(&env, "Course 1"),
        &String::from_str(&env, "Desc"),
        &10,
        &Some(String::from_str(&env, "Programming")),
        &None,
        &None,
        &None,
        &None,
    );
    client.create_course(
        &creator,
        &String::from_str(&env, "Course 2"),
        &String::from_str(&env, "Desc"),
        &10,
        &Some(String::from_str(&env, "Data")),
        &None,
        &None,
        &None,
        &None,
    );

    // Manually delete course 2 to create an ID gap
    env.as_contract(&contract_id, || {
        let key = (symbol_short!("course"), String::from_str(&env, "2"));
        env.storage().persistent().remove(&key);
    });

    // Create course 3 (this will still have ID 3)
    client.create_course(
        &creator,
        &String::from_str(&env, "Course 3"),
        &String::from_str(&env, "Desc"),
        &10,
        &Some(String::from_str(&env, "Programming")),
        &None,
        &None,
        &None,
        &None,
    );

    // Call the function - it should skip missing ID 2 but still count 1 and 3
    let cats = client.list_categories();
    let mut prog = 0u128;
    let mut data = 0u128;
    for c in cats.iter() {
        if c.name == String::from_str(&env, "Programming") {
            prog = c.count;
        }
        if c.name == String::from_str(&env, "Data") {
            data = c.count;
        }
    }
    assert_eq!(prog, 2); // Course 1 and Course 3
    assert_eq!(data, 0); // Course 2 was deleted
}
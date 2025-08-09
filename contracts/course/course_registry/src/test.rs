use crate::{CourseRegistry, CourseRegistryClient};
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env, String};

use crate::{
    functions::get_prerequisites_by_course::get_prerequisites_by_course_id, schema::Course,
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
    );

    let prerequisites = env.as_contract(&contract_id, || {
        get_prerequisites_by_course_id(&env, course.id.clone())
    });
    assert!(prerequisites.is_empty());
}

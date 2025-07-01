


use soroban_sdk::{symbol_short, Address, Env, String, Symbol, U256};
use crate::schema::{Course, CourseId};

const COURSE_KEY: Symbol = symbol_short!("course");
const COURSE_ID_KEY: Symbol = symbol_short!("course_id");

pub fn course_registry_create_course(env: Env, title: String, description: String) {

    // generate a course id
    // use the title to generate the id
    let course_id: String = String::from_bytes(&env, &generate_course_id(env.clone()).to_be_bytes());
    // let caller: Address = Address::from_string("strkey");
    let caller: Address = env.current_contract_address();

    caller.require_auth();

    let storage_key: (Symbol, String) = (COURSE_KEY, course_id.clone());

    if env.storage().persistent().has(&storage_key) {
        panic!("Course with this ID already exists");
    }

    // create a new course
    let new_course: Course = Course {
        id: course_id.clone(),
        title: title,
        description: description,
        creator: caller,
        published: false,
    };

    // save to the storage
    env.storage().persistent().set(&storage_key, &new_course);
}

fn generate_course_id(env: Env) -> u128 {

    let optional_course_id: Option<CourseId> = env.storage().persistent().get(&COURSE_ID_KEY);

    let mut course_id: CourseId = optional_course_id.unwrap_or_else(|| CourseId {
        id: String::from_str(&env, "course_id"),
        count: 0,
    });

    let current_id = course_id.count;

    course_id.count += 1;

    env.storage().persistent().set(&COURSE_ID_KEY, &course_id);

    current_id
}
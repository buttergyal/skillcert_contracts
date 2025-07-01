

use soroban_sdk::{symbol_short, Address, Env, String, Symbol};
use crate::schema::Course;

const COURSE_KEY: Symbol = symbol_short!("course");

pub fn course_registry_create_course(env: Env, title: String, description: String) {

    // generate a course id
    let timestamp:u64 = env.ledger().timestamp();
    let caller: Address = env.invoker();
    let course_id:String = format!("{}-{}", title, timestamp);

    let storage_key: (Symbol, String) = (COURSE_KEY, course_id.clone());

    if env.storage().persistent().has(&storage_key) {
        panic!("Course with this ID already exists");
    }

    // create a new course
    let new_course = Course {
        id: course_id,
        title: title,
        description: description,
        creator: caller,
        published: false,
    };

    // save to the storage
    env.storage().persistent().set(&storage_key, &new_course);
}
use soroban_sdk::{Env, String, Symbol};

use crate::schema::Course;

pub fn course_registry_get_course(env: &Env, course_id: String) -> Course {
    // Create the storage key for the course
    let key = Symbol::new(env, "course");

    // Get the course from storage
    let course: Course = env
        .storage()
        .instance()
        .get(&(key, course_id.clone()))
        .expect("Course not found");

    course
}

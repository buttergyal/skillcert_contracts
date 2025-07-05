pub mod schema;
pub mod functions;

use soroban_sdk::{contract, contractimpl, Env, String};

use crate::schema::Course;

#[contract]
pub struct CourseRegistry;

#[contractimpl]
impl CourseRegistry {
    pub fn get_course(env: Env, course_id: String) -> Course {
        functions::get_course::course_registry_get_course(&env, course_id)
    }
}

#[cfg(test)]
mod test;

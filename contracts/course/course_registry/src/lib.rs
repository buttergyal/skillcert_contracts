pub mod schema;
pub mod functions;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Env, String};

use crate::schema::{Course, CourseModule};

#[contract]
pub struct CourseRegistry;

#[contractimpl]
impl CourseRegistry {

    pub fn create_course(
        env: Env,
        title: String,
        description: String,
    ) -> Course {
        functions::create_course::course_registry_create_course(env, title, description)
    }

    pub fn get_course(env: Env, course_id: String) -> Course {
        functions::get_course::course_registry_get_course(&env, course_id)
    }

    pub fn remove_module(env: Env, module_id: String) -> Result<(), &'static str> {
        functions::remove_module::course_registry_remove_module(&env, module_id)
    }

    pub fn add_module(
        env: Env,
        course_id: String,
        position: i32,
        title: String,
    ) -> CourseModule {
        functions::add_module::course_registry_add_module(env, course_id, position, title)
    }

    pub fn delete_course(env: Env, course_id: String) -> Result<(), &'static str> {
        functions::delete_course::course_registry_delete_course(&env, course_id)
    }
}

#[cfg(test)]
mod test;

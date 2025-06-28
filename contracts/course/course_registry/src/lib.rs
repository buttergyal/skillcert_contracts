use soroban_sdk::{contract, contractimpl, Env, Address, String};

mod functions;
mod schema;

pub use functions::*;
pub use schema::*;

#[contract]
pub struct CourseRegistry;

#[contractimpl]
impl CourseRegistry {
    pub fn add_module(env: Env, course_id: String, position: i32, title: String) -> CourseModule {
        course_registry_add_module(env, course_id, position, title)
    }
}

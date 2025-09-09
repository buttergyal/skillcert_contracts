#![no_std]

pub mod functions;
pub mod schema;

#[cfg(test)]
mod test;

use crate::schema::{Course, CourseFilters, CourseGoal, CourseLevel, CourseModule, EditCourseParams, CourseCategory};
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

#[contract]
pub struct CourseRegistry;

#[contractimpl]
impl CourseRegistry {
    pub fn create_course(
        env: Env,
        creator: Address,
        title: String,
        description: String,
        price: u128,
        category: Option<String>,
        language: Option<String>,
        thumbnail_url: Option<String>,
        level: Option<CourseLevel>,
        duration_hours: Option<u32>,
    ) -> Course {
        functions::create_course::course_registry_create_course(
            env,
            creator,
            title,
            description,
            price,
            category,
            language,
            thumbnail_url,
            level,
            duration_hours,
        )
    }

    pub fn get_course(env: Env, course_id: String) -> Course {
        functions::get_course::course_registry_get_course(&env, course_id)
    }

    pub fn get_courses_by_instructor(env: Env, instructor: Address) -> Vec<Course> {
        functions::get_courses_by_instructor::course_registry_get_courses_by_instructor(
            &env, instructor,
        )
    }

    pub fn remove_module(env: Env, module_id: String) -> () {
        functions::remove_module::course_registry_remove_module(&env, module_id)
            .unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn add_module(env: Env, course_id: String, position: u32, title: String) -> CourseModule {
        functions::add_module::course_registry_add_module(env, course_id, position, title)
    }

    pub fn delete_course(env: Env, creator: Address, course_id: String) -> () {
        functions::delete_course::course_registry_delete_course(&env, creator, course_id)
            .unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn hello_world(_env: Env) -> String {
        String::from_str(&_env, "Hello from Web3 👋")
    }

    pub fn edit_goal(
        env: Env,
        creator: Address,
        course_id: String,
        goal_id: String,
        new_content: String,
    ) -> CourseGoal {
        functions::edit_goal::course_registry_edit_goal(env, creator, course_id, goal_id, new_content)
    }

    pub fn add_goal(env: Env, creator: Address, course_id: String, content: String) -> CourseGoal {
        functions::add_goal::course_registry_add_goal(env, creator, course_id, content)
    }

    pub fn remove_goal(env: Env, caller: Address, course_id: String, goal_id: String) -> () {
        functions::remove_goal::course_registry_remove_goal(env, caller, course_id, goal_id)
    }

    pub fn add_prerequisite(
        env: Env,
        creator: Address,
        course_id: String,
        prerequisite_course_ids: Vec<String>,
    ) {
        functions::create_prerequisite::course_registry_add_prerequisite(
            env,
            creator,
            course_id,
            prerequisite_course_ids,
        )
    }

    pub fn remove_prerequisite(
        env: Env,
        creator: Address,
        course_id: String,
        prerequisite_course_id: String,
    ) {
        functions::remove_prerequisite::course_registry_remove_prerequisite(
            env,
            creator,
            course_id,
            prerequisite_course_id,
        )
    }

    pub fn edit_prerequisite(
        env: Env,
        creator: Address,
        course_id: String,
        new_prerequisites: Vec<String>,
    ) {
        functions::edit_prerequisite::course_registry_edit_prerequisite(
            env,
            creator,
            course_id,
            new_prerequisites,
        )
    }

    pub fn edit_course(
        env: Env,
        creator: Address,
        course_id: String,
        params: EditCourseParams,
    ) -> Course {
        functions::edit_course::course_registry_edit_course(
            env,
            creator,
            course_id,
            params,
        )
    }

    pub fn archive_course(env: &Env, creator: Address, course_id: String) -> Course {
        functions::archive_course::course_registry_archive_course(env, creator, course_id)
    }

    pub fn is_course_creator(env: &Env, course_id: String, user: Address) -> bool {
        functions::is_course_creator::is_course_creator(env, course_id, user)
    }

    pub fn list_categories(env: Env) -> Vec<crate::schema::Category> {
        functions::list_categories::course_registry_list_categories(&env)
    }

    pub fn list_courses_with_filters(
        env: Env,
        filters: CourseFilters,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Vec<Course> {
        functions::list_courses_with_filters::course_registry_list_courses_with_filters(
            &env,
            filters,
            limit,
            offset,
        )
    }

    /// Create a new course category (admin-only)
    pub fn create_course_category(
        env: Env,
        caller: Address,
        name: String,
        description: Option<String>,
    ) -> u128 {
        functions::create_course_category::course_registry_create_course_category(
            env,
            caller,
            name,
            description,
        )
    }

    /// Get a course category by ID
    pub fn get_course_category(env: Env, category_id: u128) -> Option<CourseCategory> {
        functions::get_course_category::course_registry_get_course_category(&env, category_id)
    }
}
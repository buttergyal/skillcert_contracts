// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

/// Contract version for tracking deployments and upgrades
pub const VERSION: &str = "1.0.0";

pub mod error;
pub mod functions;
pub mod schema;

#[cfg(test)]
mod test;

use crate::schema::{
    Course, CourseCategory, CourseFilters, CourseGoal, CourseLevel, CourseModule, EditCourseParams,
};
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

/// Course Registry Contract
///
/// This contract manages the creation, modification, and querying of courses
/// in the SkillCert platform. It handles course metadata, categories, modules,
/// goals, prerequisites, and provides comprehensive course management functionality.
#[contract]
pub struct CourseRegistry;

#[contractimpl]
impl CourseRegistry {
    /// Create a new course in the registry.
    ///
    /// This function creates a new course with the specified metadata and
    /// returns the created course object with a unique identifier.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `creator` - The address of the course creator
    /// * `title` - The course title
    /// * `description` - The course description
    /// * `price` - The course price in the platform's currency
    /// * `category` - Optional course category
    /// * `language` - Optional course language
    /// * `thumbnail_url` - Optional URL for the course thumbnail image
    /// * `level` - Optional course difficulty level
    /// * `duration_hours` - Optional estimated duration in hours
    ///
    /// # Returns
    ///
    /// Returns the created `Course` object with all metadata and a unique ID.
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
        functions::create_course::create_course(
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

    /// Create a new course category.
    ///
    /// This function creates a new category that can be used to classify courses.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `caller` - The address of the user creating the category
    /// * `name` - The name of the category
    /// * `description` - Optional description of the category
    ///
    /// # Returns
    ///
    /// Returns the unique ID of the created category.
    pub fn create_course_category(
        env: Env,
        caller: Address,
        name: String,
        description: Option<String>,
    ) -> u128 {
        functions::create_course_category::create_course_category(env, caller, name, description)
    }

    /// Retrieve a course by its ID.
    ///
    /// This function fetches a course's complete information using its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `course_id` - The unique identifier of the course to retrieve
    ///
    /// # Returns
    ///
    /// Returns the `Course` object containing all course metadata.
    pub fn get_course(env: Env, course_id: String) -> Course {
        functions::get_course::get_course(&env, course_id)
    }

    /// Retrieve a course category by its ID.
    ///
    /// This function fetches a category's information using its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `category_id` - The unique identifier of the category to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(CourseCategory)` if found, `None` if the category doesn't exist.
    pub fn get_course_category(env: Env, category_id: u128) -> Option<CourseCategory> {
        functions::get_course_category::get_course_category(&env, category_id)
    }

    /// Get all courses created by a specific instructor.
    ///
    /// This function retrieves all courses that were created by the specified instructor.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `instructor` - The address of the instructor to query courses for
    ///
    /// # Returns
    ///
    /// Returns a vector of `Course` objects created by the instructor.
    pub fn get_courses_by_instructor(env: Env, instructor: Address) -> Vec<Course> {
        functions::get_courses_by_instructor::get_courses_by_instructor(&env, instructor)
    }

    /// Remove a module from a course.
    ///
    /// This function removes a specific module from its associated course.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `module_id` - The unique identifier of the module to remove
    ///
    /// # Panics
    ///
    /// Panics if the module removal fails or if the module doesn't exist.
    pub fn remove_module(env: Env, module_id: String) -> () {
        functions::remove_module::remove_module(&env, module_id).unwrap_or_else(|e| panic!("{}", e))
    }

    /// Add a new module to a course.
    ///
    /// This function creates and adds a new module to the specified course
    /// at the given position.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `course_id` - The unique identifier of the course to add the module to
    /// * `position` - The position where the module should be inserted
    /// * `title` - The title of the new module
    ///
    /// # Returns
    ///
    /// Returns the created `CourseModule` object.
    pub fn add_module(env: Env, caller: Address, course_id: String, position: u32, title: String) -> CourseModule {
        functions::add_module::course_registry_add_module(env, caller, course_id, position, title)
    }

    /// Delete a course from the registry.
    ///
    /// This function permanently removes a course from the registry.
    /// Only the course creator can delete their own courses.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `creator` - The address of the course creator
    /// * `course_id` - The unique identifier of the course to delete
    ///
    /// # Panics
    ///
    /// Panics if the deletion fails or if the creator is not authorized.
    pub fn delete_course(env: Env, creator: Address, course_id: String) -> () {
        functions::delete_course::delete_course(&env, creator, course_id)
            .unwrap_or_else(|e| panic!("{}", e))
    }

    /// Simple hello world function for testing.
    ///
    /// This is a basic function that returns a greeting message,
    /// primarily used for testing contract deployment and basic functionality.
    ///
    /// # Arguments
    ///
    /// * `_env` - The Soroban environment (unused)
    ///
    /// # Returns
    ///
    /// Returns a greeting string.
    pub fn hello_world(_env: Env) -> String {
        String::from_str(&_env, "Hello from Web3 👋")
    }

    /// Edit an existing course goal.
    ///
    /// This function allows the course creator to modify the content of an existing goal.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `creator` - The address of the course creator
    /// * `course_id` - The unique identifier of the course
    /// * `goal_id` - The unique identifier of the goal to edit
    /// * `new_content` - The new content for the goal
    ///
    /// # Returns
    ///
    /// Returns the updated `CourseGoal` object.
    pub fn edit_goal(
        env: Env,
        creator: Address,
        course_id: String,
        goal_id: String,
        new_content: String,
    ) -> CourseGoal {
        functions::edit_goal::edit_goal(env, creator, course_id, goal_id, new_content)
    }

    /// Add a new goal to a course.
    ///
    /// This function creates and adds a new learning goal to the specified course.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `creator` - The address of the course creator
    /// * `course_id` - The unique identifier of the course
    /// * `content` - The content/description of the goal
    ///
    /// # Returns
    ///
    /// Returns the created `CourseGoal` object.
    pub fn add_goal(env: Env, creator: Address, course_id: String, content: String) -> CourseGoal {
        functions::add_goal::add_goal(env, creator, course_id, content)
    }

    /// Remove a goal from a course.
    ///
    /// This function removes a specific learning goal from the course.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `caller` - The address of the user requesting the removal
    /// * `course_id` - The unique identifier of the course
    /// * `goal_id` - The unique identifier of the goal to remove
    pub fn remove_goal(env: Env, caller: Address, course_id: String, goal_id: String) -> () {
        functions::remove_goal::remove_goal(env, caller, course_id, goal_id)
    }

    /// Add prerequisites to a course.
    ///
    /// This function adds prerequisite courses that must be completed
    /// before a student can enroll in the target course.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `creator` - The address of the course creator
    /// * `course_id` - The unique identifier of the course
    /// * `prerequisite_course_ids` - Vector of course IDs that are prerequisites
    pub fn add_prerequisite(
        env: Env,
        creator: Address,
        course_id: String,
        prerequisite_course_ids: Vec<String>,
    ) {
        functions::create_prerequisite::add_prerequisite(
            env,
            creator,
            course_id,
            prerequisite_course_ids,
        )
    }

    /// Remove a prerequisite from a course.
    ///
    /// This function removes a specific prerequisite course requirement
    /// from the target course.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `creator` - The address of the course creator
    /// * `course_id` - The unique identifier of the course
    /// * `prerequisite_course_id` - The ID of the prerequisite course to remove
    pub fn remove_prerequisite(
        env: Env,
        creator: Address,
        course_id: String,
        prerequisite_course_id: String,
    ) {
        functions::remove_prerequisite::remove_prerequisite(
            env,
            creator,
            course_id,
            prerequisite_course_id,
        )
    }

    /// Edit the prerequisites for a course.
    ///
    /// This function replaces all existing prerequisites with a new set
    /// of prerequisite courses.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `creator` - The address of the course creator
    /// * `course_id` - The unique identifier of the course
    /// * `new_prerequisites` - Vector of new prerequisite course IDs
    pub fn edit_prerequisite(
        env: Env,
        creator: Address,
        course_id: String,
        new_prerequisites: Vec<String>,
    ) {
        functions::edit_prerequisite::edit_prerequisite(env, creator, course_id, new_prerequisites)
    }

    /// Edit course information.
    ///
    /// This function allows the course creator to update various aspects
    /// of the course using the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `creator` - The address of the course creator
    /// * `course_id` - The unique identifier of the course to edit
    /// * `params` - Parameters containing the fields to update
    ///
    /// # Returns
    ///
    /// Returns the updated `Course` object.
    pub fn edit_course(
        env: Env,
        creator: Address,
        course_id: String,
        params: EditCourseParams,
    ) -> Course {
        functions::edit_course::edit_course(env, creator, course_id, params)
    }

    /// Archive a course.
    ///
    /// This function marks a course as archived, making it unavailable for new enrollments
    /// while preserving existing data and access for current students.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `creator` - The address of the course creator
    /// * `course_id` - The unique identifier of the course to archive
    ///
    /// # Returns
    ///
    /// Returns the updated `Course` object with archived status.
    pub fn archive_course(env: &Env, creator: Address, course_id: String) -> Course {
        functions::archive_course::archive_course(env, creator, course_id)
    }

    /// Check if a user is the creator of a specific course.
    ///
    /// This function verifies whether the specified user is the original creator
    /// of the given course.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `course_id` - The unique identifier of the course
    /// * `user` - The address of the user to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the user is the course creator, `false` otherwise.
    pub fn is_course_creator(env: &Env, course_id: String, user: Address) -> bool {
        functions::is_course_creator::is_course_creator(env, course_id, user)
    }

    /// List all available course categories.
    ///
    /// This function retrieves all course categories that have been created
    /// in the system.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    ///
    /// # Returns
    ///
    /// Returns a vector of all available `Category` objects.
    pub fn list_categories(env: Env) -> Vec<crate::schema::Category> {
        functions::list_categories::list_categories(&env)
    }

    /// List courses with filtering and pagination.
    ///
    /// This function retrieves courses based on the provided filters
    /// with optional pagination support.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `filters` - Filtering criteria for courses
    /// * `limit` - Optional maximum number of courses to return
    /// * `offset` - Optional number of courses to skip for pagination
    ///
    /// # Returns
    ///
    /// Returns a vector of `Course` objects matching the filter criteria.
    pub fn list_courses_with_filters(
        env: Env,
        filters: CourseFilters,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Vec<Course> {
        functions::list_courses_with_filters::list_courses_with_filters(
            &env, filters, limit, offset,
        )
    }
}

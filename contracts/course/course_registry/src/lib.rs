// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert
#![allow(clippy::too_many_arguments)]
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
    ///
    /// # Panics
    ///
    /// * If title or description are empty
    /// * If creator address is invalid
    /// * If price exceeds maximum allowed value
    ///
    /// # Examples
    ///
    /// ```rust
    /// let course = contract.create_course(
    ///     env.clone(),
    ///     instructor_address,
    ///     "Rust Programming Basics".try_into().unwrap(),
    ///     "Learn Rust from scratch".try_into().unwrap(),
    ///     5000, // price in platform currency
    ///     Some("Programming".try_into().unwrap()),
    ///     Some("en".try_into().unwrap()),
    ///     Some("https://example.com/thumb.jpg".try_into().unwrap()),
    ///     Some(CourseLevel::Beginner),
    ///     Some(40)
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Empty strings**: Title and description cannot be empty
    /// * **Large prices**: Price must be within reasonable bounds
    /// * **Invalid URLs**: Thumbnail URL should be valid if provided
    /// * **Auto-generated ID**: Course ID is automatically generated
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
    ///
    /// # Panics
    ///
    /// * If category name is empty
    /// * If category with same name already exists
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Create a programming category
    /// let category_id = contract.create_course_category(
    ///     env.clone(),
    ///     admin_address,
    ///     "Programming".try_into().unwrap(),
    ///     Some("Computer programming courses".try_into().unwrap())
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Duplicate names**: Cannot create categories with existing names
    /// * **Empty names**: Category name cannot be empty
    /// * **Unique IDs**: Each category gets a unique auto-generated ID
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
    ///
    /// # Panics
    ///
    /// * If course with given ID doesn't exist
    /// * If course_id is invalid or empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Get course by ID
    /// let course = contract.get_course(env.clone(), "course_123".try_into().unwrap());
    /// println!("Course title: {}", course.title);
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Non-existent course**: Will panic if course ID doesn't exist
    /// * **Archived courses**: Still retrievable but marked as archived
    /// * **Public access**: Anyone can retrieve course information
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Get category by ID
    /// if let Some(category) = contract.get_course_category(env.clone(), 1) {
    ///     println!("Category: {}", category.name);
    /// } else {
    ///     println!("Category not found");
    /// }
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Non-existent category**: Returns `None` instead of panicking
    /// * **Invalid ID**: Returns `None` for invalid category IDs
    /// * **Public access**: Anyone can retrieve category information
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Get all courses by an instructor
    /// let instructor_courses = contract.get_courses_by_instructor(env.clone(), instructor_address);
    /// for course in instructor_courses {
    ///     println!("Course: {}", course.title);
    /// }
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **No courses**: Returns empty vector if instructor has no courses
    /// * **Archived courses**: Includes archived courses in results
    /// * **Public access**: Anyone can query instructor courses
    /// * **Invalid instructor**: Returns empty vector for non-existent instructors
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
    /// * If the module doesn't exist
    /// * If the module_id is invalid or empty
    /// * If module removal operation fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Remove a module from a course
    /// contract.remove_module(env.clone(), "module_123".try_into().unwrap());
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Non-existent module**: Will panic if module ID doesn't exist
    /// * **Invalid ID**: Will panic for invalid or empty module IDs
    /// * **Course updates**: Automatically updates course module count
    ///
    /// Panics if the module removal fails or if the module doesn't exist.
    pub fn remove_module(env: Env, module_id: String) {
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If caller is not the course creator
    /// * If module title is empty
    /// * If position is invalid
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Add a module at position 1
    /// let module = contract.add_module(
    ///     env.clone(),
    ///     course_creator_address,
    ///     "course_123".try_into().unwrap(),
    ///     1,
    ///     "Introduction to Variables".try_into().unwrap()
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Invalid position**: Position must be valid for the course
    /// * **Empty title**: Module title cannot be empty
    /// * **Creator only**: Only course creator can add modules
    /// * **Auto-generated ID**: Module gets unique auto-generated ID
    pub fn add_module(
        env: Env,
        caller: Address,
        course_id: String,
        position: u32,
        title: String,
    ) -> CourseModule {
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
    /// * If course doesn't exist
    /// * If creator is not the actual course creator
    /// * If course_id is invalid or empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Course creator deleting their course
    /// contract.delete_course(env.clone(), course_creator_address, "course_123".try_into().unwrap());
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Permission denied**: Only course creator can delete their courses
    /// * **Non-existent course**: Will panic if course doesn't exist
    /// * **Permanent deletion**: Course and all associated data are permanently removed
    /// * **Enrolled students**: Consider impact on enrolled students before deletion
    /// 
    /// Panics if the deletion fails or if the creator is not authorized.
    pub fn delete_course(env: Env, creator: Address, course_id: String) {
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Test contract deployment
    /// let greeting = contract.hello_world(env.clone());
    /// assert_eq!(greeting, "Hello from Web3 👋");
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Always succeeds**: This function never fails
    /// * **No dependencies**: Requires no external data or state
    /// * **Testing only**: Primarily intended for contract testing
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If goal doesn't exist
    /// * If creator is not the course creator
    /// * If new_content is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Edit a course goal
    /// let updated_goal = contract.edit_goal(
    ///     env.clone(),
    ///     course_creator_address,
    ///     "course_123".try_into().unwrap(),
    ///     "goal_456".try_into().unwrap(),
    ///     "Updated learning objective".try_into().unwrap()
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Empty content**: New content cannot be empty
    /// * **Creator only**: Only course creator can edit goals
    /// * **Non-existent goal**: Will panic if goal ID doesn't exist
    /// * **Content validation**: New content must meet validation requirements
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If creator is not the course creator
    /// * If content is empty
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Add a learning goal to a course
    /// let goal = contract.add_goal(
    ///     env.clone(),
    ///     course_creator_address,
    ///     "course_123".try_into().unwrap(),
    ///     "Students will learn basic programming concepts".try_into().unwrap()
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Empty content**: Goal content cannot be empty
    /// * **Creator only**: Only course creator can add goals
    /// * **Auto-generated ID**: Goal gets unique auto-generated ID
    /// * **Content validation**: Goal content must meet validation requirements
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If goal doesn't exist
    /// * If caller is not the course creator
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Remove a goal from a course
    /// contract.remove_goal(
    ///     env.clone(),
    ///     course_creator_address,
    ///     "course_123".try_into().unwrap(),
    ///     "goal_456".try_into().unwrap()
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Creator only**: Only course creator can remove goals
    /// * **Non-existent goal**: Will panic if goal ID doesn't exist
    /// * **Permanent removal**: Goal is permanently deleted from course
    /// * **Goal count**: Automatically updates course goal count
    pub fn remove_goal(env: Env, caller: Address, course_id: String, goal_id: String) {
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If creator is not the course creator
    /// * If any prerequisite course doesn't exist
    /// * If trying to add self as prerequisite
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut prerequisites = Vec::new(&env);
    /// prerequisites.push_back("basic_rust".try_into().unwrap());
    /// prerequisites.push_back("programming_fundamentals".try_into().unwrap());
    /// 
    /// contract.add_prerequisite(
    ///     env.clone(),
    ///     course_creator_address,
    ///     "advanced_rust".try_into().unwrap(),
    ///     prerequisites
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Circular dependencies**: Cannot add self as prerequisite
    /// * **Non-existent courses**: All prerequisite courses must exist
    /// * **Creator only**: Only course creator can add prerequisites
    /// * **Duplicate prerequisites**: Adding same prerequisite multiple times is ignored
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If creator is not the course creator
    /// * If prerequisite doesn't exist for the course
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Remove a prerequisite from a course
    /// contract.remove_prerequisite(
    ///     env.clone(),
    ///     course_creator_address,
    ///     "advanced_rust".try_into().unwrap(),
    ///     "basic_rust".try_into().unwrap()
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Non-existent prerequisite**: Will panic if prerequisite doesn't exist
    /// * **Creator only**: Only course creator can remove prerequisites
    /// * **No effect**: Removing non-existent prerequisite has no effect
    /// * **Student impact**: Consider impact on enrolled students
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If creator is not the course creator
    /// * If any prerequisite course doesn't exist
    /// * If trying to add self as prerequisite
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut new_prerequisites = Vec::new(&env);
    /// new_prerequisites.push_back("updated_course_1".try_into().unwrap());
    /// new_prerequisites.push_back("updated_course_2".try_into().unwrap());
    /// 
    /// contract.edit_prerequisite(
    ///     env.clone(),
    ///     course_creator_address,
    ///     "target_course".try_into().unwrap(),
    ///     new_prerequisites
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Complete replacement**: All old prerequisites are removed
    /// * **Empty vector**: Can clear all prerequisites with empty vector
    /// * **Circular dependencies**: Cannot add self as prerequisite
    /// * **Student impact**: Consider impact on enrolled students
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If creator is not the course creator
    /// * If any field validation fails
    ///
    /// # Examples
    ///
    /// ```rust
    /// let params = EditCourseParams {
    ///     title: Some("Updated Course Title".try_into().unwrap()),
    ///     description: Some("Updated description".try_into().unwrap()),
    ///     price: Some(7500),
    ///     level: Some(CourseLevel::Intermediate),
    ///     ..Default::default()
    /// };
    /// 
    /// let updated_course = contract.edit_course(
    ///     env.clone(),
    ///     course_creator_address,
    ///     "course_123".try_into().unwrap(),
    ///     params
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Partial updates**: Only provided fields are updated
    /// * **Validation**: All fields must pass validation rules
    /// * **Creator only**: Only course creator can edit course
    /// * **Price limits**: Price must be within allowed bounds
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    /// * If creator is not the course creator
    /// * If course is already archived
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Archive a course
    /// let archived_course = contract.archive_course(
    ///     &env,
    ///     course_creator_address,
    ///     "course_123".try_into().unwrap()
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Already archived**: Will panic if course is already archived
    /// * **Creator only**: Only course creator can archive course
    /// * **Student access**: Current students retain access
    /// * **Reversible**: Course can be unarchived if needed
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
    ///
    /// # Panics
    ///
    /// * If course doesn't exist
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Check if user is course creator
    /// let is_creator = contract.is_course_creator(
    ///     &env,
    ///     "course_123".try_into().unwrap(),
    ///     user_address
    /// );
    /// 
    /// if is_creator {
    ///     // User can edit this course
    /// }
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Non-existent course**: Will panic if course doesn't exist
    /// * **Public access**: Anyone can check creator status
    /// * **Creator verification**: Useful for permission checks
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Get all categories
    /// let categories = contract.list_categories(env.clone());
    /// for category in categories {
    ///     println!("Category: {}", category.name);
    /// }
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **Empty system**: Returns empty vector if no categories exist
    /// * **Public access**: Anyone can list categories
    /// * **Order**: Categories are returned in creation order
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// // List first 10 courses
    /// let courses = contract.list_courses_with_filters(
    ///     env.clone(),
    ///     CourseFilters::default(),
    ///     Some(10),
    ///     Some(0)
    /// );
    /// 
    /// // Filter by category
    /// let mut filters = CourseFilters::default();
    /// filters.category = Some("Programming".try_into().unwrap());
    /// let programming_courses = contract.list_courses_with_filters(
    ///     env.clone(),
    ///     filters,
    ///     Some(20),
    ///     None
    /// );
    /// ```
    ///
    /// # Edge Cases
    ///
    /// * **No matches**: Returns empty vector if no courses match filters
    /// * **Large limits**: Limit should be reasonable to avoid gas issues
    /// * **Public access**: Anyone can list courses
    /// * **Archived courses**: May or may not be included based on filter settings
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

    /// Get the current contract version
    ///
    /// Returns the semantic version of the current contract deployment.
    /// This is useful for tracking contract upgrades and compatibility.
    ///
    /// # Arguments
    /// * `_env` - The Soroban environment (unused)
    ///
    /// # Returns
    /// * `String` - The current contract version
    pub fn get_contract_version(_env: Env) -> String {
        String::from_str(&_env, VERSION)
    }

    /// Get contract version history
    ///
    /// Returns a list of all versions that have been deployed for this contract.
    /// This helps track the evolution of the contract over time.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    ///
    /// # Returns
    /// * `Vec<String>` - Vector of version strings in chronological order
    pub fn get_version_history(env: Env) -> Vec<String> {
        functions::contract_versioning::get_version_history(&env)
    }

    /// Check compatibility between contract versions
    ///
    /// Determines if data from one version can be safely used with another version.
    /// This is crucial for migration processes and backward compatibility.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `from_version` - The source version to check compatibility from
    /// * `to_version` - The target version to check compatibility to
    ///
    /// # Returns
    /// * `bool` - True if the versions are compatible, false otherwise
    pub fn is_version_compatible(env: Env, from_version: String, to_version: String) -> bool {
        functions::contract_versioning::is_version_compatible(&env, from_version, to_version)
    }

    /// Migrate course data between contract versions
    ///
    /// Performs data migration from one contract version to another.
    /// This function handles the transformation of course data structures
    /// when upgrading contract versions.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    /// * `caller` - The address performing the migration (must be course creator or admin)
    /// * `from_version` - The source version to migrate from
    /// * `to_version` - The target version to migrate to
    ///
    /// # Returns
    /// * `bool` - True if migration was successful, false otherwise
    ///
    /// # Events
    /// Emits a migration event upon successful completion
    pub fn migrate_course_data(env: Env, caller: Address, from_version: String, to_version: String) -> bool {
        functions::contract_versioning::migrate_course_data(&env, caller, from_version, to_version)
    }

    /// Get migration status for the current contract
    ///
    /// Returns information about the current migration status and any
    /// pending migrations that need to be completed.
    ///
    /// # Arguments
    /// * `env` - The Soroban environment
    ///
    /// # Returns
    /// * `String` - Migration status information
    pub fn get_migration_status(env: Env) -> String {
        functions::contract_versioning::get_migration_status(&env)
    }
}

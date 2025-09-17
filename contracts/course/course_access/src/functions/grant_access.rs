// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::functions::config::{TTL_BUMP, TTL_TTL};
use crate::schema::{CourseAccess, CourseUsers, DataKey, UserCourses};
use soroban_sdk::{Address, Env, String, Vec};

/// Grant access to a specific user for a given course.
///
/// This function creates a new course access entry for the specified user and course.
/// It also updates the user's course list and the course's user list to maintain
/// bidirectional relationships for efficient querying.
///
/// # Arguments
///
/// * `env` - The Soroban environment
/// * `course_id` - The unique identifier of the course to grant access to
/// * `user` - The address of the user to grant access to
///
/// # Panics
///
/// Panics with `Error::UserAlreadyHasAccess` if the user already has access to the course.
pub fn CourseAccessGrantAccess(env: Env, course_id: String, user: Address) {
    // Input validation
    if course_id.is_empty() {
        handle_error(&env, Error::InvalidInput)
    }

    let key: DataKey = DataKey::CourseAccess(course_id.clone(), user.clone());

    // Check if access already exists to prevent duplicates
    if env.storage().persistent().has(&key) {
        handle_error(&env, Error::UserAlreadyHasAccess)
    }

    // Create the course access entry
    let course_access: CourseAccess = CourseAccess {
        course_id: course_id.clone(),
        user: user.clone(),
    };

    // Store the access entry
    env.storage().persistent().set(&key, &course_access);
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_BUMP, TTL_TTL);

    // Update UserCourses
    let user_courses_key = DataKey::UserCourses(user.clone());
    let mut user_courses: UserCourses = env
        .storage()
        .persistent()
        .get(&user_courses_key)
        .unwrap_or(UserCourses {
            user: user.clone(),
            courses: Vec::new(&env),
        });
    if !user_courses.courses.contains(&course_id) {
        user_courses.courses.push_back(course_id.clone());
        env.storage()
            .persistent()
            .set(&user_courses_key, &user_courses);
        env.storage()
            .persistent()
            .extend_ttl(&user_courses_key, TTL_BUMP, TTL_TTL);
    }

    // Update CourseUsers
    let course_users_key = DataKey::CourseUsers(course_id.clone());
    let mut course_users: CourseUsers = env
        .storage()
        .persistent()
        .get(&course_users_key)
        .unwrap_or(CourseUsers {
            course: course_id.clone(),
            users: Vec::new(&env),
        });
    if !course_users.users.contains(&user) {
        course_users.users.push_back(user.clone());
        env.storage()
            .persistent()
            .set(&course_users_key, &course_users);
        env.storage()
            .persistent()
            .extend_ttl(&course_users_key, TTL_BUMP, TTL_TTL);
    }
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::{CourseUsers, DataKey, UserCourses};
use soroban_sdk::{Address, Env, String};
use crate::functions::config::{TTL_BUMP, TTL_TTL};

/// Revoke access for a specific user from a course.
///
/// This function removes a user's access to a specific course and updates
/// both the user's course list and the course's user list to maintain
/// data consistency across all storage structures.
///
/// # Arguments
///
/// * `env` - The Soroban environment
/// * `course_id` - The unique identifier of the course to revoke access from
/// * `user` - The address of the user to revoke access from
///
/// # Returns
///
/// Returns `true` if access was successfully revoked, `false` if the user
/// didn't have access to the course in the first place.
pub fn CourseAccessRevokeAccess(env: Env, course_id: String, user: Address) -> bool {
    // Input validation
    if course_id.is_empty() {
        return false;
    }

    let key: DataKey = DataKey::CourseAccess(course_id.clone(), user.clone());

    // Check if the CourseAccess entry exists in persistent storage
    if env.storage().persistent().has(&key) {
        // Remove the CourseAccess entry
        env.storage().persistent().remove(&key);

        // Update UserCourses
        let user_courses_key = DataKey::UserCourses(user.clone());
        if let Some(mut user_courses) = env
            .storage()
            .persistent()
            .get::<DataKey, UserCourses>(&user_courses_key)
        {
            if let Some(index) = user_courses.courses.iter().position(|c| c == course_id) {
                user_courses.courses.remove(index as u32);
                env.storage()
                    .persistent()
                    .set(&user_courses_key, &user_courses);
                env.storage()
                    .persistent()
                    .extend_ttl(&user_courses_key, TTL_BUMP, TTL_TTL);
            }
        }

        // Update CourseUsers
        let course_users_key = DataKey::CourseUsers(course_id.clone());
        if let Some(mut course_users) = env
            .storage()
            .persistent()
            .get::<DataKey, CourseUsers>(&course_users_key)
        {
            if let Some(index) = course_users.users.iter().position(|u| u == user) {
                course_users.users.remove(index as u32);
                env.storage()
                    .persistent()
                    .set(&course_users_key, &course_users);
                env.storage()
                    .persistent()
                    .extend_ttl(&course_users_key, TTL_BUMP, TTL_TTL);
            }
        }

        true
    } else {
        false
    }
}

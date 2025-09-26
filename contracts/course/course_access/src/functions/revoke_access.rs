// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Address, Env, String, Symbol, symbol_short};

use crate::schema::{DataKey, UserCourses, CourseUsers};
use crate::error::{Error, handle_error};

const COURSE_ACCESS_REVOKE_EVENT: Symbol = symbol_short!("crsAcRvk");

/// Revoke access for a specific user from a course
pub fn course_access_revoke_access(env: Env, course_id: String, user: Address) -> bool {
    // Validate input parameters
    if course_id.is_empty() {
        handle_error(&env, Error::EmptyCourseId);
    }
    
    // Check course_id length to prevent extremely long IDs
    if course_id.len() > 100 {
        handle_error(&env, Error::InvalidCourseId);
    }
    
    let key: DataKey = DataKey::CourseAccess(course_id.clone(), user.clone());

    // Check if the CourseAccess entry exists in persistent storage
    if env.storage().persistent().has(&key) {
        // Remove the CourseAccess entry
        env.storage().persistent().remove(&key);

        // Update UserCourses
        let user_courses_key: DataKey = DataKey::UserCourses(user.clone());
        if let Some(mut user_courses) = env.storage().persistent().get::<DataKey, UserCourses>(&user_courses_key) {
            if let Some(index) = user_courses.courses.iter().position(|c| c == course_id) {
                user_courses.courses.remove(index as u32);
                env.storage().persistent().set(&user_courses_key, &user_courses);
                env.storage().persistent().extend_ttl(&user_courses_key, 100, 1000);
            }
        }

        // Update CourseUsers
        let course_users_key: DataKey = DataKey::CourseUsers(course_id.clone());
        if let Some(mut course_users) = env.storage().persistent().get::<DataKey, CourseUsers>(&course_users_key) {
            if let Some(index) = course_users.users.iter().position(|u| u == user) {
                course_users.users.remove(index as u32);
                env.storage().persistent().set(&course_users_key, &course_users);
                env.storage().persistent().extend_ttl(&course_users_key, 100, 1000);
            }
        }
    env.events()
        .publish((COURSE_ACCESS_REVOKE_EVENT,), (course_id, user));

        true
    } else {
        false
    }
}

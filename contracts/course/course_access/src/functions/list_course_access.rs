// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Env, String, Vec, Symbol, symbol_short};

use crate::schema::{CourseUsers, DataKey};
use crate::error::{Error, handle_error};

const COURSE_ACCESS_LIST_EVENT: Symbol = symbol_short!("crs_lst_a");

pub fn course_access_list_course_access(env: Env, course_id: String) -> CourseUsers {
    // Validate input parameters
    if course_id.is_empty() {
        handle_error(&env, Error::EmptyCourseId);
    }
    
    // Check course_id length to prevent extremely long IDs
    if course_id.len() > 100 {
        handle_error(&env, Error::InvalidCourseId);
    }
    
    let key: DataKey = DataKey::CourseUsers(course_id.clone());

    let res: CourseUsers = env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(CourseUsers {
            course: course_id.clone(),
            users: Vec::new(&env),
        });
    env.events()
        .publish((COURSE_ACCESS_LIST_EVENT,), course_id);

    return res
}

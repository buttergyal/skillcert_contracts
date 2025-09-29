// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{symbol_short, Address, Env, IntoVal, String, Symbol, Vec};

use crate::error::{handle_error, Error};
use crate::schema::{DataKey, KEY_COURSE_REG_ADDR, KEY_USER_MGMT_ADDR};

/// Storage key symbol for user data
const USER_KEY: Symbol = symbol_short!("user");

/// Storage key symbol for courses data
const COURSES_KEY: Symbol = symbol_short!("courses");

/// Event symbol for revoke all access operations
const REVOKE_ALL_EVENT: Symbol = symbol_short!("revokeAll");

/// Revoke access for all users from a specific course.
///
/// This function removes access for all users from the specified course and
/// updates all related storage structures. Only admin users or course creators
/// are authorized to perform this operation.
///
/// # Arguments
///
/// * `env` - The Soroban environment
/// * `caller` - The address of the user requesting the operation
/// * `course_id` - The unique identifier of the course to revoke all access from
///
/// # Returns
///
/// Returns the number of users whose access was revoked.
///
/// # Authorization
///
/// The caller must be either:
/// - An admin user (verified via user management contract)
/// - The creator of the course (verified via course registry contract)
///
/// # Events
///
/// Emits a `revokeall` event with the course ID and number of affected users.
///
/// # Panics
///
/// Panics with `Error::Unauthorized` if the caller is not authorized to perform this operation.
pub fn revoke_all_access(env: Env, caller: Address, course_id: String) -> u32 {
    caller.require_auth();

    // Validate input parameters
    if course_id.is_empty() {
        handle_error(&env, Error::EmptyCourseId);
    }

    // Check course_id length to prevent extremely long IDs
    if course_id.len() > 100 {
        handle_error(&env, Error::InvalidCourseId);
    }

    // Resolve admin via cross-contract if configured
    let user_mgmt_addr: Address = env
        .storage()
        .instance()
        .get(&(KEY_USER_MGMT_ADDR,))
        .expect("user_mgmt_addr not configured; call initialize/set_config");
    let is_admin: bool = env.invoke_contract(
        &user_mgmt_addr,
        &Symbol::new(&env, "is_admin"),
        (caller.clone(),).into_val(&env),
    );

    // Resolve creator via cross-contract if configured
    let course_registry_addr: Address = env
        .storage()
        .instance()
        .get(&(KEY_COURSE_REG_ADDR,))
        .expect("course_registry_addr not configured; call initialize/set_config");
    let is_creator: bool = env.invoke_contract(
        &course_registry_addr,
        &Symbol::new(&env, "is_course_creator"),
        (course_id.clone(), caller.clone()).into_val(&env),
    );

    // Authorization: only admin or course creator
    if !(is_admin || is_creator) {
        handle_error(&env, Error::Unauthorized)
    }

    // Fetch all users with access to this course
    let course_key: (Symbol, String) = (COURSES_KEY, course_id.clone());
    let affected_users: Vec<Address> = match env.storage().persistent().get(&course_key) {
        Some(course_users) => {
            let cu: crate::schema::CourseUsers = course_users;
            cu.users
        }
        None => Vec::new(&env),
    };

    let count: u32 = affected_users.len();
    if count == 0 {
        env.events()
            .publish((REVOKE_ALL_EVENT, course_id.clone()), count);
        return 0;
    }

    // Remove each user's access entry and update per-user course index if present
    let mut i: u32 = 0u32;
    while i < count {
        if let Some(user) = affected_users.get(i) {
            let access_key: DataKey = DataKey::CourseAccess(course_id.clone(), user.clone());
            if env.storage().persistent().has(&access_key) {
                env.storage().persistent().remove(&access_key);
            }

            let user_key: (Symbol, String) = (USER_KEY, user.to_string());
            if let Some(mut uc) = env
                .storage()
                .persistent()
                .get::<_, crate::schema::UserCourses>(&user_key)
            {
                let mut new_courses: Vec<String> = Vec::new(&env);
                let mut j: u32 = 0u32;
                let total: u32 = uc.courses.len();
                while j < total {
                    if let Some(cid) = uc.courses.get(j) {
                        if cid != course_id {
                            new_courses.push_back(cid);
                        }
                    }
                    j = j.saturating_add(1);
                }
                uc.courses = new_courses;
                env.storage().persistent().set(&user_key, &uc);
            }
        }
        i = i.saturating_add(1);
    }

    // Clear course -> users index
    let empty: Vec<Address> = Vec::new(&env);
    if env.storage().persistent().has(&course_key) {
        let mut cu: crate::schema::CourseUsers =
            env.storage().persistent().get(&course_key).unwrap();
        cu.users = empty;
        env.storage().persistent().set(&course_key, &cu);
    }

    env.events()
        .publish((REVOKE_ALL_EVENT, course_id.clone()), count);

    count
}

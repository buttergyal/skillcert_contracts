// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::functions::rbac::{has_permission, is_admin};
use crate::schema::Permission;
use soroban_sdk::{symbol_short, Address, Env, Symbol, Vec};

const ACCESS_LISTED_EVENT: Symbol = symbol_short!("acListed");
/// Helper function to check if the given address is the creator of the course.
/// Adjust the storage key or logic to match your contract.
fn is_creator(env: &Env, course_id: u128, who: &Address) -> bool {
    // Retrieve the creator address for the course from storage
    let creator: Address = env
        .storage()
        .persistent()
        .get(&(("course_creator", course_id),))
        .expect("Course not found");
    creator == *who
}

/// Check if user has permission to list course access
/// Uses RBAC system to determine access rights
fn can_list_course_access(env: &Env, caller: &Address, course_id: u128) -> bool {
    // Course creator always has access
    if is_creator(env, course_id, caller) {
        return true;
    }

    // Check RBAC permissions
    if has_permission(env, caller, &Permission::ManageCourseAccess) {
        return true;
    }

    // Check if user has ViewUsers permission (for admin-level access)
    if has_permission(env, caller, &Permission::ViewUsers) {
        return true;
    }

    // Fallback to legacy admin check for backward compatibility
    is_admin(env, caller)
}

/// List all users who currently have access to a course.
///
/// Now uses RBAC system for granular permission control:
/// - Course creators can always list access for their courses
/// - Users with ManageCourseAccess permission can list access
/// - Users with ViewUsers permission can list access (admin-level)
/// - Legacy admin check as fallback for backward compatibility
///
/// # Arguments
/// * env - Soroban environment.
/// * caller - Address of the caller.
/// * course_id - Identifier of the course.
///
/// # Returns
/// * Vec<Address> - List of addresses with access to the course.
pub fn list_users_with_access(env: Env, caller: Address, course_id: u128) -> Vec<Address> {
    // Require the caller to be authenticated
    caller.require_auth();

    // RBAC: Check if caller has permission to list course access
    if !can_list_course_access(&env, &caller, course_id) {
        handle_error(&env, Error::AccessDenied);
    }

    // Retrieve the list of users with access from storage
    let access_list: Vec<Address> = env
        .storage()
        .persistent()
        .get(&(("course_access", course_id),))
        .unwrap_or_else(|| Vec::new(&env));

    // Optional: Emit an event with the number of users
    env.events().publish(
        (ACCESS_LISTED_EVENT,),
        (course_id, caller.clone(), access_list.len() as u32),
    );

    access_list
}
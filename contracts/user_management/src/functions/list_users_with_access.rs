// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::functions::rbac::{has_permission, is_admin};
use crate::schema::Permission;
use soroban_sdk::{symbol_short, Address, Env, Symbol, Vec};

const ACCESS_LISTED_EVENT: Symbol = symbol_short!("acListed");

fn is_creator(env: &Env, course_id: u128, who: &Address) -> bool {
    // Retrieve the creator address for the course from storage
    let creator: Address = env
        .storage()
        .persistent()
        .get(&(("course_creator", course_id),))
        .expect("Course not found");
    creator == *who
}

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
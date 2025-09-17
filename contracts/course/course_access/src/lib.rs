// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

/// Contract version for tracking deployments and upgrades
pub const VERSION: &str = "1.0.0";

mod error;
mod functions;
mod schema;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

pub use error::CourseAccessError;
pub use functions::*;
pub use schema::{CourseUsers, UserCourses};

/// Course Access Contract
///
/// This contract manages user access to courses in the SkillCert platform.
/// It provides functionality to grant, revoke, and query course access permissions,
/// as well as manage user profiles.
#[contract]
pub struct CourseAccessContract;

#[contractimpl]
impl CourseAccessContract {
    /// One-time constructor to set owner and config addresses.
    ///
    /// Initializes the contract with the necessary external contract addresses.
    /// This function can only be called once during contract deployment.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `caller` - The address of the contract deployer/owner
    /// * `user_mgmt_addr` - Address of the user management contract
    /// * `course_registry_addr` - Address of the course registry contract
    ///
    /// # Panics
    ///
    /// Fails if the contract has already been initialized.
    pub fn Initialize(
        env: Env,
        caller: Address,
        user_mgmt_addr: Address,
        course_registry_addr: Address,
    ) {
        functions::config::Initialize(env, caller, user_mgmt_addr, course_registry_addr)
    }

    /// Grant access to a specific user for a given course.
    ///
    /// Allows a user to access a specific course. Only authorized users
    /// (course creators or admins) can grant access.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `course_id` - The unique identifier of the course
    /// * `user` - The address of the user to grant access to
    pub fn GrantAccess(env: Env, course_id: String, user: Address) {
        functions::grant_access::CourseAccessGrantAccess(env, course_id, user)
    }

    /// Revoke access for a specific user from a course.
    ///
    /// Removes a user's access to a specific course. Only authorized users
    /// (course creators or admins) can revoke access.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `course_id` - The unique identifier of the course
    /// * `user` - The address of the user to revoke access from
    ///
    /// # Returns
    ///
    /// Returns `true` if access was successfully revoked, `false` otherwise.
    pub fn RevokeAccess(env: Env, course_id: String, user: Address) -> bool {
        functions::revoke_access::CourseAccessRevokeAccess(env, course_id, user)
    }

    /// Save or update a user's profile on-chain.
    ///
    /// Stores user profile information in the contract storage.
    /// This includes personal and professional information.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `name` - The user's full name
    /// * `email` - The user's email address
    /// * `profession` - Optional profession/job title
    /// * `goals` - Optional learning goals or objectives
    /// * `country` - The user's country of residence
    pub fn SaveUserProfile(
        env: Env,
        name: String,
        email: String,
        profession: Option<String>,
        goals: Option<String>,
        country: String,
    ) {
        let user = env.current_contract_address();
        SaveUserProfile(env, name, email, profession, goals, country, user);
    }

    /// List all courses a user has access to.
    ///
    /// Retrieves all courses that the specified user is enrolled in
    /// or has been granted access to.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `user` - The address of the user to query
    ///
    /// # Returns
    ///
    /// Returns a `UserCourses` struct containing the list of accessible courses.
    pub fn ListUserCourses(env: Env, user: Address) -> UserCourses {
        functions::list_user_courses::ListUserCourses(env, user)
    }

    /// List all users who have access to a course.
    ///
    /// Retrieves all users who have been granted access to the specified course.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `course_id` - The unique identifier of the course
    ///
    /// # Returns
    ///
    /// Returns a `CourseUsers` struct containing the list of users with access.
    pub fn ListCourseAccess(env: Env, course_id: String) -> CourseUsers {
        functions::list_course_access::ListCourseAccess(env, course_id)
    }

    /// Revoke all user access for a course.
    ///
    /// Removes access for all users from the specified course.
    /// Only admin or course creator is allowed to perform this operation.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `user` - The address of the user requesting the operation
    /// * `course_id` - The unique identifier of the course
    ///
    /// # Returns
    ///
    /// Returns the number of users affected by the revocation and emits an event.
    pub fn RevokeAllAccess(env: Env, user: Address, course_id: String) -> u32 {
        functions::revoke_all_access::RevokeAllAccess(env, user, course_id)
    }

    /// Configure external contract addresses used for auth checks.
    ///
    /// Updates the addresses of external contracts that this contract
    /// depends on for authentication and authorization checks.
    ///
    /// # Arguments
    ///
    /// * `env` - The Soroban environment
    /// * `caller` - The address of the user making the configuration change
    /// * `user_mgmt_addr` - Address of the user management contract
    /// * `course_registry_addr` - Address of the course registry contract
    ///
    /// # Storage
    ///
    /// Stores the addresses in instance storage keys: ("user_mgmt_addr",) and ("course_registry_addr",)
    pub fn SetConfig(
        env: Env,
        caller: Address,
        user_mgmt_addr: Address,
        course_registry_addr: Address,
    ) {
        functions::config::SetContractAddrs(env, caller, user_mgmt_addr, course_registry_addr)
    }
}

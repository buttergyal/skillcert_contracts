// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracttype, Address, String, Vec};

/// Represents access permission for a user to a specific course.
///
/// This struct defines the relationship between a user and a course
/// they have been granted access to.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CourseAccess {
    /// The unique identifier of the course
    pub course_id: String,
    /// The address of the user who has access
    pub user: Address,
}

/// Contains all courses that a specific user has access to.
///
/// This struct is used to efficiently query and return all courses
/// accessible by a particular user.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct UserCourses {
    /// The address of the user
    pub user: Address,
    /// List of course IDs the user has access to
    pub courses: Vec<String>,
}

/// Storage keys for different data types in the contract.
///
/// This enum defines the various keys used to store and retrieve
/// data from the contract's persistent storage.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    /// Key for storing course access: (course_id, user) -> CourseAccess
    CourseAccess(String, Address),
    /// Key for storing user profile: user -> UserProfile
    UserProfile(Address),
    /// Key for storing courses per user: user -> UserCourses
    UserCourses(Address),
    /// Key for storing users per course: course_id -> CourseUsers
    CourseUsers(String),
}

/// Represents a user's profile information.
///
/// This struct contains all the personal and professional information
/// that users can store on-chain as part of their profile.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct UserProfile {
    /// The user's full name
    pub name: String,
    /// The user's email address
    pub email: String,
    /// Optional profession or job title
    pub profession: Option<String>,
    /// Optional learning goals or objectives
    pub goals: Option<String>,
    /// The user's country of residence
    pub country: String,
}

/// Contains all users who have access to a specific course.
///
/// This struct is used to efficiently query and return all users
/// who have been granted access to a particular course.
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CourseUsers {
    /// The unique identifier of the course
    pub course: String,
    /// List of user addresses who have access to the course
    pub users: Vec<Address>,
}

/// Global configuration key for storing the user management contract address
pub const KEY_USER_MGMT_ADDR: &str = "USER_MGMT_ADDR";

/// Global configuration key for storing the course registry contract address
pub const KEY_COURSE_REG_ADDR: &str = "COURSE_REGISTRY_ADDR";

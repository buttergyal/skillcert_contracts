// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracttype, Address, String, Vec};

/// Default and limit constants for user management configuration
pub const DEFAULT_MAX_PAGE_SIZE: u32 = 100;
pub const ABSOLUTE_MAX_PAGE_SIZE: u32 = 1000;
pub const MAX_ADMINS: usize = 10;

/// Complete user profile information.
///
/// This struct contains all user data including personal information,
/// credentials, preferences, and platform-specific details.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserProfile {
    /// User's first name
    pub name: String,
    /// User's last name
    pub lastname: String,
    /// User's email address (must be unique)
    pub email: String,
    /// User's role in the platform
    pub role: UserRole,
    /// User's country of residence
    pub country: String,
    /// Optional profession or job title
    pub profession: Option<String>,
    /// Optional learning goals or objectives
    pub goals: Option<String>,
    /// Optional URL to profile picture
    pub profile_picture: Option<String>,
    /// User's preferred language
    pub language: String,
    /// User's password (hashed)
    pub password: String,
    /// Password confirmation (for validation)
    pub confirm_password: String,
    /// User's area of specialization
    pub specialization: String,
    /// Languages the user speaks
    pub languages: Vec<String>,
    /// Categories the user can teach (for instructors)
    pub teaching_categories: Vec<String>,
    /// User's blockchain address
    pub user: Address,
}

/// User roles in the SkillCert platform.
///
/// Defines the different types of users and their permission levels.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum UserRole {
    /// Regular platform user who can enroll in courses
    Student,
    /// User who can create and manage courses
    Instructor,
    /// Platform administrator with elevated privileges
    Admin,
}

/// User account status.
///
/// Represents the current state of a user's account.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum UserStatus {
    /// User account is active and functional
    Active,
    /// User account is deactivated
    Inactive,
    /// User account is temporarily suspended
    Suspended,
}

/// Filtering criteria for user queries.
///
/// Used to filter users based on various attributes when listing or searching.
#[derive(Clone, Debug, PartialEq)]
pub struct UserFilter {
    /// Filter by user role
    pub role: Option<UserRole>,
    /// Filter by country
    pub country: Option<String>,
    /// Filter by account status
    pub status: Option<UserStatus>,
}

/// Lightweight user profile for listing operations.
///
/// Contains essential user information without sensitive data like passwords.
/// Used for efficient querying and display in user lists.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct LightProfile {
    /// User's first name
    pub name: String,
    /// User's last name
    pub lastname: String,
    /// User's area of specialization
    pub specialization: String,
    /// Languages the user speaks
    pub languages: Vec<String>,
    /// Categories the user can teach
    pub teaching_categories: Vec<String>,
    /// User's role in the platform
    pub role: UserRole,
    /// User's account status
    pub status: UserStatus,
    /// User's blockchain address
    pub user_address: Address,
}

/// Administrative configuration for the user management system.
///
/// Contains system-wide settings and administrative information.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct AdminConfig {
    /// Whether the system has been initialized
    pub initialized: bool,
    /// Address of the super administrator
    pub super_admin: Address,
    /// Maximum allowed page size for queries
    pub max_page_size: u32,
    /// Total number of registered users
    pub total_user_count: u32,
}

/// Storage keys for different data types in the user management contract.
///
/// This enum defines the various keys used to store and retrieve
/// user data from the contract's persistent storage.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    /// Key for storing complete user profiles: user_address -> UserProfile
    UserProfile(Address),
    /// Key for storing admin flags: address -> bool
    Admin(Address),
    /// Key for storing lightweight user profiles: user_address -> LightProfile
    UserProfileLight(Address),
    /// Key for storing the list of all registered user addresses
    UsersIndex,
    /// Key for email to address mapping to ensure email uniqueness: email -> Address
    EmailIndex(String),
    /// Key for storing the list of admin addresses
    Admins,
    /// Key for storing user role assignments
    UserRoles,
    /// Key for storing administrative configuration
    AdminConfig,
}

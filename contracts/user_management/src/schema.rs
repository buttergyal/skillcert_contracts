// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracttype, Address, String, Vec};

/// Default and limit constants for user management configuration
pub const DEFAULT_MAX_PAGE_SIZE: u32 = 100;
pub const ABSOLUTE_MAX_PAGE_SIZE: u32 = 1000;
pub const MAX_ADMINS: u32 = 10;

/// Rate limiting constants
pub const DEFAULT_RATE_LIMIT_WINDOW: u64 = 3600; // 1 hour in seconds
pub const DEFAULT_MAX_USER_CREATIONS_PER_WINDOW: u32 = 5; // Max user creations per hour per address

/// Password validation constants
pub const MIN_PASSWORD_LENGTH: u32 = 8;
pub const MAX_PASSWORD_LENGTH: u32 = 128;
pub const REQUIRED_SPECIAL_CHARS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";
pub const REQUIRED_DIGITS: &str = "0123456789";
pub const REQUIRED_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const REQUIRED_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

/// User profile information matching UI definition.
///
/// This struct contains user profile data with required and optional fields
/// as defined by the user interface requirements.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserProfile {
    /// User's full name (required)
    pub full_name: String,
    /// User's contact email address (required, must be unique)
    pub contact_email: String,
    /// User's profession or job title (optional)
    pub profession: Option<String>,
    /// User's country of residence (optional)
    pub country: Option<String>,
    /// User's learning goals or purpose (optional)
    pub purpose: Option<String>,
    /// User's profile picture URL (optional)
    pub profile_picture_url: Option<String>,
}

/// Struct for profile update parameters
/// Only includes fields that can be updated
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileUpdateParams {
    /// User's full name (optional update)
    pub full_name: Option<String>,
    /// User's profession or job title
    pub profession: Option<String>,
    /// User's country of residence
    pub country: Option<String>,
    /// User's learning goals or purpose
    pub purpose: Option<String>,
    /// User's profile picture URL
    pub profile_picture_url: Option<String>,
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
/// Contains essential user information for efficient querying and display in user lists.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct LightProfile {
    /// User's full name
    pub full_name: String,
    /// User's profession or job title
    pub profession: Option<String>,
    /// User's country of residence
    pub country: Option<String>,
    /// User's role in the platform
    pub role: UserRole,
    /// User's account status
    pub status: UserStatus,
    /// User's blockchain address
    pub user_address: Address,
}

/// Rate limiting configuration for user operations.
///
/// Tracks rate limiting settings and current usage for spam protection.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct RateLimitConfig {
    /// Time window for rate limiting in seconds
    pub window_seconds: u64,
    /// Maximum operations allowed per window
    pub max_operations_per_window: u32,
}

/// Rate limiting tracking data for a specific address.
///
/// Stores the current usage count and window start time for rate limiting.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct RateLimitData {
    /// Current count of operations in this window
    pub count: u32,
    /// Timestamp when the current window started
    pub window_start: u64,
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
    /// Rate limiting configuration for user creation
    pub rate_limit_config: RateLimitConfig,
}

/// Pagination parameters for cursor-based pagination.
///
/// Used to implement efficient pagination that avoids gas limit issues
/// with large datasets by using cursor-based navigation.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PaginationParams {
    /// Cursor for pagination (address of the last item from previous page)
    pub cursor: Option<Address>,
    /// Maximum number of items to return per page
    pub limit: u32,
}

/// Pagination result with metadata for efficient navigation.
///
/// Contains the paginated data along with pagination metadata
/// to enable cursor-based navigation.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PaginatedLightProfiles {
    /// The paginated data items
    pub data: Vec<LightProfile>,
    /// Cursor for the next page (None if this is the last page)
    pub next_cursor: Option<Address>,
    /// Total count of items matching the filter (optional, may be expensive to compute)
    pub total_count: Option<u32>,
    /// Whether there are more pages available
    pub has_more: bool,
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
    /// Key for storing rate limiting data per address: address -> RateLimitData
    RateLimit(Address),
}

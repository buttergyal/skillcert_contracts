// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracttype, Address, String, Vec};

/// Course registry defaults and limits
pub const DEFAULT_COURSE_PRICE: u128 = 1000;
pub const MAX_LOOP_GUARD: u32 = 1000;
pub const FILTER_MIN_PRICE: u128 = 500;
pub const MAX_SCAN_ID: u32 = 50;
pub const MAX_EMPTY_CHECKS: u32 = 10;

/// Rate limiting constants for course operations
pub const DEFAULT_COURSE_RATE_LIMIT_WINDOW: u64 = 3600; // 1 hour in seconds
pub const DEFAULT_MAX_COURSE_CREATIONS_PER_WINDOW: u32 = 3; // Max course creations per hour per address

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseModule {
    pub id: String,
    pub course_id: String,
    pub position: u32,
    pub title: String,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseGoal {
    pub goal_id: String,
    pub course_id: String,
    pub content: String,
    pub created_by: Address,
    pub created_at: u64,
}

/// Rate limiting configuration for course operations.
///
/// Tracks rate limiting settings for spam protection in course creation.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseRateLimitConfig {
    /// Time window for rate limiting in seconds
    pub window_seconds: u64,
    /// Maximum course creations allowed per window
    pub max_courses_per_window: u32,
}

/// Rate limiting tracking data for course operations per address.
///
/// Stores the current usage count and window start time for course rate limiting.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseRateLimitData {
    /// Current count of course creations in this window
    pub count: u32,
    /// Timestamp when the current window started
    pub window_start: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseCategory {
    pub id: u128,
    pub name: String,
    pub description: Option<String>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    Module(String),
    Courses,
    CourseGoalList(String),     // Optional: Keep a list of goal IDs per course
    CourseGoal(String, String), // (course_id, goal_id)
    CoursePrerequisites(String),
    CategorySeq,          // Sequence counter for category IDs
    CourseCategory(u128), // Course category by ID
    Admins,               // List of admin addresses
    /// Key for storing course rate limiting configuration
    CourseRateLimitConfig,
    /// Key for storing course rate limiting data per address: address -> CourseRateLimitData
    CourseRateLimit(Address),
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator: Address,
    pub price: u128,
    pub category: Option<String>,
    pub language: Option<String>,
    pub thumbnail_url: Option<String>,
    pub published: bool,
    pub prerequisites: Vec<CourseId>,
    pub is_archived: bool,
    pub level: Option<CourseLevel>,
    pub duration_hours: Option<u32>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseId {
    pub id: String,
    pub count: u128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Category {
    pub name: String,
    pub count: u128,
}

// Course level as string to avoid Soroban enum serialization issues
// Valid values: "Beginner", "Intermediate", "Advanced"
pub type CourseLevel = String;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseFilters {
    pub min_price: Option<u128>,
    pub max_price: Option<u128>,
    pub category: Option<String>,
    pub level: Option<CourseLevel>,
    pub min_duration: Option<u32>,
    pub max_duration: Option<u32>,
    /// Text search in course title and description
    pub search_text: Option<String>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EditCourseParams {
    pub new_title: Option<String>,
    pub new_description: Option<String>,
    pub new_price: Option<u128>,
    pub new_category: Option<Option<String>>,
    pub new_language: Option<Option<String>>,
    pub new_thumbnail_url: Option<Option<String>>,
    pub new_published: Option<bool>,
    pub new_level: Option<Option<CourseLevel>>,
    pub new_duration_hours: Option<Option<u32>>,
}

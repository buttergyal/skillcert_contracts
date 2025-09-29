// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{DataKey, CourseRateLimitData, CourseRateLimitConfig, DEFAULT_COURSE_RATE_LIMIT_WINDOW, DEFAULT_MAX_COURSE_CREATIONS_PER_WINDOW};
use soroban_sdk::{Address, Env};

/// Check if the user has exceeded the rate limit for course creation operations.
///
/// This function validates if the caller can perform a course creation operation
/// based on the configured rate limiting rules.
///
/// # Arguments
/// * `env` - The Soroban environment
/// * `creator` - The address attempting to create a course
///
/// # Panics
/// * If rate limit is exceeded
/// * If rate limit configuration is not found
pub fn check_course_creation_rate_limit(env: &Env, creator: &Address) {
    // Get rate limit configuration
    let config_key = DataKey::CourseRateLimitConfig;
    let rate_config = match env
        .storage()
        .persistent()
        .get::<DataKey, CourseRateLimitConfig>(&config_key)
    {
        Some(config) => config,
        None => {
            // If no configuration exists, use default
            get_default_course_rate_limit_config()
        }
    };

    let current_time = env.ledger().timestamp();
    let rate_limit_key = DataKey::CourseRateLimit(creator.clone());
    
    // Get existing rate limit data or create new one
    let mut rate_data = match env
        .storage()
        .persistent()
        .get::<DataKey, CourseRateLimitData>(&rate_limit_key)
    {
        Some(data) => data,
        None => CourseRateLimitData {
            count: 0,
            window_start: current_time,
        }
    };

    // Check if we need to reset the window
    if current_time >= rate_data.window_start + rate_config.window_seconds {
        // Reset the window
        rate_data.count = 0;
        rate_data.window_start = current_time;
    }

    // Check if user has exceeded the rate limit
    if rate_data.count >= rate_config.max_courses_per_window {
        handle_error(env, Error::CourseRateLimitExceeded);
    }

    // Increment the count and save
    rate_data.count += 1;
    env.storage()
        .persistent()
        .set(&rate_limit_key, &rate_data);
}

/// Get the default rate limiting configuration for course operations.
///
/// This function returns the default rate limiting settings that can be
/// used when initializing the system or when no custom configuration is set.
pub fn get_default_course_rate_limit_config() -> CourseRateLimitConfig {
    CourseRateLimitConfig {
        window_seconds: DEFAULT_COURSE_RATE_LIMIT_WINDOW,
        max_courses_per_window: DEFAULT_MAX_COURSE_CREATIONS_PER_WINDOW,
    }
}

/// Initialize the default rate limiting configuration for course operations.
///
/// This function should be called during system initialization to set up
/// the default rate limiting configuration.
///
/// # Arguments
/// * `env` - The Soroban environment
pub fn initialize_course_rate_limit_config(env: &Env) {
    let config_key = DataKey::CourseRateLimitConfig;
    
    // Only initialize if not already set
    if !env.storage().persistent().has(&config_key) {
        let default_config = get_default_course_rate_limit_config();
        env.storage()
            .persistent()
            .set(&config_key, &default_config);
    }
}

/// Update the course rate limiting configuration.
///
/// This function allows administrators to modify the rate limiting settings.
///
/// # Arguments
/// * `env` - The Soroban environment
/// * `new_config` - The new rate limiting configuration
pub fn update_course_rate_limit_config(env: &Env, new_config: CourseRateLimitConfig) {
    let config_key = DataKey::CourseRateLimitConfig;
    env.storage()
        .persistent()
        .set(&config_key, &new_config);
}


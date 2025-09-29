// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{DataKey, RateLimitData, RateLimitConfig, DEFAULT_RATE_LIMIT_WINDOW, DEFAULT_MAX_USER_CREATIONS_PER_WINDOW};
use soroban_sdk::{Address, Env};

/// Check if the user has exceeded the rate limit for user creation operations.
///
/// This function validates if the caller can perform a user creation operation
/// based on the configured rate limiting rules.
///
/// # Arguments
/// * `env` - The Soroban environment
/// * `user` - The address attempting to create a user
/// * `rate_config` - The rate limiting configuration to use
///
/// # Panics
/// * If rate limit is exceeded
pub fn check_user_creation_rate_limit(env: &Env, user: &Address, rate_config: &RateLimitConfig) {
    let current_time = env.ledger().timestamp();
    let rate_limit_key = DataKey::RateLimit(user.clone());
    
    // Get existing rate limit data or create new one
    let mut rate_data = match env
        .storage()
        .persistent()
        .get::<DataKey, RateLimitData>(&rate_limit_key)
    {
        Some(data) => data,
        None => RateLimitData {
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
    if rate_data.count >= rate_config.max_operations_per_window {
        handle_error(env, Error::RateLimitExceeded);
    }

    // Increment the count and save
    rate_data.count += 1;
    env.storage()
        .persistent()
        .set(&rate_limit_key, &rate_data);
}

/// Get the default rate limiting configuration for user operations.
///
/// This function returns the default rate limiting settings that can be
/// used when initializing the system or when no custom configuration is set.
pub fn get_default_rate_limit_config() -> RateLimitConfig {
    RateLimitConfig {
        window_seconds: DEFAULT_RATE_LIMIT_WINDOW,
        max_operations_per_window: DEFAULT_MAX_USER_CREATIONS_PER_WINDOW,
    }
}

/// Initialize rate limiting configuration in the admin config.
///
/// This function should be called during system initialization to set up
/// the default rate limiting configuration.
///
/// # Arguments
/// * `_env` - The Soroban environment (unused but kept for interface consistency)
pub fn initialize_rate_limit_config(_env: &Env) -> RateLimitConfig {
    get_default_rate_limit_config()
}


// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{Course, CourseModule};
use soroban_sdk::{symbol_short, Address, Env, String, Vec};

/// Efficiently validates course existence and optionally checks creator authorization
/// Returns the course if it exists, reducing redundant storage reads
pub fn validate_course(
    env: &Env,
    course_id: &String,
    require_creator: Option<&Address>,
) -> Course {
    // Check temporary storage first
    let temp_key = (symbol_short!("temp_course"), course_id.clone());
    if let Some(course) = env.storage().temporary().get(&temp_key) {
        // Still verify creator if required
        if let Some(creator) = require_creator {
            if &course.creator != creator {
                handle_error(env, Error::OnlyCreatorCanEdit);
            }
        }
        return course;
    }

    // Get from persistent storage
    let course_key = (symbol_short!("course"), course_id.clone());
    let course: Course = env
        .storage()
        .persistent()
        .get(&course_key)
        .unwrap_or_else(|| handle_error(env, Error::CourseIdNotExist));

    if let Some(creator) = require_creator {
        if &course.creator != creator {
            handle_error(env, Error::OnlyCreatorCanEdit);
        }
    }

    // Cache in temporary storage
    env.storage().temporary().set(&temp_key, &course);
    // Cache for 15 minutes
    env.storage().temporary().extend_ttl(&temp_key, 0, 900);

    course
}

/// Get course modules with efficient storage access
pub fn get_course_modules(
    env: &Env,
    course_id: &String,
) -> Vec<CourseModule> {
    let temp_key = (symbol_short!("temp_modules"), course_id.clone());
    
    // Try cache first
    if let Some(modules) = env.storage().temporary().get(&temp_key) {
        return modules;
    }

    // Get from persistent storage
    let modules: Vec<CourseModule> = env
        .storage()
        .persistent()
        .get(&(symbol_short!("course_modules"), course_id.clone()))
        .unwrap_or_else(|| Vec::new(env));

    // Cache for future use
    env.storage().temporary().set(&temp_key, &modules);
    env.storage().temporary().extend_ttl(&temp_key, 0, 900);

    modules
}

/// Update module list with proper cache invalidation
pub fn update_course_modules(
    env: &Env,
    course_id: &String,
    modules: &Vec<CourseModule>,
) {
    // Update persistent storage
    env.storage().persistent().set(
        &(symbol_short!("course_modules"), course_id.clone()),
        modules,
    );

    // Update cache
    let temp_key = (symbol_short!("temp_modules"), course_id.clone());
    env.storage().temporary().set(&temp_key, modules);
    env.storage().temporary().extend_ttl(&temp_key, 0, 900);
}

/// Remove module with cache updates
pub fn remove_module(
    env: &Env,
    module_id: &String,
    course_id: &String,
) {
    // Remove from persistent storage
    env.storage()
        .persistent()
        .remove(&(symbol_short!("module"), module_id.clone()));

    // Invalidate module cache
    let temp_key = (symbol_short!("temp_modules"), course_id.clone());
    env.storage().temporary().remove(&temp_key);
}

/// Cache invalidation helper for course updates
pub fn invalidate_course_cache(
    env: &Env,
    course_id: &String,
) {
    let temp_course_key = (symbol_short!("temp_course"), course_id.clone());
    let temp_modules_key = (symbol_short!("temp_modules"), course_id.clone());
    
    env.storage().temporary().remove(&temp_course_key);
    env.storage().temporary().remove(&temp_modules_key);
}
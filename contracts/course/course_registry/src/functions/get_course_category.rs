// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::Env;
use crate::schema::{CourseCategory, DataKey};

/// Retrieves a course category by its ID.
///
/// Arguments:
/// - env: Soroban environment.
/// - category_id: the ID of the category to retrieve.
///
/// Returns:
/// - Option<CourseCategory>: the category if found, None otherwise.
///
/// Storage used:
/// - DataKey::CourseCategory(id) -> CourseCategory
pub fn get_course_category(env: &Env, category_id: u128) -> Option<CourseCategory> {
    env.storage()
        .persistent()
        .get(&DataKey::CourseCategory(category_id))
}

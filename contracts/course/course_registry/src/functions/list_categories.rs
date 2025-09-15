// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use super::utils::u32_to_string;
use crate::schema::{Category, Course};
use soroban_sdk::{symbol_short, Env, Map, String, Symbol, Vec};

const COURSE_KEY: Symbol = symbol_short!("course");
const COURSE_ID_COUNTER: Symbol = symbol_short!("course");

/// Lists all unique course categories and counts how many courses belong to each category.
///
/// # Parameters
/// - `env`: Soroban environment reference.
///
/// # Returns
/// A `Vec<Category>` where each `Category` contains:
/// - `name`: The category name (`String`)
/// - `count`: How many courses belong to this category (`u128`)
///
/// # Notes
/// - Iterates from course ID `1` to the last generated ID (`COURSE_ID_COUNTER`).
/// - Skips deleted courses (holes in the ID sequence).
/// - Ignores courses without a category (`None`).
/// - Uses persistent storage to retrieve each course.
pub fn list_categories(env: &Env) -> Vec<Category> {
    // Temporary map to store category name -> count
    let mut categories_map: Map<String, u128> = Map::new(env);

    // Get the maximum course ID generated so far (0 if no courses exist yet)
    let max_id: u128 = env
        .storage()
        .persistent()
        .get(&COURSE_ID_COUNTER)
        .unwrap_or(0);

    // Iterate over all possible course IDs from 1 to max_id
    let mut id: u128 = 1;
    while id <= max_id {
        let course_id = u32_to_string(env, id as u32);
        let key = (COURSE_KEY, course_id);

        // Check if a course with this ID exists
        if env.storage().persistent().has(&key) {
            // Retrieve the course from storage
            if let Some(course) = env.storage().persistent().get::<_, Course>(&key) {
                // Only count courses that have a category set
                if let Some(cat) = course.category {
                    let name = cat;
                    let current = categories_map.get(name.clone()).unwrap_or(0);
                    categories_map.set(name, current + 1);
                }
            }
        }

        id += 1;
    }

    // Convert the map (name -> count) into a Vec<Category> for the final output
    let mut out = Vec::new(env);
    let keys = categories_map.keys();
    for k in keys.iter() {
        let count = categories_map.get(k.clone()).unwrap_or(0);
        out.push_back(Category { name: k, count });
    }

    out
}

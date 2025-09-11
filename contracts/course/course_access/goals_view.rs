// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

// src/course_registry/goals_view.rs

use soroban_sdk::{contracttype, Address, Env, String, Symbol, Vec};

// ---------- Schema (align with your existing schema module) ----------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Goal {
    /// The learning objective text.
    pub text: String,
    /// Optional metadata for future use (e.g., tags, difficulty).
    /// Keep it flexible so "add goal" can evolve without breaking this view.
    pub metadata: Option<String>,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    // Existing keys in your project may include Course, Title, etc.
    // We add a dedicated storage bucket for course goals:
    CourseGoals(u128),

    // (Optional) If you track course creators for auth checks elsewhere.
    CourseCreator(u128),
}

// ---------- Read-only (view) functions ----------

/// View: List goals (as plain strings) for a given course.
/// - Does not require auth.
/// - Will not mutate state.
/// - Safe to expose for frontend details pages.
pub fn list_goals_by_course(env: Env, course_id: u128) -> Vec<String> {
    // Attempt to read goals that were saved by the "add goal to course" feature.
    // Primary path: goals are stored as Vec<Goal>.
    if let Some(stored) = env.storage().get::<DataKey, Vec<Goal>>(&DataKey::CourseGoals(course_id))
    {
        return stored.iter().map(|g| g.text.clone()).collect();
    }

    // Backward-compat path: if your "add goal" issue stored Vec<String> directly.
    if let Some(stored_simple) =
        env.storage().get::<DataKey, Vec<String>>(&DataKey::CourseGoals(course_id))
    {
        return stored_simple;
    }

    // No goals found; return empty vector for a graceful UX.
    Vec::new(&env)
}

/// View (detailed): List full Goal structs if your storage includes metadata.
/// - Returns empty Vec if nothing found or your storage only has plain strings.
/// - Frontend can prefer this when metadata is desirable.
pub fn list_goals_by_course_detailed(env: Env, course_id: u128) -> Vec<Goal> {
    if let Some(stored) = env.storage().get::<DataKey, Vec<Goal>>(&DataKey::CourseGoals(course_id))
    {
        return stored;
    }

    // If goals were stored as plain strings, lift them into Goal structs on the fly.
    if let Some(stored_simple) =
        env.storage().get::<DataKey, Vec<String>>(&DataKey::CourseGoals(course_id))
    {
        let mut lifted: Vec<Goal> = Vec::new(&env);
        for text in stored_simple.iter() {
            lifted.push_back(Goal {
                text: text.clone(),
                metadata: None,
            });
        }
        return lifted;
    }

    Vec::new(&env)
}

// ---------- Optional: helper to assert course existence (no-op if you don't need it) ----------

/// If you want to fail fast when a course doesn't exist, call this before reads.
/// Keep it as a no-op until your Course registry is finalized.
pub fn assert_course_exists(_env: &Env, _course_id: u128) {
    // Example (uncomment/adapt once your Course is keyed in storage):
    // if _env.storage().get::<DataKey, Course>(&DataKey::Course(_course_id)).is_none() {
    //     panic!("Course not found");
    // }
}
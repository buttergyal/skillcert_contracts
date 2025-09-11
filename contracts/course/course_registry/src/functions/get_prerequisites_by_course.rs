// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::{Course, CourseId};
use soroban_sdk::{symbol_short, Env, String, Vec};

pub fn get_prerequisites_by_course_id(env: &Env, course_id: String) -> Vec<CourseId> {
    let key = (symbol_short!("course"), course_id);

    match env.storage().persistent().get::<_, Course>(&key) {
        Some(course) => course.prerequisites,
        None => Vec::new(&env), // Return empty if course doesn't exist
    }
}

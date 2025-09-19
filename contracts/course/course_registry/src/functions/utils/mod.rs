// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

pub mod cache;
pub mod storage_utils;

pub mod string_utils {
    use soroban_sdk::{Env, String, Vec};

    pub fn concat_strings(env: &Env, strings: Vec<String>) -> String {
        let mut result = String::new(env);
        for s in strings.iter() {
            result.push_str(&s);
        }
        result
    }

    pub fn u32_to_string(env: &Env, num: u32) -> String {
        String::from_str(env, &num.to_string())
    }
}
// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

use soroban_sdk::{contract, contractimpl, Env, String};

/// Test Contract
///
/// This is a simple test contract used for development, testing, and
/// demonstration purposes. It provides basic functionality to verify
/// contract deployment and execution.
#[contract]
pub struct TestContract;

#[contractimpl]
impl TestContract {
    /// Simple hello world function for testing.
    ///
    /// This function returns a greeting message and can be used to test
    /// basic contract functionality and deployment.
    ///
    /// # Arguments
    ///
    /// * `_env` - The Soroban environment (unused)
    /// * `_name` - A name parameter (unused in current implementation)
    ///
    /// # Returns
    ///
    /// Returns a greeting string.
    pub fn hello_world(_env: Env, _name: String) -> String {
        String::from_str(&_env, "Hello from Web3 👋")
    }
}

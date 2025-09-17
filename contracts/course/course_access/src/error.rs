// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracterror, panic_with_error, Env};

/// Error types for the course access contract.
///
/// This enum defines all possible error conditions that can occur
/// during course access operations.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CourseAccessError {
    /// User already has access to the specified course
    UserAlreadyHasAccess = 1,
    /// User does not have access to the specified course
    UserNoAccessCourse = 2,
    /// Operation not authorized for the current user
    Unauthorized = 3,
    /// Name field is required but not provided
    NameRequired = 4,
    /// Email field is required but not provided
    EmailRequired = 5,
    /// Country field is required but not provided
    CountryRequired = 6,
}

/// Handles contract errors by panicking with the specified error.
///
/// This function is used to propagate errors in the Soroban contract runtime.
/// When called, it will cause the contract execution to halt and return the error.
///
/// # Arguments
///
/// * `env` - The Soroban environment reference
/// * `error` - The error to propagate
///
/// # Panics
///
/// This function always panics with the provided error, which is the intended behavior
/// for error handling in Soroban contracts.
pub fn HandleError(env: &Env, error: CourseAccessError) -> ! {
    panic_with_error!(env, error);
}

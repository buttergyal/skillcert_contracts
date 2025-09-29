// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracterror, panic_with_error, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    UserAlreadyHasAccess = 1,  
    UserNoAccessCourse = 2,   
    Unauthorized = 3,
    NameRequired = 4,
    EmailRequired = 5,
    CountryRequired = 6,
    InvalidCourseId = 7,
    InvalidUser = 8,
    EmptyCourseId = 9,
    // Removed InvalidInput - replaced with specific validation errors below
    InvalidTransferData = 10,
    SameUserTransfer = 11,
}

pub fn handle_error(env: &Env, error: Error) -> ! {
    panic_with_error!(env, error);
}

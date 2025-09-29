// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracterror, panic_with_error, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadInitialized = 1,
    InvalidMaxPageSize = 2,
    SystemNotInitialized = 3,
    AccessDenied = 4,
    SuperAdminNotRegular = 5,
    OperationFailed = 6,
    MaxAdminsReached = 7,
    CannotRemoveSuperAdmin = 8,
    UserProfileExists = 9,
    NameRequired = 10,
    EmailRequired = 11,
    CountryRequired = 12,
    InvalidEmailFormat = 15,
    EmailAlreadyExists = 16,
    InvalidField = 17,
    InvalidProfilePicURL = 19,
    UserNotFound = 20,
    UserProfileNotFound = 21,
    InactiveUser = 22,
    PageParamTooLarge = 23,
    InvalidTitleLength = 24,
    PasswordMismatch = 25,
    // Rate limiting errors
    RateLimitExceeded = 26,
    RateLimitNotConfigured = 27,
    PasswordTooShort = 28,
    PasswordTooLong = 29,
    PasswordMissingUppercase = 30,
    PasswordMissingLowercase = 31,
    PasswordMissingDigit = 32,
    PasswordMissingSpecialChar = 33,
    RequiredFieldMissing = 34,
    Unauthorized = 35
}

pub fn handle_error(env: &Env, error: Error) -> ! {
    panic_with_error!(env, error);
}

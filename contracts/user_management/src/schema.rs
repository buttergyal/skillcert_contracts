// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserProfile {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub role: UserRole,
    pub country: String,
    pub profession: Option<String>,
    pub goals: Option<String>,
    pub profile_picture: Option<String>,
    pub language: String,
    pub password: String,
    pub confirm_password: String,
    pub specialization: String,
    pub languages: Vec<String>,
    pub teaching_categories: Vec<String>,
    pub user: Address,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum UserRole {
    Student,
    Instructor,
    Admin,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserFilter {
    pub role: Option<UserRole>,
    pub country: Option<String>,
    pub status: Option<UserStatus>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct LightProfile {
    pub name: String,
    pub lastname: String,
    pub specialization: String,
    pub languages: Vec<String>,
    pub teaching_categories: Vec<String>,
    pub role: UserRole,
    pub status: UserStatus,
    pub user_address: Address,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct AdminConfig {
    pub initialized: bool,
    pub super_admin: Address,
    pub max_page_size: u32,
    pub total_user_count: u32,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    UserProfile(Address), // This represents the ("user_profile", user_address) key
    Admin(Address),       // Admin flag per address
    UserProfileLight(Address), // Lightweight profile storage
    UsersIndex,           // List of all registered user addresses
    EmailIndex(String),   // Email to Address mapping for uniqueness
    Admins,               // List of admin addresses
    UserRoles,            // Role assignments
    AdminConfig,          // System configuration
}

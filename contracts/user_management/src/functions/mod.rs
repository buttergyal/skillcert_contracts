// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

pub mod admin_management;
pub mod backup_recovery;
pub mod contract_versioning;
pub mod create_user_profile;
pub mod delete_user;
pub mod edit_user_profile;
pub mod get_user_by_id;
pub mod is_admin;
pub mod list_all_registered_users;
pub mod list_users_with_access;
pub mod rbac;
pub mod save_profile;
pub mod user;
pub mod utils;

#[cfg(test)]
mod test_utils{
use soroban_sdk::{contract, contractimpl, Env};

    #[contract]
    pub struct DummyContract;

    #[contractimpl]
    impl DummyContract {
        pub fn __constructor(_env: Env) {}
    }
}

pub mod schema;
pub mod functions;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Env, String, Address};
use crate::schema::UserProfile;

#[contract]
pub struct UserManagement;

#[contractimpl]
impl UserManagement {
    pub fn save_profile(
        env: Env,
        name: String,
        email: String,
        profession: Option<String>,
        goals: Option<String>,
        country: String,
        user: Address,
    ) -> UserProfile {
        functions::save_profile::user_management_save_profile(
            env,
            name,
            email,
            profession,
            goals,
            country,
            user,
        )
    }
} 
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

    // Aquí agregamos la nueva función get_user_by_id
    pub fn get_user_by_id(env: Env, requester: Address, user_id: Address) -> UserProfile {
        functions::get_user_by_id::get_user_by_id(env, requester, user_id)
    }
}

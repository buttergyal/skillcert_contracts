// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

#![no_std]

pub mod schema;
pub mod functions;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Env, Address};
use crate::schema::UserProfile;

#[contract]
pub struct UserProfileContract;

#[contractimpl]
impl UserProfileContract {
    // Get user profile by address (public function)
    pub fn get_user_profile(env: Env, user_address: Address) -> UserProfile {
        functions::get_user_profile::user_profile_get_user_profile(&env, user_address)
    }

    // Get user profile with privacy check
    // This function respects privacy settings and hides email if not public
    pub fn get_user_profile_with_privacy(
        env: Env, 
        user_address: Address, 
        requester_address: Address
    ) -> UserProfile {
        functions::get_user_profile::user_profile_get_user_profile_with_privacy(
            &env, 
            user_address, 
            requester_address
        )
    }
}

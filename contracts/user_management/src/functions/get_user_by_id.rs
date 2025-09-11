// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::{DataKey, UserProfile};
use crate::error::{Error, handle_error};
use core::iter::Iterator;
use soroban_sdk::{symbol_short, Address, Env, Symbol};

// Optional: event symbol
const EVT_GET_USER: Symbol = symbol_short!("get_user");

fn is_admin(env: &Env, who: &Address) -> bool {
    // Use the secure admin check from admin_management module
    use crate::schema::AdminConfig;

    // Check if system is initialized
    let config: Option<AdminConfig> = env.storage().persistent().get(&DataKey::AdminConfig);
    match config {
        Some(cfg) if cfg.initialized => {
            // Check if caller is super admin
            if &cfg.super_admin == who {
                return true;
            }

            // Check regular admin list
            let admins: Option<soroban_sdk::Vec<Address>> =
                env.storage()
                    .persistent()
                    .get::<DataKey, soroban_sdk::Vec<Address>>(&DataKey::Admins);
            match admins {
                Some(list) => list.iter().any(|a| a == *who),
                None => false,
            }
        }
        _ => false,
    }
}

/// Get User by ID
/// - Only the profile owner or an admin can access it.
/// - Returns the full profile (assuming no sensitive data like passwords are stored in UserProfile).
pub fn get_user_by_id(env: Env, requester: Address, user_id: Address) -> UserProfile {
    // Require authentication for the requester
    requester.require_auth();

    // Authorization: allow only if the requester is the same as the user_id or is an admin
    let allowed = requester == user_id || is_admin(&env, &requester);
    if !allowed {
        handle_error(&env, Error::AccessDenied); // Generic error message
    }

    // Retrieve the user profile from storage
    let profile: UserProfile = env
        .storage()
        .persistent()
        .get::<DataKey, UserProfile>(&DataKey::UserProfile(user_id.clone()))
        .unwrap_or_else(|| handle_error(&env, Error::AccessDenied)); // Don't disclose if user exists

    // (Optional) Emit a read event
    env.events()
        .publish((EVT_GET_USER, &requester), user_id.clone());

    profile
}

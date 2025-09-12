// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::{AdminConfig, DataKey};
use soroban_sdk::{Address, Env, Vec};

/// Returns true if the given address is an admin (either super admin or in admin list).
pub fn is_admin(env: Env, who: Address) -> bool {
    // Check if system is initialized
    let config: Option<AdminConfig> = env.storage().persistent().get(&DataKey::AdminConfig);
    match config {
        Some(cfg) if cfg.initialized => {
            // Check if caller is super admin
            if cfg.super_admin == who {
                return true;
            }

            // Check regular admin list
            let admins: Option<Vec<Address>> = env
                .storage()
                .persistent()
                .get::<DataKey, Vec<Address>>(&DataKey::Admins);
            match admins {
                Some(list) => list.iter().any(|a| a == who),
                None => false,
            }
        }
        _ => false,
    }
}

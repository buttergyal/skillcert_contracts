// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::{KEY_COURSE_REG_ADDR, KEY_USER_MGMT_ADDR};
use soroban_sdk::{Address, Env};

const KEY_INIT: &str = "init";
const KEY_OWNER: &str = "owner";

/// One-time constructor: sets owner and external contract addresses.
/// Fails if already initialized.
pub fn initialize(env: Env, caller: Address, user_mgmt_addr: Address, course_registry_addr: Address) {
    caller.require_auth();

    // Prevent re-initialization
    if env
        .storage()
        .instance()
        .get::<_, bool>(&((KEY_INIT,),))
        .unwrap_or(false)
    {
        panic!("already initialized");
    }

    let inst = env.storage().instance();
    inst.set(&(KEY_OWNER,), &caller);
    inst.set(&(KEY_USER_MGMT_ADDR,), &user_mgmt_addr);
    inst.set(&(KEY_COURSE_REG_ADDR,), &course_registry_addr);
    inst.set(&(KEY_INIT,), &true);
}

/// Update external contract addresses; only the stored owner can change.
pub fn set_contract_addrs(env: Env, caller: Address, user_mgmt_addr: Address, course_registry_addr: Address) {
    caller.require_auth();

    // Require initialized
    if !env
        .storage()
        .instance()
        .get::<_, bool>(&((KEY_INIT,),))
        .unwrap_or(false)
    {
        panic!("not initialized");
    }

    // Only owner may update
    let owner: Address = env
        .storage()
        .instance()
        .get(&((KEY_OWNER,),))
        .expect("owner missing");
    if caller != owner {
        panic!("only owner");
    }

    let inst = env.storage().instance();
    inst.set(&(KEY_USER_MGMT_ADDR,), &user_mgmt_addr);
    inst.set(&(KEY_COURSE_REG_ADDR,), &course_registry_addr);
}

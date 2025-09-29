// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Address, Env, Symbol, symbol_short};
use soroban_sdk::storage::Instance;

use crate::error::{Error, handle_error};
use crate::schema::{KEY_COURSE_REG_ADDR, KEY_USER_MGMT_ADDR};

const INIT_EVENT: Symbol = symbol_short!("initialz");
const UPDATE_ADDRESS_EVENT: Symbol = symbol_short!("updAddr");


const KEY_INIT: &str = "init";

const KEY_OWNER: &str = "owner";


pub fn initialize(
    env: Env,
    caller: Address,
    user_mgmt_addr: Address,
    course_registry_addr: Address,
) {
    caller.require_auth();

    // Prevent re-initialization
    if env
        .storage()
        .instance()
        .get::<_, bool>(&((KEY_INIT,),))
        .unwrap_or(false)
    {
        handle_error(&env, Error::Initialized);
    }

    let inst: Instance = env.storage().instance();
    inst.set(&(KEY_OWNER,), &caller);
    inst.set(&(KEY_USER_MGMT_ADDR,), &user_mgmt_addr);
    inst.set(&(KEY_COURSE_REG_ADDR,), &course_registry_addr);
    inst.set(&(KEY_INIT,), &true);

    env.events()
        .publish((INIT_EVENT,), (caller, user_mgmt_addr, course_registry_addr));
}


pub fn set_contract_addrs(
    env: Env,
    caller: Address,
    user_mgmt_addr: Address,
    course_registry_addr: Address,
) {
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


    let owner: Address = env
        .storage()
        .instance()
        .get(&((KEY_OWNER,),))
        .expect("owner missing");
    if caller != owner {
        panic!("only owner");
    }

    let inst: Instance = env.storage().instance();
    inst.set(&(KEY_USER_MGMT_ADDR,), &user_mgmt_addr);
    inst.set(&(KEY_COURSE_REG_ADDR,), &course_registry_addr);
    env.events()
        .publish((UPDATE_ADDRESS_EVENT,), (caller, user_mgmt_addr, course_registry_addr));
}

/* /// TTL configuration constants for persistent storage entries
pub const TTL_TTL: u32 = 1000; // time-to-live
pub const TTL_BUMP: u32 = 100; // bump amount on access */

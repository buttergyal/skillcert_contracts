// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::schema::{KEY_COURSE_REG_ADDR, KEY_USER_MGMT_ADDR};
use soroban_sdk::{Address, Env};

/// Storage key for initialization flag
const KEY_INIT: &str = "init";

/// Storage key for contract owner address
const KEY_OWNER: &str = "owner";

/// One-time constructor that sets owner and external contract addresses.
///
/// This function initializes the contract with the necessary configuration
/// and can only be called once during the contract's lifetime.
///
/// # Arguments
///
/// * `env` - The Soroban environment
/// * `caller` - The address of the contract deployer who becomes the owner
/// * `user_mgmt_addr` - Address of the user management contract
/// * `course_registry_addr` - Address of the course registry contract
///
/// # Panics
///
/// Panics if the contract has already been initialized.
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
        // TODO: Implement graceful initialization error handling instead of panic
        panic!("already initialized");
    }

    let inst = env.storage().instance();
    inst.set(&(KEY_OWNER,), &caller);
    inst.set(&(KEY_USER_MGMT_ADDR,), &user_mgmt_addr);
    inst.set(&(KEY_COURSE_REG_ADDR,), &course_registry_addr);
    inst.set(&(KEY_INIT,), &true);
}

/// Update external contract addresses.
///
/// This function allows the contract owner to update the addresses of external
/// contracts that this contract depends on for authentication and authorization.
/// Only the stored owner can perform this operation.
///
/// # Arguments
///
/// * `env` - The Soroban environment
/// * `caller` - The address of the user attempting to update the configuration
/// * `user_mgmt_addr` - New address of the user management contract
/// * `course_registry_addr` - New address of the course registry contract
///
/// # Panics
///
/// Panics if the contract is not initialized or if the caller is not the owner.
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

/// TTL configuration constants for persistent storage entries
pub const TTL_TTL: u32 = 1000; // time-to-live
pub const TTL_BUMP: u32 = 100; // bump amount on access

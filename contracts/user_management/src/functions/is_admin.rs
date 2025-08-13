use soroban_sdk::{Address, Env};
use crate::schema::DataKey;

/// Returns true if the given address has admin flag set in storage.
pub fn is_admin(env: Env, who: Address) -> bool {
    env.storage().persistent().get(&DataKey::Admin(who)).unwrap_or(false)
}

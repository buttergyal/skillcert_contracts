use soroban_sdk::{symbol_short, Address, Env, Symbol};
use crate::schema::{UserProfile, DataKey};

// Optional: event symbol
const EVT_GET_USER: Symbol = symbol_short!("get_user");

fn is_admin(env: &Env, who: &Address) -> bool {
    // Checks if the given address is marked as an admin in storage
    env.storage().get(&DataKey::Admin(who.clone())).unwrap_or(false)
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
        panic!("Unauthorized: only the user or an admin can read this profile");
    }

    // Retrieve the user profile from storage
    let profile: UserProfile = env
        .storage()
        .get(&DataKey::UserProfile(user_id.clone()))
        .unwrap_or_else(|| panic!("User profile not found"));

    // (Optional) Emit a read event
    env.events().publish((EVT_GET_USER, &requester), user_id.clone());

    profile
}
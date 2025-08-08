use soroban_sdk::{contracttype, Address, Env, String, Vec};

/// Lightweight profile returned to callers.
#[derive(Clone)]
#[contracttype]
pub struct LightProfile {
    pub name: String,
    pub country: String,
    pub profession: String, // keep it simple; if optional in your schema, you can switch to Option<String>
}

/// Checks whether who is an admin.
/// Adjust the storage key or logic to your contract.
fn is_admin(env: &Env, who: &Address) -> bool {
    let admins: Option<Vec<Address>> = env.storage().get(&("admins",));
    match admins {
        Some(list) => list.iter().any(|a| a == who),
        None => false,
    }
}

/// Lists all registered users with pagination (admin-only).
///
/// Arguments:
/// - env: Soroban environment
/// - caller: address performing the call (must be admin)
/// - page: zero-based page index
/// - page_size: number of items per page (must be > 0)
///
/// Returns:
/// - Vec<LightProfile> containing the lightweight profile for the requested page.
///
/// Storage expectations (adapt keys to your schema):
/// - ("users_index",) -> Vec<Address>   // ordered list of registered user addresses
/// - (("user_profile_light", Address),) -> LightProfile
pub fn list_all_users(env: Env, caller: Address, page: u32, page_size: u32) -> Vec<LightProfile> {
    // Require the caller to be authenticated
    caller.require_auth();

    // Authorization: only admins can call
    if !is_admin(&env, &caller) {
        panic!("Not authorized");
    }

    // Validate input
    if page_size == 0 {
        panic!("page_size must be greater than 0");
    }

    // Read user index (list of registered user addresses)
    let users_index: Vec<Address> = env
        .storage()
        .get(&("users_index",))
        .unwrap_or_else(|| Vec::new(&env));

    let total = users_index.len();
    if total == 0 {
        return Vec::new(&env);
    }

    // Compute pagination window safely
    // start = page * page_size (saturating, clamped to total)
    let start = {
        let s = (page as u64).saturating_mul(page_size as u64);
        if s > u32::MAX as u64 { u32::MAX } else { s as u32 }
    };
    let start = start.min(total);

    // end = min(start + page_size, total) with saturation
    let end = {
        let e = (start as u64).saturating_add(page_size as u64);
        let e = if e > u32::MAX as u64 { u32::MAX } else { e as u32 };
        e.min(total)
    };

    // Build result page
    let mut result = Vec::new(&env);
    let mut i = start;
    while i < end {
        if let Some(addr) = users_index.get(i) {
            // Fetch lightweight profile for each address.
            // If not found, skip to avoid failing the whole page.
            if let Some(profile) = env
                .storage()
                .get(&(("user_profile_light", addr.clone()),))
            {
                result.push_back(profile);
            }
        }
        i = i.saturating_add(1);
    }

    result
}
use crate::schema::{DataKey, CourseCategory};
use soroban_sdk::{Address, Env, String, Vec};

/// Checks whether who is an admin using the same pattern as user_management contract.
/// This assumes the course_registry contract has its own admin system or uses a similar pattern.
fn is_admin(env: &Env, who: &Address) -> bool {
    // For now, we'll use a simple storage-based admin check
    // In a production environment, you might want to integrate with the user_management contract
    let admins: Option<Vec<Address>> = env.storage().persistent().get(&DataKey::Admins);
    match admins {
        Some(list) => list.iter().any(|a| a == *who),
        None => false,
    }
}

/// Retrieves and increments a sequence used for category IDs.
/// Storage key is DataKey::CategorySeq -> u128.
fn next_category_id(env: &Env) -> u128 {
    let mut seq: u128 = env.storage().persistent().get(&DataKey::CategorySeq).unwrap_or(0u128);
    seq = seq.saturating_add(1);
    env.storage().persistent().set(&DataKey::CategorySeq, &seq);
    seq
}

/// Creates a new course category (admin-only).
///
/// Arguments:
/// - env: Soroban environment.
/// - caller: transaction caller (must be admin).
/// - name: category name (must be non-empty).
/// - description: optional category description.
///
/// Returns:
/// - u128: the newly created category ID.
///
/// Storage used (replace keys if your schema differs):
/// - ("category_seq",) -> u128                // sequence counter
/// - (("category", id),) -> CourseCategory    // category record by id
pub fn course_registry_create_course_category(
    env: Env,
    caller: Address,
    name: String,
    description: Option<String>,
) -> u128 {
    // Authentication and authorization
    caller.require_auth();
    if !is_admin(&env, &caller) {
        panic!("Not authorized");
    }

    // Basic validation
    if name.is_empty() {
        panic!("Name is required");
    }

    // Generate a new category ID
    let id = next_category_id(&env);

    // Build and persist the category
    let category = CourseCategory {
        id,
        name,
        description,
    };
    env.storage().persistent().set(&DataKey::CourseCategory(id), &category);

    // Return the new ID
    id
}

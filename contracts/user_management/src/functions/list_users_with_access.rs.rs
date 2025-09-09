use soroban_sdk::{symbol_short, Address, Env, Symbol, Vec};

const EVT_ACCESS_LISTED: Symbol = symbol_short!("ac_listed");

/// Helper function to check if the given address is an admin.
/// Adjust the storage key or logic to match your contract.
fn is_admin(env: &Env, who: &Address) -> bool {
    // Retrieve the list of admin addresses from storage
    let admins: Option<Vec<Address>> = env.storage().get(&("admins",));
    match admins {
        Some(list) => list.iter().any(|a| a == who),
        None => false,
    }
}

/// Helper function to check if the given address is the creator of the course.
/// Adjust the storage key or logic to match your contract.
fn is_creator(env: &Env, course_id: u128, who: &Address) -> bool {
    // Retrieve the creator address for the course from storage
    let creator: Address = env
        .storage()
        .get(&(("course_creator", course_id),))
        .expect("Course not found");
    creator == *who
}

/// List all users who currently have access to a course.
///
/// - Only callable by the course creator or an admin.
/// - Emits an event with the number of users listed (optional).
///
/// # Arguments
/// * env - Soroban environment.
/// * caller - Address of the caller.
/// * course_id - Identifier of the course.
///
/// # Returns
/// * Vec<Address> - List of addresses with access to the course.
pub fn list_users_with_access(env: Env, caller: Address, course_id: u128) -> Vec<Address> {
    // Require the caller to be authenticated
    caller.require_auth();

    // Authorization: must be course creator or admin
    if !(is_creator(&env, course_id, &caller) || is_admin(&env, &caller)) {
        panic!("Not authorized");
    }

    // Retrieve the list of users with access from storage
    let access_list: Vec<Address> = env
        .storage()
        .get(&(("course_access", course_id),))
        .unwrap_or_else(|| Vec::new(&env));

    // Optional: Emit an event with the number of users
    env.events().publish(
        (EVT_ACCESS_LISTED,),
        (course_id, caller.clone(), access_list.len() as u32),
    );

    access_list
}
use soroban_sdk::{Address, Env, String};

pub fn course_access_revoke_access(env: Env, course_id: String, user: Address) -> bool {
    // Create storage key
    let key = ("course_access", (course_id, user));

    // Check if the CourseAccess entry exists in storage
    if env.storage().instance().has(&key) {
        // Remove the CourseAccess entry from storage
        env.storage().instance().remove(&key);
        true
    } else {
        // Fail if key does not exist
        false
    }
}

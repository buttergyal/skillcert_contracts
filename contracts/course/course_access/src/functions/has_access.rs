use soroban_sdk::{Env, Address, String};
use crate::schema::DataKey;

/// Check if a user has access to a specific course

pub fn course_access_has_access(env: Env, course_id: String, user: Address) -> bool {
    let key = DataKey::CourseAccess(course_id, user);
    env.storage().persistent().has(&key)
}

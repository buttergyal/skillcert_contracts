use crate::schema::{CourseAccess, DataKey};
use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

const COURSE_TRANSFER_EVENT: Symbol = symbol_short!("transfer");

// Transfer course access from one user to another
pub fn transfer_course_access(env: Env, course_id: String, from: Address, to: Address) {
    // Create the storage key for this course and current  user combination
    let key: DataKey = DataKey::CourseAccess(course_id.clone(), from.clone());

    // Check if access exists to transfer
    if !env.storage().persistent().has(&key) {
        panic!("User does not have access to this course");
    }

    // Create the course access entry for the new user
    let course_access: CourseAccess = CourseAccess {
        course_id: course_id.clone(),
        user: to.clone(),
    };

    // Store the access entry with the composite key for the new user
    env.storage().persistent().set(
        &DataKey::CourseAccess(course_id.clone(), to.clone()),
        &course_access,
    );

    // Remove the old user's access
    env.storage().persistent().remove(&key);

    // Extend the TTL for the new user's storage entry
    env.storage().persistent().extend_ttl(
        &DataKey::CourseAccess(course_id.clone(), to.clone()),
        100,
        1000,
    );

    //emit an event
    env.events()
        .publish((COURSE_TRANSFER_EVENT,), (course_id, from, to));
}

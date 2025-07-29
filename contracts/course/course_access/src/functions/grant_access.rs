use soroban_sdk::{Env, Address, String};
use crate::schema::{CourseAccess, DataKey};

/// Grant access to a specific user for a given course

pub fn course_access_grant_access(env: Env, course_id: String, user: Address) {
    // Create the storage key for this course and user combination
    let key: DataKey = DataKey::CourseAccess(course_id.clone(), user.clone());
    
    // Check if access already exists to prevent duplicates
    if env.storage().persistent().has(&key) {
        panic!("User already has access to this course");
    }
    
    // Create the course access entry
    let course_access: CourseAccess = CourseAccess {
        course_id: course_id.clone(),
        user: user.clone(),
    };
    
    // Store the access entry with the composite key
    env.storage().persistent().set(&key, &course_access);
    
    // Extend the TTL for the storage entry to ensure it persists
    env.storage().persistent().extend_ttl(&key, 100, 1000);
}

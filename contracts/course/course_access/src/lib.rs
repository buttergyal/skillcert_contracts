mod schema;
mod functions;

use soroban_sdk::{contract, contractimpl, Env, Address, String};

pub use schema::*;
pub use functions::*;


#[contract]
pub struct CourseAccessContract;

#[contractimpl]
impl CourseAccessContract {
    /// Grant access to a specific user for a given course
  
    pub fn grant_access(env: Env, course_id: String, user: Address) {
        course_access_grant_access(env, course_id, user);
    }

    /// Check if a user has access to a specific course

    /* pub fn has_access(env: Env, course_id: String, user: Address) -> bool {
        course_access_has_access(env, course_id, user)
    }
 */
    /// Revoke access for a specific user from a course
    pub fn revoke_access(env: Env, course_id: String, user: Address) -> bool {
        course_access_revoke_access(env, course_id, user)
    }
}

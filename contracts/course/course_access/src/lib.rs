mod functions;
mod schema;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

pub use functions::*;
pub use schema::{CourseUsers, UserCourses};

#[contract]
pub struct CourseAccessContract;

#[contractimpl]
impl CourseAccessContract {
    /// Grant access to a specific user for a given course
    pub fn grant_access(env: Env, course_id: String, user: Address) {
        course_access_grant_access(env, course_id, user)
    }

    /// Revoke access for a specific user from a course
    pub fn revoke_access(env: Env, course_id: String, user: Address) -> bool {
        course_access_revoke_access(env, course_id, user)
    }

    /// Save or update a user's profile on-chain
    pub fn save_profile(
        env: Env,
        name: String,
        email: String,
        profession: Option<String>,
        goals: Option<String>,
        country: String,
    ) {
        let user = env.current_contract_address();
        save_profile(env, name, email, profession, goals, country, user);
    }

    /// List all courses a user has access to
    pub fn list_user_courses(env: Env, user: Address) -> UserCourses {
        course_access_list_user_courses(env, user)
    }

    /// List all users who have access to a course
    pub fn list_course_access(env: Env, course_id: String) -> CourseUsers {
        course_access_list_course_access(env, course_id)
    }
}

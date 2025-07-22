use soroban_sdk::{Env, Address, String};
use crate::schema::{CourseAccess, DataKey};

pub fn course_access_list_user_courses(env: Env, course_id: String, user: Address) -> CourseAccess {
    // Create the storage key for this course and user combination
    let key: DataKey = DataKey::CourseAccess(course_id.clone(), user.clone());
    
    let course_access: CourseAccess = env
        .storage()
        .persistent()
        .get(&(key))
        .expect("No such user and course combination found");
    
    course_access
}

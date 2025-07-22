use soroban_sdk::{contracttype, Address, String, Vec};

/// Represents access permission for a user to a specific course
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CourseAccess {
    pub course_id: String,
    pub user: Address,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct UserCourses {
    pub user: Address,
    pub courses: Vec<String>,
}

/// Storage key types for the course access contract
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    /// Key for storing course access: (course_id, user) -> CourseAccess
    CourseAccess(String, Address),
}

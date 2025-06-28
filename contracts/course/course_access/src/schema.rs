use soroban_sdk::{contracttype, Address, String};

/// Represents access permission for a user to a specific course
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CourseAccess {
    pub course_id: String,
    pub user: Address,
}

/// Storage key types for the course access contract
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    /// Key for storing course access: (course_id, user) -> CourseAccess
    CourseAccess(String, Address),
}

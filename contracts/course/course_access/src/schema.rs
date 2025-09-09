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

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    /// Key for storing course access: (course_id, user) -> CourseAccess
    CourseAccess(String, Address),
    /// Key for storing user profile: user -> UserProfile
    UserProfile(Address),
    /// Key for storing courses per user: user -> UserCourses
    UserCourses(Address),
    /// Key for storing users per course: course_id -> CourseUsers
    CourseUsers(String),
}

/// Represents a user's profile information
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct UserProfile {
    pub name: String,
    pub email: String,
    pub profession: Option<String>,
    pub goals: Option<String>,
    pub country: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CourseUsers {
    pub course: String,
    pub users: Vec<Address>,
}

// Global config keys
pub const KEY_USER_MGMT_ADDR: &str = "user_mgmt_addr";
pub const KEY_COURSE_REG_ADDR: &str = "course_registry_addr";

use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseModule {
    pub id: String,
    pub course_id: String,
    pub position: u32,
    pub title: String,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseGoal {
    pub goal_id: String,
    pub course_id: String,
    pub content: String,
    pub created_by: Address,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    Module(String),
    Courses,
    CourseGoalList(String),     // Optional: Keep a list of goal IDs per course
    CourseGoal(String, String), // (course_id, goal_id)
    CoursePrerequisites(String),
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator: Address,
    pub price: u128,
    pub category: Option<String>,
    pub language: Option<String>,
    pub thumbnail_url: Option<String>,
    pub published: bool,
    pub prerequisites: Vec<CourseId>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseId {
    pub id: String,
    pub count: u128,
}

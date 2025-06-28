use soroban_sdk::{contracttype, Env, String, Address};

#[contracttype]
#[derive(Clone)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator: Address,
    pub published: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct CourseModule {
    pub id: String,
    pub course_id: String,
    pub position: i32,
    pub title: String,
    pub created_at: u64,
}

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
pub enum DataKey {
    Module(String), // This would represent the ("module", module_id) key
    Courses,        // If courses are stored as a single map
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

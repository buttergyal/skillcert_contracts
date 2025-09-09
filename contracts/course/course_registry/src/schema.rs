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
pub struct CourseCategory {
    pub id: u128,
    pub name: String,
    pub description: Option<String>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    Module(String),
    Courses,
    CourseGoalList(String),     // Optional: Keep a list of goal IDs per course
    CourseGoal(String, String), // (course_id, goal_id)
    CoursePrerequisites(String),
    CategorySeq,                // Sequence counter for category IDs
    CourseCategory(u128),       // Course category by ID
    Admins,                     // List of admin addresses
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
    pub is_archived: bool,
    pub level: Option<CourseLevel>,
    pub duration_hours: Option<u32>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseId {
    pub id: String,
    pub count: u128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Category {
    pub name: String,
    pub count: u128,
}

// Course level as string to avoid Soroban enum serialization issues
// Valid values: "Beginner", "Intermediate", "Advanced"
pub type CourseLevel = String;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CourseFilters {
    pub min_price: Option<u128>,
    pub max_price: Option<u128>,
    pub category: Option<String>,
    pub level: Option<CourseLevel>,
    pub min_duration: Option<u32>,
    pub max_duration: Option<u32>,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EditCourseParams {
    pub new_title: Option<String>,
    pub new_description: Option<String>,
    pub new_price: Option<u128>,
    pub new_category: Option<Option<String>>,
    pub new_language: Option<Option<String>>,
    pub new_thumbnail_url: Option<Option<String>>,
    pub new_published: Option<bool>,
    pub new_level: Option<Option<CourseLevel>>,
    pub new_duration_hours: Option<Option<u32>>,
}
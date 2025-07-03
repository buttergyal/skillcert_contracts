use soroban_sdk::{contracttype, Address, String};



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
pub struct Course {
    pub id: String,
    pub title: String,
    pub owner_id: Address,
    pub created_at: u64,
}


#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    Module(String), // This would represent the ("module", module_id) key
    Courses, // If courses are stored as a single map
}

use soroban_sdk::{Address, String, contracttype};

#[contracttype]
#[derive(Clone,)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator: Address,
    pub published: bool,
}


#[contracttype]
#[derive(Clone,)]
pub  struct CourseId {
    pub id: String,
    pub count: u128,
}
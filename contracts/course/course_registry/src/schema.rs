
use soroban_sdk::{Address, String, contracttype};

#[derive(Clone,)]
#[contracttype]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator: Address,
    pub published: bool,
}


#[derive(Clone,)]
#[contracttype]
pub  struct CourseId {
    pub id: String,
    pub count: u128,
}
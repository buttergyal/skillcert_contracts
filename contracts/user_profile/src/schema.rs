use soroban_sdk::{Address, String, contracttype};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserProfile {
    pub address: Address,
    pub name: String,
    pub email: Option<String>, // Optional email for privacy
    pub country: String,
    pub profession: String,
    pub goals: String,
    pub privacy_public: bool, // If true, profile is publicly viewable
    pub created_at: u64,
    pub updated_at: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Profile(Address), // Key for storing user profiles
}

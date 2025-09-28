/// Models for the User Management Contract
pub mod user;

// Using 
use soroban_sdk::{contracttype, Address, String};


/// Data keys for contract storage
/// 
/// Currently includes only UserProfile keyed by user Address
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    UserProfile(Address),
    EmailIndex(String), // To ensure email uniqueness
}
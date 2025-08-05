use soroban_sdk::{Env, Address, String};
use crate::schema::{UserProfile, DataKey};

pub fn save_profile(
    env: Env,
    name: String,
    email: String,
    profession: Option<String>,
    goals: Option<String>,
    country: String,
    user: Address,
) {
    // Validate required fields
    if name.is_empty() {
        panic!("Name is required");
    }
    if email.is_empty() {
        panic!("Email is required");
    }
    if country.is_empty() {
        panic!("Country is required");
    }

    let profile = UserProfile {
        name,
        email,
        profession,
        goals,
        country,
    };
    
    env.storage().set(&DataKey::UserProfile(user), &profile);
}

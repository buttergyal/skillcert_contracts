use crate::schema::{DataKey, UserProfile};
use soroban_sdk::{Address, Env, String};

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

    env.storage()
        .persistent()
        .set(&DataKey::UserProfile(user), &profile);
}

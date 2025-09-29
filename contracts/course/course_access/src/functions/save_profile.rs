// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Address, Env, String, Symbol, symbol_short};

use crate::error::{handle_error, Error};
use crate::schema::{DataKey, UserProfile};

const SAVE_USER_PROFILE_EVENT: Symbol = symbol_short!("saveUsPrl");


pub fn save_user_profile(
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
        handle_error(&env, Error::NameRequired)
    }
    // TODO: Implement full email validation according to RFC 5322 standard
    if email.is_empty() {
        handle_error(&env, Error::EmailRequired)
    }
    if country.is_empty() {
        handle_error(&env, Error::CountryRequired)
    }

    let profile: UserProfile = UserProfile {
        name: name.clone(),
        email: email.clone(),
        profession: profession.clone(),
        goals: goals.clone(),
        country: country.clone(),
    };

    env.storage()
        .persistent()
        .set(&DataKey::UserProfile(user.clone()), &profile);

    env.events()
        .publish((SAVE_USER_PROFILE_EVENT,), (name, email, profession, goals, country, user));
}

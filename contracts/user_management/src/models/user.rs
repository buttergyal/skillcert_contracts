
use soroban_sdk::{contracttype, String};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserProfile {
    /// User's full name (required)
    pub full_name: String,
    /// User's contact email address (required, must be unique)
    pub contact_email: String,
    /// User's profession or job title (optional)
    pub profession: Option<String>,
    /// User's country of residence (optional)
    pub country: Option<String>,
    /// User's learning goals or purpose (optional)
    pub purpose: Option<String>,
    /// User's profile picture URL (optional)
    pub profile_picture_url: Option<String>,
}

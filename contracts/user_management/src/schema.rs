use soroban_sdk::{Address, String, contracttype, IntoVal, TryFromVal, Val, Env};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct UserProfile {
    pub name: String,
    pub email: String,
    pub profession: Option<String>,
    pub goals: Option<String>,
    pub country: String,
    pub user: Address,
}

impl IntoVal<Env, Val> for UserProfile {
    fn into_val(&self, env: &Env) -> Val {
        (
            self.name.clone(),
            self.email.clone(),
            self.profession.clone(),
            self.goals.clone(),
            self.country.clone(),
            self.user.clone(),
        )
            .into_val(env)
    }
}

impl TryFromVal<Env, Val> for UserProfile {
    type Error = soroban_sdk::ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let (name, email, profession, goals, country, user): (
            String,
            String,
            Option<String>,
            Option<String>,
            String,
            Address,
        ) = TryFromVal::try_from_val(env, val)?;
        Ok(UserProfile {
            name,
            email,
            profession,
            goals,
            country,
            user,
        })
    }
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DataKey {
    UserProfile(Address), // This represents the ("user_profile", user_address) key
    Admin(Address),       // Admin flag per address
}
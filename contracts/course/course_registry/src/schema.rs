
use soroban_sdk::{Address, String, contracttype, IntoVal, TryFromVal, Val, Env};


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
pub enum DataKey {
    Module(String), // This would represent the ("module", module_id) key
    Courses, // If courses are stored as a single map
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator: Address,
    pub published: bool,
}

impl IntoVal<Env, Val> for Course {
    fn into_val(&self, env: &Env) -> Val {
        (
            self.id.clone(),
            self.title.clone(),
            self.description.clone(),
            self.creator.clone(),
            self.published,
        )
            .into_val(env)
    }
}

impl TryFromVal<Env, Val> for Course {
    type Error = soroban_sdk::ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let (id, title, description, creator, published): (
            String,
            String,
            String,
            Address,
            bool,
        ) = TryFromVal::try_from_val(env, val)?;
        Ok(Course {
            id,
            title,
            description,
            creator,
            published,
        })
    }
}

#[contracttype]
#[derive(Clone)]
pub struct CourseId {
    pub id: String,
    pub count: u128,
}

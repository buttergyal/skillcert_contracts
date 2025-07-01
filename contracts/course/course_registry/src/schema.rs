


#[contracttype]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub creator: Address,
    pub published: bool,
}
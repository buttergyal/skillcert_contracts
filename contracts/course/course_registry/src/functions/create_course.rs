use soroban_sdk::{symbol_short, Address, Env, String, Symbol};
use crate::schema::Course;

const COURSE_KEY: Symbol = symbol_short!("course");
const TITLE_KEY: Symbol = symbol_short!("title");
const COURSE_ID: Symbol = symbol_short!("course");

/// Creates a new course and stores it in persistent storage.
/// Validates title, price, and uniqueness before saving.
/// Emits a `course_created` event upon success.
pub fn course_registry_create_course(
    env: Env, 
    title: String, 
    description: String,
    price: u128,
    category: Option<String>,
    language: Option<String>,
    thumbnail_url: Option<String>
) -> Course {
    let caller: Address = env.current_contract_address();
    
    // Validate that title is not empty or just whitespace
    let title_string = title.to_string();
    let trimmed_title = title_string.trim();
    if title.is_empty() || trimmed_title.is_empty() { 
        panic!("Course error: Course Title cannot be empty");
    }

    // Validate that price is greater than 0
    if price == 0 {
        panic!("Course error: Price must be greater than 0");
    }

    // Prevent duplicate titles (case-insensitive)
    let title_key: (Symbol, String) = (
        TITLE_KEY, 
        String::from_str(&env, title.to_lowercase().as_str())
    );

    if env.storage().persistent().has(&title_key) { 
        panic!("Course error: Course Title already exists");
    }

    // Generate a unique ID for the course
    let id: u128 = generate_course_id(&env);
    let converted_id: String = String::from_str(&env, id.to_string().as_str());
    let storage_key: (Symbol, String) = (COURSE_KEY, converted_id.clone());

    if env.storage().persistent().has(&storage_key) {
        panic!("Course with this ID already exists");
    }

    // Set default language if none is provided
    let language = match language {
        Some(lang) => Some(lang),
        None => Some(String::from_str(&env, "en")),
    };

    // Build the Course struct
    let new_course: Course = Course {
        id: converted_id.clone(),
        title,
        description,
        creator: caller,
        price,
        category,
        language,
        thumbnail_url,
        published: false,
    };

    // Save the course to persistent storage
    env.storage().persistent().set(&storage_key, &new_course);
    env.storage().persistent().set(&title_key, &true);

    // Emit course_created event
    emit_course_created_event(&env, &new_course);

    new_course
}

/// Generates a new sequential course ID and stores it in persistent storage
pub fn generate_course_id(env: &Env) -> u128 {
    let current_id: u128 = env.storage().persistent()
        .get(&COURSE_ID)
        .unwrap_or(0);
    let new_id = current_id + 1;
    env.storage().persistent().set(&COURSE_ID, &new_id);
    new_id
}

/// Emits an event when a course is successfully created
pub fn emit_course_created_event(env: &Env, course: &Course) {
    env.events().publish(
        ("course_created", course.id.clone()),
        course,
    );
}

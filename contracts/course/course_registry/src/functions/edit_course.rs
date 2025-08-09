// contracts/course_registry/src/functions/edit_course.rs
use crate::schema::Course;
use soroban_sdk::{symbol_short, Address, Env, String, Symbol, Vec};

const COURSE_KEY: Symbol = symbol_short!("course");
const TITLE_KEY: Symbol = symbol_short!("title");

const EDIT_COURSE_EVENT: Symbol = symbol_short!("editcours");

pub fn course_registry_edit_course(
    env: Env,
    course_id: String,

    // Editable fields (use Option for "provided or not")
    new_title: Option<String>,
    new_description: Option<String>,
    new_price: Option<u128>,

    // Double-Option lets caller clear the value: Some(None) -> clear, None -> no change
    new_category: Option<Option<String>>,
    new_language: Option<Option<String>>,
    new_thumbnail_url: Option<Option<String>>,

    new_published: Option<bool>,
) -> Course {
    // --- Load existing course ---
    let storage_key: (Symbol, String) = (COURSE_KEY, course_id.clone());
    let mut course: Course = env
        .storage()
        .persistent()
        .get(&storage_key)
        .expect("Course error: Course not found");

    // --- Permission: only creator can edit ---
    let caller: Address = env.current_contract_address();
    if caller != course.creator {
        panic!("Course error: Unauthorized edit");
    }

    // --- Title update (validate + uniqueness) ---
    if let Some(t) = new_title {
        let t_str = t.to_string();
        let t_trim = t_str.trim();
        if t_trim.is_empty() {
            panic!("Course error: Course Title cannot be empty");
        }

        // Only check/rotate title index if it's effectively changing (case-insensitive)
        let old_title_lc = course.title.to_string().to_lowercase();
        let new_title_lc = t_str.to_lowercase();

        if old_title_lc != new_title_lc {
            // uniqueness index key for the *new* title
            let new_title_key: (Symbol, String) =
                (TITLE_KEY, String::from_str(&env, &new_title_lc));
            if env.storage().persistent().has(&new_title_key) {
                panic!("Course error: Course Title already exists");
            }

            // remove old title index and set new one
            let old_title_key: (Symbol, String) =
                (TITLE_KEY, String::from_str(&env, &old_title_lc));
            env.storage().persistent().remove(&old_title_key);
            env.storage().persistent().set(&new_title_key, &true);

            course.title = String::from_str(&env, t_trim);
        }
    }

    // --- Description ---
    if let Some(d) = new_description {
        course.description = d;
    }

    // --- Price (>0) ---
    if let Some(p) = new_price {
        if p == 0 {
            panic!("Course error: Price must be greater than 0");
        }
        course.price = p;
    }

    // --- Optional fields: category / language / thumbnail ---
    if let Some(cat) = new_category {
        course.category = cat; // Some(value) sets; None clears
    }
    if let Some(lang) = new_language {
        course.language = lang;
    }
    if let Some(url) = new_thumbnail_url {
        course.thumbnail_url = url;
    }

    // --- Published flag ---
    if let Some(p) = new_published {
        course.published = p;
    }

    // --- Persist updated course ---
    env.storage().persistent().set(&storage_key, &course);

    // --- Emit event ---
    env.events()
        .publish((EDIT_COURSE_EVENT, course_id.clone()), course.clone());

    course
}

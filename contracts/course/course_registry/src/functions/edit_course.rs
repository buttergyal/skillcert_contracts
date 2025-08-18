use crate::schema::Course;
use soroban_sdk::{symbol_short, Address, Env, String, Symbol};
use super::utils::{trim, to_lowercase};

const COURSE_KEY: Symbol = symbol_short!("course");
const TITLE_KEY: Symbol = symbol_short!("title");

const EDIT_COURSE_EVENT: Symbol = symbol_short!("editcours");

pub fn course_registry_edit_course(
    env: Env,
    creator: Address,
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
    creator.require_auth();

    // --- Load existing course ---
    let storage_key: (Symbol, String) = (COURSE_KEY, course_id.clone());
    let mut course: Course = env
        .storage()
        .persistent()
        .get(&storage_key)
        .expect("Course error: Course not found");

    // --- Permission: only creator can edit ---
    if creator != course.creator {
        panic!("Course error: Unauthorized edit");
    }

    // --- Title update (validate + uniqueness) ---
    if let Some(t) = new_title {
        let t_str = t;
        let t_trim = trim(&env, &t_str);
        if t_trim.is_empty() {
            panic!("Course error: Course Title cannot be empty");
        }

        // Only check/rotate title index if it's effectively changing (case-insensitive)
        let old_title_lc = to_lowercase(&env,&course.title);
        let new_title_lc = to_lowercase(&env, &t_str);

        if old_title_lc != new_title_lc {
            // uniqueness index key for the *new* title
            let new_title_key: (Symbol, String) =
                (TITLE_KEY, new_title_lc);
            if env.storage().persistent().has(&new_title_key) {
                panic!("Course error: Course Title already exists");
            }

            // remove old title index and set new one
            let old_title_key: (Symbol, String) =
                (TITLE_KEY, old_title_lc);
            env.storage().persistent().remove(&old_title_key);
            env.storage().persistent().set(&new_title_key, &true);

            course.title = t_trim;
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

#[cfg(test)]
mod test {
    use crate::schema::Course;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_edit_course_success() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &Some(String::from_str(&env, "original_category")),
            &Some(String::from_str(&env, "original_language")),
            &Some(String::from_str(&env, "original_thumbnail")),
        );

        let edited_course = client.edit_course(
            &creator,
            &course.id,
            &Some(String::from_str(&env, "New Title")),
            &Some(String::from_str(&env, "New Description")),
            &Some(2000_u128),
            &Some(Some(String::from_str(&env, "new_category"))),
            &Some(Some(String::from_str(&env, "new_language"))),
            &Some(Some(String::from_str(&env, "new_thumbnail"))),
            &Some(true),
        );

        assert_eq!(edited_course.title, String::from_str(&env, "New Title"));
        assert_eq!(
            edited_course.description,
            String::from_str(&env, "New Description")
        );
        assert_eq!(edited_course.price, 2000_u128);
        assert_eq!(
            edited_course.category,
            Some(String::from_str(&env, "new_category"))
        );
        assert_eq!(
            edited_course.language,
            Some(String::from_str(&env, "new_language"))
        );
        assert_eq!(
            edited_course.thumbnail_url,
            Some(String::from_str(&env, "new_thumbnail"))
        );
        assert_eq!(edited_course.published, true);
        assert_eq!(edited_course.creator, creator);

        let retrieved_course = client.get_course(&course.id);
        assert_eq!(retrieved_course.title, String::from_str(&env, "New Title"));
        assert_eq!(
            retrieved_course.description,
            String::from_str(&env, "New Description")
        );
        assert_eq!(retrieved_course.price, 2000_u128);
        assert_eq!(
            retrieved_course.category,
            Some(String::from_str(&env, "new_category"))
        );
        assert_eq!(
            retrieved_course.language,
            Some(String::from_str(&env, "new_language"))
        );
        assert_eq!(
            retrieved_course.thumbnail_url,
            Some(String::from_str(&env, "new_thumbnail"))
        );
        assert_eq!(retrieved_course.published, true);
    }

    #[test]
    #[should_panic(expected = "Course error: Unauthorized edit")]
    fn test_edit_course_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let impostor: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &None,
            &None,
            &None,
        );

        client.edit_course(
            &impostor,
            &course.id,
            &Some(String::from_str(&env, "New Title")),
            &None,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    #[should_panic(expected = "Course error: Course not found")]
    fn test_edit_course_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let fake_course_id = String::from_str(&env, "nonexistent_course");

        client.edit_course(
            &creator,
            &fake_course_id,
            &Some(String::from_str(&env, "New Title")),
            &None,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    #[should_panic(expected = "Course error: Course Title cannot be empty")]
    fn test_edit_course_empty_title() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &None,
            &None,
            &None,
        );

        client.edit_course(
            &creator,
            &course.id,
            &Some(String::from_str(&env, "")),
            &None,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    #[should_panic(expected = "Course error: Price must be greater than 0")]
    fn test_edit_course_zero_price() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &None,
            &None,
            &None,
        );

        client.edit_course(
            &creator,
            &course.id,
            &None,
            &None,
            &Some(0_u128),
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    #[should_panic(expected = "Course error: Course Title already exists")]
    fn test_edit_course_duplicate_title() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let _course1: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Course 1"),
            &String::from_str(&env, "Description 1"),
            &1000_u128,
            &None,
            &None,
            &None,
        );

        let course2: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Course 2"),
            &String::from_str(&env, "Description 2"),
            &1000_u128,
            &None,
            &None,
            &None,
        );

        client.edit_course(
            &creator,
            &course2.id,
            &Some(String::from_str(&env, "Course 1")),
            &None,
            &None,
            &None,
            &None,
            &None,
            &None,
        );
    }

    #[test]
    fn test_edit_course_partial_fields() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        // Create a course
        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &Some(String::from_str(&env, "original_category")),
            &Some(String::from_str(&env, "original_language")),
            &Some(String::from_str(&env, "original_thumbnail")),
        );

        let edited_course = client.edit_course(
            &creator,
            &course.id,
            &Some(String::from_str(&env, "New Title")),
            &None,
            &Some(2000_u128),
            &None,
            &None,
            &None,
            &None,
        );

        assert_eq!(edited_course.title, String::from_str(&env, "New Title"));
        assert_eq!(
            edited_course.description,
            String::from_str(&env, "Original Description")
        );
        assert_eq!(edited_course.price, 2000_u128);
        assert_eq!(
            edited_course.category,
            Some(String::from_str(&env, "original_category"))
        );
        assert_eq!(
            edited_course.language,
            Some(String::from_str(&env, "original_language"))
        );
        assert_eq!(
            edited_course.thumbnail_url,
            Some(String::from_str(&env, "original_thumbnail"))
        );
        assert_eq!(edited_course.published, false); // Default value, unchanged
    }

    #[test]
    fn test_edit_course_same_title_no_change() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "Original Title"),
            &String::from_str(&env, "Original Description"),
            &1000_u128,
            &None,
            &None,
            &None,
        );

        let edited_course = client.edit_course(
            &creator,
            &course.id,
            &Some(String::from_str(&env, "original title")), // Same title, different case
            &Some(String::from_str(&env, "New Description")),
            &None,
            &None,
            &None,
            &None,
            &None,
        );

        assert_eq!(
            edited_course.title,
            String::from_str(&env, "Original Title")
        );
        assert_eq!(
            edited_course.description,
            String::from_str(&env, "New Description")
        );
    }
}

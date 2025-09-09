use crate::schema::{DataKey, CourseCategory};
use soroban_sdk::Env;

/// Retrieves a course category by its ID.
///
/// Arguments:
/// - env: Soroban environment.
/// - category_id: the ID of the category to retrieve.
///
/// Returns:
/// - Option<CourseCategory>: the category if found, None otherwise.
///
/// Storage used:
/// - DataKey::CourseCategory(id) -> CourseCategory
pub fn course_registry_get_course_category(env: &Env, category_id: u128) -> Option<CourseCategory> {
    env.storage().persistent().get(&DataKey::CourseCategory(category_id))
}

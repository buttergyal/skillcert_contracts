use crate::schema::{CourseModule, DataKey};
use soroban_sdk::{symbol_short, Env, String};

pub fn course_registry_remove_module(env: &Env, module_id: String) -> Result<(), &'static str> {
    if module_id.len() == 0 {
        return Err("Module ID cannot be empty");
    }

    // Try to get the module data to verify it exists and is a valid CourseModule
    let module: Option<CourseModule> = env
        .storage()
        .persistent()
        .get(&(symbol_short!("module"), module_id.clone()));

    // Validate that the module exists and is a valid CourseModule
    if module.is_none() {
        return Err("Module not found");
    }

    // Delete the CourseModule directly from persistent storage using its key.
    env.storage()
        .persistent()
        .remove(&(symbol_short!("module"), module_id.clone()));

    // Emit an event to indicate the module has been removed.
    env.events().publish((module_id,), "module_removed");

    Ok(())
}

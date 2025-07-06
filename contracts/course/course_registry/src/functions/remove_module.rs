use soroban_sdk::{Env, String};
use crate::schema::{CourseModule, DataKey};

pub fn course_registry_remove_module(env: &Env, module_id: String) -> Result<(), &'static str> {
    if module_id.len() == 0 {
        return Err("Module ID cannot be empty");
    }
    let modules_storage = env.storage().persistent();
    // Construct the specific DataKey for this module
    let module_key = DataKey::Module(module_id.clone());

    // Try to get the module data to verify it exists and is a valid CourseModule
    let module: Option<CourseModule> = modules_storage.get(&module_key);

    // Validate that the module exists and is a valid CourseModule
    if module.is_none() {
        return Err("Module not found");
    }

    // Delete the CourseModule directly from persistent storage using its key.
    modules_storage.remove(&module_key);

    // Emit an event to indicate the module has been removed.
    env.events().publish((module_id,), "module_removed");

    Ok(())
}
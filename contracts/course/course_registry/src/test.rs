use soroban_sdk::{
    testutils::{Address as _}, Address, Env, String
};

use crate::{
    course_registry_remove_module,
    schema::{CourseModule, DataKey},
};

// Helper function to create a test environment
fn create_test_env() -> Env {
    Env::default()
}

// Helper function to create a sample CourseModule
fn create_sample_module(env: &Env) -> CourseModule {
    CourseModule {
        id: String::from_str(env, "test_module_123"),
        course_id: String::from_str(env, "test_course_123"),
        position: 0,
        title: String::from_str(env, "Introduction to Blockchain"),
        created_at: 0,
    }
}

#[test]
fn test_remove_module_success() {
    let env = create_test_env();
    let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let contract_id = contract.address();
    let module = create_sample_module(&env);
    let module_id = module.id.clone();

    env.mock_all_auths();

    // Add module to storage first
    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        let key = DataKey::Module(module_id.clone());
        storage.set(&key, &module);
    });

    // Remove the module
    let result = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module_id.clone())
    });

    // Assert successful removal
    assert!(result.is_ok());

    // Verify module no longer exists in storage
    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        let key = DataKey::Module(module_id.clone());
        assert!(!storage.has(&key));
    });
}

#[test]
fn test_remove_multiple_different_modules() {
    let env = create_test_env();
    let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let contract_id = contract.address();

    env.mock_all_auths();

    // Create two different modules
    let mut module1 = create_sample_module(&env);
    module1.id = String::from_str(&env, "module_1");

    let mut module2 = create_sample_module(&env);
    module2.id = String::from_str(&env, "module_2");

    // Add both modules to storage
    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        storage.set(&DataKey::Module(module1.id.clone()), &module1);
        storage.set(&DataKey::Module(module2.id.clone()), &module2);
    });

    // Remove first module
    let result1 = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module1.id.clone())
    });
    assert!(result1.is_ok());

    // Remove second module
    let result2 = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module2.id.clone())
    });
    assert!(result2.is_ok());

    // Verify both modules are removed from storage
    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        assert!(!storage.has(&DataKey::Module(module1.id.clone())));
        assert!(!storage.has(&DataKey::Module(module2.id.clone())));
    });
}

#[test]
fn test_remove_module_storage_isolation() {
    let env = create_test_env();
    let contract = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let contract_id = contract.address();
    let module = create_sample_module(&env);
    let module_id = module.id.clone();

    env.mock_all_auths();

    // Add modules to storage
    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        let key = DataKey::Module(module_id.clone());
        storage.set(&key, &module);

        // Add some other data to storage (simulating other modules or data)
        let other_key = DataKey::Module(String::from_str(&env, "other_module"));
        let mut other_module = create_sample_module(&env);
        other_module.id = String::from_str(&env, "other_module");
        storage.set(&other_key, &other_module);
    });

    // Remove the target module
    let result = env.as_contract(&contract_id, || {
        course_registry_remove_module(&env, module_id)
    });
    assert!(result.is_ok());

    // Verify target module is removed but other data remains
    env.as_contract(&contract_id, || {
        let storage = env.storage().persistent();
        let other_key = DataKey::Module(String::from_str(&env, "other_module"));
        assert!(!storage.has(&DataKey::Module(module.id)));
        assert!(storage.has(&other_key)); // Other data should still exist
    });
}
pub use crate::schema::{Course, CourseModule};
use soroban_sdk::{symbol_short, Env, String, Symbol, vec};
use super::utils::{concat_strings, u32_to_string};

const COURSE_KEY: Symbol = symbol_short!("course");
const MODULE_KEY: Symbol = symbol_short!("module");

pub fn course_registry_add_module(
    env: Env,
    course_id: String,
    position: u32,
    title: String,
) -> CourseModule {
    // Verify course exists
    let course_storage_key: (Symbol, String) = (COURSE_KEY, course_id.clone());

    // require!(env.storage().persistent().has(&course_storage_key), "Course with the specified ID does not exist");

    if !env.storage().persistent().has(&course_storage_key) {
        panic!("Course with the specified ID does not exist");
    }

    let ledger_seq: u32 = env.ledger().sequence();

     let arr = vec![
        &env, String::from_str(&env, "module_"), 
        course_id.clone(), 
        String::from_str(&env, "_"),
        u32_to_string(&env, position),
        String::from_str(&env, "_"),
        u32_to_string(&env, ledger_seq)
        ];   

    let module_id = concat_strings(&env, arr);


    // Create new module
    let module: CourseModule = CourseModule {
        id: module_id.clone(),
        course_id,
        position,
        title,
        created_at: env.ledger().timestamp(),
    };

    let storage_key: (Symbol, String) = (MODULE_KEY, module_id.clone());

    env.storage().persistent().set(&storage_key, &module);

    module
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn create_course<'a>(client: &CourseRegistryClient<'a>, creator: &Address) -> Course {
        let title = String::from_str(&client.env, "title");
        let description = String::from_str(&client.env, "description");
        let price = 1000_u128;
        client.create_course(&creator, &title, &description, &price, &None, &None, &None, &None, &None)
    }

    #[test]
    fn test_add_module_success() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);

        let module = client.add_module(&course.id, &1, &String::from_str(&env, "Module 1"));

        assert_eq!(module.course_id, course.id);
        assert_eq!(module.position, 1);
        assert_eq!(module.title, String::from_str(&env, "Module 1"));
    }

    #[test]
    #[should_panic(expected = "Course with the specified ID does not exist")]
    fn test_add_module_invalid_course() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        client.add_module(
            &String::from_str(&env, "invalid_course"),
            &1,
            &String::from_str(&env, "Module 1"),
        );
    }

    #[test]
    fn test_course_registry_add_module_generates_unique_ids() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);

        let module1 = client.add_module(&course.id, &1, &String::from_str(&env, "Module 1"));
        let module2 = client.add_module(&course.id, &2, &String::from_str(&env, "Module 2"));

        assert_ne!(module1.id, module2.id);
    }

    #[test]
    fn test_course_registry_add_module_storage_key_format() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let course = create_course(&client, &creator);

        let module = client.add_module(&course.id, &1, &String::from_str(&env, "Module 1"));

        let exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(MODULE_KEY, module.id.clone()))
        });

        assert!(exists);
    }
}

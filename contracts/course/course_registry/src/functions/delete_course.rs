use crate::schema::{Course, CourseModule};
use soroban_sdk::{
    symbol_short, Address, Env, String, Symbol, Vec,vec
};
use super::utils::{concat_strings, u32_to_string, to_lowercase};

const COURSE_KEY: Symbol = symbol_short!("course");
const MODULE_KEY: Symbol = symbol_short!("module");
const TITLE_KEY: Symbol = symbol_short!("title");

pub fn course_registry_delete_course(
    env: &Env,
    creator: Address,
    course_id: String,
) -> Result<(), &'static str> {
    creator.require_auth();

    if course_id.is_empty() {
        return Err("Course ID cannot be empty");
    }

    let course_storage_key = (COURSE_KEY, course_id.clone());

    if !env.storage().persistent().has(&course_storage_key) {
        return Err("Course not found");
    }

    let course: Course = env
        .storage()
        .persistent()
        .get(&course_storage_key)
        .ok_or("Course not found")?;

    if course.creator != creator {
        return Err("Unauthorized");
    }

    delete_course_modules(env, &course_id);

    let lowercase_title = to_lowercase(env, &course.title);

    let title_key = (
        TITLE_KEY,
        lowercase_title
    );
    env.storage().persistent().remove(&title_key);
    env.storage().persistent().remove(&course_storage_key);
    env.events().publish((course_id,), "course_deleted");

    Ok(())
}

fn delete_course_modules(env: &Env, course_id: &String) {
    let mut modules_to_delete: Vec<String> = Vec::new(&env);

    let mut counter = 0u32;
    loop {
        let arr = vec![
            &env, String::from_str(&env, "module_"), 
            course_id.clone(), 
            String::from_str(&env, "_"),
            u32_to_string(&env, counter),
            String::from_str(&env, "_0"),
            ];   

        let module_id = concat_strings(&env, arr);
        let key = (MODULE_KEY, module_id.clone());
        if env.storage().persistent().has(&key) {
            if let Some(module) = env.storage().persistent().get::<_, CourseModule>(&key) {
                if module.course_id == *course_id {
                    modules_to_delete.push_back(module_id);
                }
            }
        } else {
            break;
        }
        counter += 1;
        if counter > 1000 {
            break;
        }
    }

    for id in modules_to_delete.iter() {
        env.storage().persistent().remove(&(MODULE_KEY, id.clone()));
        env.events().publish((id.clone(),), "module_deleted");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::Course;
    use crate::{CourseRegistry, CourseRegistryClient};
    use soroban_sdk::{Env, String};
    use soroban_sdk::testutils::Address as _;


    #[test]
    #[should_panic(expected = "Unauthorized")]
    fn test_delete_course_unauthorized() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let impostor: Address = Address::generate(&env);

        let new_course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "title"),
            &String::from_str(&env, "description"),
            &1000_u128,
            &Some(String::from_str(&env, "category")),
            &Some(String::from_str(&env, "language")),
            &Some(String::from_str(&env, "thumbnail_url")),
            &None,
            &None,
        );

        client.delete_course(&impostor, &new_course.id.clone());
    }

    #[test]
    #[should_panic(expected = "Unauthorized")]
    fn test_impostor_cannot_delete_course() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let actual_creator: Address = Address::generate(&env);
        let someone_else: Address = Address::generate(&env);

        // Create a course with actual_creator
        let course: Course = client.create_course(
            &actual_creator,
            &String::from_str(&env, "Protected Course"),
            &String::from_str(&env, "This course should only be deletable by its creator"),
            &500_u128,
            &Some(String::from_str(&env, "security")),
            &Some(String::from_str(&env, "english")),
            &None,
            &None,
            &None,
        );

        let retrieved_course = client.get_course(&course.id);
        assert_eq!(retrieved_course.creator, actual_creator);

        client.delete_course(&someone_else, &course.id);
    }

    #[test]
    fn test_delete_course_success() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let new_course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "title"),
            &String::from_str(&env, "description"),
            &1000_u128,
            &Some(String::from_str(&env, "category")),
            &Some(String::from_str(&env, "language")),
            &Some(String::from_str(&env, "thumbnail_url")),
            &None,
            &None,
        );

        assert_eq!(new_course, client.get_course(&new_course.id.clone()));
        assert_eq!(
            new_course.creator,
            client.get_course(&new_course.id.clone()).creator
        );

        client.delete_course(&creator, &new_course.id.clone());

        let exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(COURSE_KEY, new_course.id.clone()))
        });
        assert!(!exists);
    }

    #[test]
    fn test_delete_course_with_modules() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let new_course: Course = client.create_course(
            &creator,
            &String::from_str(&env, "title"),
            &String::from_str(&env, "description"),
            &1000_u128,
            &Some(String::from_str(&env, "category")),
            &Some(String::from_str(&env, "language")),
            &Some(String::from_str(&env, "thumbnail_url")),
            &None,
            &None,
        );

        let module = client.add_module(&new_course.id, &0, &String::from_str(&env, "Module Title"));

        let module_exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(MODULE_KEY, module.id.clone()))
        });
        assert!(module_exists);

        client.delete_course(&creator, &new_course.id.clone());

        let course_exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(COURSE_KEY, new_course.id.clone()))
        });
        assert!(!course_exists);

        let module_exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(MODULE_KEY, module.id.clone()))
        });
        assert!(!module_exists);
    }

    #[test]
    #[should_panic(expected = "Course not found")]
    fn test_delete_course_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);
        let fake_id = String::from_str(&env, "not_found");

        client.delete_course(&creator, &fake_id);
    }

    #[test]
    fn test_delete_course_preserves_others() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(CourseRegistry, {});
        let client = CourseRegistryClient::new(&env, &contract_id);

        let creator: Address = Address::generate(&env);

        let course1: Course = client.create_course(
            &creator,
            &String::from_str(&env, "title1"),
            &String::from_str(&env, "description1"),
            &1000_u128,
            &Some(String::from_str(&env, "category1")),
            &Some(String::from_str(&env, "language1")),
            &Some(String::from_str(&env, "thumbnail_url1")),
            &None,
            &None,
        );

        let course2: Course = client.create_course(
            &creator,
            &String::from_str(&env, "title2"),
            &String::from_str(&env, "description2"),
            &1000_u128,
            &Some(String::from_str(&env, "category2")),
            &Some(String::from_str(&env, "language2")),
            &Some(String::from_str(&env, "thumbnail_url2")),
            &None,
            &None,
        );

        client.delete_course(&creator, &course1.id.clone());

        let course1_exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(COURSE_KEY, course1.id.clone()))
        });
        assert!(!course1_exists);

        let course2_exists: bool = env.as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .has(&(COURSE_KEY, course2.id.clone()))
        });
        assert!(course2_exists);
    }
}

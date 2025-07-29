use soroban_sdk::{Env, Symbol, String, symbol_short};

use crate::schema::CourseUsers;

const COURSES_KEY: Symbol = symbol_short!("courses");

pub fn course_access_list_course_access(env: Env, course_id: String) -> CourseUsers {

    let key: (Symbol, String) = (COURSES_KEY, course_id.clone());
    
    let addresses: CourseUsers = env
        .storage()
        .persistent()
        .get(&(key)).expect("User Courses Not Found");
    
    addresses
}

#[cfg(test)]
mod test {
    use soroban_sdk::{testutils::{Address as _}, Env, String, Address, vec, symbol_short, Symbol};

    use crate::{course_access_list_course_access, CourseAccessContract, CourseUsers};

    const COURSES_KEY: Symbol = symbol_short!("courses");

    #[test]
    fn test() {
        let env: Env = Env::default();
        
        let contract_id: Address = env.register(CourseAccessContract, {});

        let course_id: String = String::from_str(&env, "test_course_123");
        let user1: Address = Address::generate(&env);
        let user2: Address = Address::generate(&env);

        let addresses: soroban_sdk::Vec<Address>= vec![&env, user1, user2];

        let course_users: CourseUsers = CourseUsers { users: addresses, course: course_id.clone() };
        
        // Set up initial course data and perform test within contract context
        env.clone().as_contract(&contract_id, || {
            env.storage().persistent().set(&(COURSES_KEY, course_id.clone()), &course_users);
            let result: CourseUsers = course_access_list_course_access(env, course_id.clone());
            assert_eq!(result, course_users);
        });
    }
}
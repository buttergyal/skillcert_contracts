use soroban_sdk::{symbol_short, Address, Env, String, Symbol};

use crate::schema::UserCourses;

const USER_KEY: Symbol = symbol_short!("user");

pub fn course_access_list_user_courses(env: Env, user: Address) -> UserCourses {
    let username: String = user.to_string();
    let key: (Symbol, String) = (USER_KEY, username.clone());

    let courses: UserCourses = env
        .storage()
        .persistent()
        .get(&(key))
        .expect("User Courses Not Found");

    courses
}

#[cfg(test)]
mod test {
    use soroban_sdk::{symbol_short, testutils::Address as _, vec, Address, Env, String, Symbol};

    use crate::{course_access_list_user_courses, CourseAccessContract, UserCourses};

    const USER_KEY: Symbol = symbol_short!("user");

    #[test]
    fn test() {
        let env: Env = Env::default();

        let contract_id: Address = env.register(CourseAccessContract, {});

        let course_id: String = String::from_str(&env, "test_course_123");
        let user: Address = Address::generate(&env);

        let courses: soroban_sdk::Vec<String> = vec![&env, course_id];

        let user_courses: UserCourses = UserCourses {
            user: user.clone(),
            courses: courses,
        };

        // Set up initial course data and perform test within contract context
        env.clone().as_contract(&contract_id, || {
            env.storage()
                .persistent()
                .set(&(USER_KEY, user.to_string().clone()), &user_courses);
            let result: UserCourses = course_access_list_user_courses(env, user.clone());
            assert_eq!(result, user_courses);
        });
    }
}

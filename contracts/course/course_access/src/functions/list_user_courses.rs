use soroban_sdk::{Address, Env, Vec};

use crate::schema::{UserCourses, DataKey};

pub fn course_access_list_user_courses(env: Env, user: Address) -> UserCourses {
    let key = DataKey::UserCourses(user.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or(UserCourses {
            user,
            courses: Vec::new(&env),
        })
}

// #[cfg(test)]
// mod test {
//     use soroban_sdk::{symbol_short, testutils::Address as _, vec, Address, Env, String, Symbol};

//     use crate::{course_access_list_user_courses, CourseAccessContract, UserCourses};

//     const USER_KEY: Symbol = symbol_short!("user");

//     #[test]
//     fn test() {
//         let env: Env = Env::default();

//         let contract_id: Address = env.register(CourseAccessContract, {});

//         let course_id: String = String::from_str(&env, "test_course_123");
//         let user: Address = Address::generate(&env);

//         let courses: soroban_sdk::Vec<String> = vec![&env, course_id];

//         let user_courses: UserCourses = UserCourses {
//             user: user.clone(),
//             courses: courses,
//         };

//         // Set up initial course data and perform test within contract context
//         env.clone().as_contract(&contract_id, || {
//             env.storage()
//                 .persistent()
//                 .set(&(USER_KEY, user.to_string().clone()), &user_courses);
//             let result: UserCourses = course_access_list_user_courses(env, user.clone());
//             assert_eq!(result, user_courses);
//         });
//     }
// }

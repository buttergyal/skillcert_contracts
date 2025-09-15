// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Address, Env, Vec};

use crate::schema::{DataKey, UserCourses};

/// List all courses that a specific user has access to.
///
/// This function retrieves all courses that the specified user is enrolled in
/// or has been granted access to. If the user has no courses, it returns
/// an empty UserCourses struct.
///
/// # Arguments
///
/// * `env` - The Soroban environment
/// * `user` - The address of the user to query courses for
///
/// # Returns
///
/// Returns a `UserCourses` struct containing the user's address and a list
/// of course IDs they have access to. If no courses are found, returns
/// an empty list.
pub fn list_user_courses(env: Env, user: Address) -> UserCourses {
    let key = DataKey::UserCourses(user.clone());
    env.storage().persistent().get(&key).unwrap_or(UserCourses {
        user,
        courses: Vec::new(&env),
    })
}

// #[cfg(test)]
// mod test {
//     use soroban_sdk::{symbol_short, testutils::Address as _, vec, Address, Env, String, Symbol};

//     use crate::{list_user_courses, CourseAccessContract, UserCourses};

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
//             let result: UserCourses = list_user_courses(env, user.clone());
//             assert_eq!(result, user_courses);
//         });
//     }
// }

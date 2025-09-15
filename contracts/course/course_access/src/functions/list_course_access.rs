// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Env, String, Vec};

use crate::schema::{CourseUsers, DataKey};

/// List all users who have access to a specific course.
///
/// This function retrieves all users who have been granted access to the
/// specified course. If no users have access, it returns an empty CourseUsers struct.
///
/// # Arguments
///
/// * `env` - The Soroban environment
/// * `course_id` - The unique identifier of the course to query users for
///
/// # Returns
///
/// Returns a `CourseUsers` struct containing the course ID and a list
/// of user addresses who have access to the course. If no users are found,
/// returns an empty list.
pub fn list_course_access(env: Env, course_id: String) -> CourseUsers {
    let key = DataKey::CourseUsers(course_id.clone());
    env.storage().persistent().get(&key).unwrap_or(CourseUsers {
        course: course_id,
        users: Vec::new(&env),
    })
}

// #[cfg(test)]
// mod test {
//     use soroban_sdk::{symbol_short, testutils::Address as _, vec, Address, Env, String, Symbol};

//     use crate::{list_course_access, CourseAccessContract, CourseUsers};

//     const COURSES_KEY: Symbol = symbol_short!("courses");

//     #[test]
//     fn test() {
//         let env: Env = Env::default();

//         let contract_id: Address = env.register(CourseAccessContract, {});

//         let course_id: String = String::from_str(&env, "test_course_123");
//         let user1: Address = Address::generate(&env);
//         let user2: Address = Address::generate(&env);

//         let addresses: soroban_sdk::Vec<Address> = vec![&env, user1, user2];

//         let course_users: CourseUsers = CourseUsers {
//             users: addresses,
//             course: course_id.clone(),
//         };

//         // Set up initial course data and perform test within contract context
//         env.clone().as_contract(&contract_id, || {
//             env.storage()
//                 .persistent()
//                 .set(&(COURSES_KEY, course_id.clone()), &course_users);
//             let result: CourseUsers = list_course_access(env, course_id.clone());
//             assert_eq!(result, course_users);
//         });
//     }
// }

// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{Address, Env, Vec};

use crate::schema::{DataKey, UserCourses};


pub fn list_user_courses(env: Env, user: Address) -> UserCourses {
    let key: DataKey = DataKey::UserCourses(user.clone());
    let res: UserCourses = env.storage().persistent().get(&key).unwrap_or(UserCourses {
        user: user.clone(),
        courses: Vec::new(&env),
    });

    return res
}

#[cfg(test)]
mod test {
    use crate::schema::DataKey;
    use crate::{CourseAccessContract, UserCourses};
    use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};
    use super::list_user_courses;

    #[test]
    fn test_list_user_courses() {
        let env: Env = Env::default();
        let contract_id: Address = env.register(CourseAccessContract, {});
        let user: Address = Address::generate(&env);
        let key: DataKey = DataKey::UserCourses(user.clone());
        let course_id: String = String::from_str(&env, "test_course_123");
        let courses: soroban_sdk::Vec<String> = vec![&env, course_id];
        let user_courses: UserCourses = UserCourses {
            user: user.clone(),
            courses: courses,
        };
        
        // Set up initial course data and perform test within contract context
        env.clone().as_contract(&contract_id, || {
            env.storage().persistent().set(&key, &user_courses);
            let result: UserCourses = list_user_courses(env, user.clone());
            assert_eq!(result, user_courses);
        });
    }
}

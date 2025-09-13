// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use soroban_sdk::{contracterror, panic_with_error, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    OnlyCreatorCanAddGoals = 1,
    EmptyGoalContent = 2,
    CourseIdNotExist = 3,
    OnlyCreatorCanArchive = 4,
    CourseAlreadyArchived = 5,
    Unauthorized = 6,
    NameRequired = 7,
    EmptyCourseTitle = 8,
    InvalidPrice = 9,
    DuplicateCourseTitle = 10,
    DuplicateCourseId = 11,
    OnlyCreatorCanEditPrereqs = 12,
    PrereqCourseNotFound = 13,
    SelfPrerequisite = 14,
    CircularDependency = 15,
    EmptyCourseId = 16,
    CourseNotFound = 17,
    EmptyNewGoalContent = 18,
    EmptyGoalId = 19,
    GoalCourseMismatch = 20,
    ModuleNotFound = 21,
    EmptyModuleId = 22,
    PrereqNotInList = 23,
}

pub fn handle_error(env: &Env, error: Error) -> ! {
    panic_with_error!(env, error);
}

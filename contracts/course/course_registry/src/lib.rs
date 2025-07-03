
#![no_std]
use soroban_sdk::{contract};
pub mod functions{
    pub mod create_course;
}

pub mod schema;

#[contract]
pub struct CourseRegistry;
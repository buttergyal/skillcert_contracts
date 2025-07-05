
#![no_std]
extern crate alloc;

use soroban_sdk::{contract};
pub mod functions{
    pub mod create_course;
}

pub mod schema;

#[contract]
pub struct CourseRegistry;
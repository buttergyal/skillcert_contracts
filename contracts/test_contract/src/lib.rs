#![no_std]

use soroban_sdk::{contract, contractimpl, Env, String};

#[contract]
pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn hello_world(_env: Env, _name: String) -> String {
        String::from_str(&_env, "Hello from Web3 👋")
    }
}

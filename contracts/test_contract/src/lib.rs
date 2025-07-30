use soroban_sdk::{Env, String, contract, contractimpl};

#[contract]
pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn hello_world(_env: Env) -> String {
        String::from_str(&_env, "Hello from Web3 👋")
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_hello() {
        let env = Env::default();
        let result = HelloContract::hello(env, "Immanuel");
        assert_eq!(result, "Hello, Immanuel!");
    }
}
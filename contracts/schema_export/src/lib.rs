#![no_std]

use soroban_sdk::{contracttype, Env, String, Vec};

/// Helper module for exporting contract schemas
pub mod schema_export {
    use super::*;
    
    /// Export contract metadata for frontend consumption
    #[contracttype]
    pub struct ContractMetadata {
        pub name: String,
        pub version: String,
        pub methods: Vec<MethodInfo>,
    }
    
    #[contracttype]
    pub struct MethodInfo {
        pub name: String,
        pub params: Vec<ParamInfo>,
        pub returns: String,
    }
    
    #[contracttype]
    pub struct ParamInfo {
        pub name: String,
        pub type_name: String,
        pub required: bool,
    }
}
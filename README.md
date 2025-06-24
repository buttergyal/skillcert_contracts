# skillcert_contracts

### 📁 Project Architecture Overview

```txt
.
├── Cargo.toml                  # Root workspace definition
├── .gitignore                 # Global ignore rules for Rust, Soroban, IDEs
└── contracts/                 # Directory for all Web3 smart contracts
    └── course/                # Grouping course contracts as a unified module (organization)
        ├── course_registry/       # Contract for managing course metadata
        │   ├── Cargo.toml         # Local contract config
        │   └── src/
        │       ├── functions/     # One file per granular function
        │       │   ├── create_course.rs
        │       │   ├── get_course.rs
        │       │   └── ...        # (add_module, delete_course, etc.)
        │       ├── schema.rs      # Struct definitions (Course, Module, etc.)
        │       ├── lib.rs         # Entry point with #[contractimpl]
        │       └── test.rs        # Unit tests
        └── course_access/         # Contract for handling user permissions
            ├── Cargo.toml
            └── src/
                ├── functions/     # Grant/revoke/list access logic
                ├── schema.rs
                ├── lib.rs
                └── test.rs
```
> As the project grows, new modules, contracts and functions will arise in the architecture.

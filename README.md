# skillcert_contracts

### Description

Skillcert is platform for issuing NFT-based digital certificates on the Stellar blockchain, ensuring authenticity, traceability, and instant verification. It also features an educational marketplace for courses with automated validated certification.

We are currently in the integration phase between the Web3 logic and the frontend application. The majority of the smart contract functions and blockchain interaction logic have already been implemented and tested on the Web3 side. On the frontend, all core views and components have been developed, and the UI/UX structure is stable. At this stage, we are focusing on wiring together the frontend interfaces with the Web3 functionalities—enabling user actions in the UI to trigger the corresponding smart contract calls and ensuring data from the blockchain is properly rendered on the client side. This step is essential to achieve full system functionality and user interaction flow.

### 📌 Roadmap

The project roadmap is available in Notion:  
🔗 [View Roadmap in Notion](https://www.notion.so/Skillcert-240bfdf2613c805898c9c91f0990600e)


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


#### Contract Deployment

To deploy the contracts to the local network (Course Access, Course Registry, User Management), use the provided script:


```bash

./scripts/deploy_contracts.sh
```

This will deploy all contracts and save their addresses in a `contract.json` file.

#### Example Contract Interactions

Here are some example interactions with the deployed contracts:

1. Create a Course:
```bash
stellar contract invoke \
  --id <course_registry_contract_id> \
  --source-account default \
  --network local \
  -- create_course \
  --creator <your_public_key> \
  --title "Introduction to Blockchain" \
  --description "Learn blockchain basics" \
  --price 1000 \
  --category null \
  --language null \
  --thumbnail_url null
```

2. Grant Course Access:
```bash
stellar contract invoke \
  --id <course_access_contract_id> \
  --source-account default \
  --network local \
  -- grant_access \
  --course_id '{"string": "1"}' \
  --user <student_public_key>
```

3. List User's Courses:
```bash
stellar contract invoke \
  --id <course_access_contract_id> \
  --source-account default \
  --network local \
  -- list_user_courses \
  --user <user_public_key>
```

4. View Course Details:
```bash
stellar contract invoke \
  --id <course_registry_contract_id> \
  --source-account default \
  --network local \
  -- get_course \
  --course_id '{"string": "1"}'
```

For more examples, check out the `scripts/invoke_examples.sh` file.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

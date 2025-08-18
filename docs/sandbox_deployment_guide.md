# SkillCert Contracts Documentation
This documentation explains how to build, deploy, and interact with the CourseAccess and CourseRegistry Stellar smart contracts in the skillcert_contracts repository using the provided shell scripts: build.sh, deploy_contract.sh, and invoke_example.sh. These scripts set up a local Stellar network, build the contracts, deploy them to the network, and invoke example contract functions to create and retrieve courses.
Prerequisites
Before running the scripts, ensure the following tools are installed:

Docker: Required to run the Stellar local network.
Install Docker Desktop on macOS/Windows or Docker on Linux.
Verify with: docker --version


Stellar CLI: Used to interact with the Stellar network and contracts.
Install: cargo install stellar-cli
Verify with: stellar --version


Rust and Cargo: Needed to build the contract WASM files.
Install: Follow instructions at rustup.rs
Add WASM target: rustup target add wasm32v1-none
Verify with: cargo --version


jq (optional): Simplifies JSON parsing for contract.json.
Install: brew install jq (macOS) or sudo apt-get install jq (Ubuntu)

Ensure you have cloned the repository:
git clone https://github.com/SkillCert/skillcert_contracts/
cd skillcert_contracts

## Scripts Overview
The repository includes three shell scripts to manage the contract lifecycle:

build.sh: Sets up the Stellar local network, funds the default account, and builds the contract WASM files.

deploy_contract.sh: Deploys the CourseAccess and CourseRegistry contracts to the local network and saves their IDs to contract.json.

invoke_example.sh: Invokes example functions on the CourseRegistry contract to create courses and retrieve course data.

### Step-by-Step Instructions
#### 1. Set Up the Environment (build.sh)
The build.sh script starts Docker, sets up a local Stellar network, funds the default account, and builds the contract WASM files.

Run the script:
```
chmod +x scripts/build_contracts.sh
./scripts/build_contracts.sh
```

What it does:

Checks if the Docker daemon is running and starts Docker Desktop (macOS-specific) if needed.
Starts the Stellar container (stellar) using docker start stellar.

Configures the local network with:
RPC URL: http://localhost:8000/soroban/rpc
Passphrase: Standalone Network ; February 2017

Generates and funds the default account using stellar keys generate and stellar keys fund.
Builds the contracts with cargo build --target wasm32v1-none --release, producing:

target/wasm32v1-none/release/course_access.wasm

target/wasm32v1-none/release/course_registry.wasm


#### Troubleshooting:

Docker Error: If you see Cannot connect to the Docker daemon, ensure Docker is installed and running. 
Container Missing: If docker start stellar fails, create the container as shown above.
Build Failure: Ensure Rust is installed and the WASM target is added (rustup target add wasm32v1-none).

#### 2. Deploy Contracts (deploy_contract.sh)
The deploy_contract.sh script deploys the CourseAccess and CourseRegistry contracts and saves their IDs to contract.json.
Run the script:

``` rust
chmod +x scripts/deploy_contracts.sh

./scripts/deploy_contracts.sh
```

What it does:

Deploys course_access.wasm and course_registry.wasm using stellar contract deploy.

Creates contract.json with the format:
```
{
  "course_access_contract": "<CourseAccess Contract ID>",
  "course_registry_contract": "<CourseRegistry Contract ID>"
}
```

Example contract.json:
```
{
  "course_access_contract": "CBF3FNVSN3EQKOCK7AVWHDUEUPPAOQCFUVO3RR7ABSAVG3CQH4HVDYRD",
  "course_registry_contract": "CAFNV3LJDUGATSPS6UJFGDIJLFZ2DDN3RIWKLSRGCIMXEWUZKDLOQM43"
}
```


#### Troubleshooting:

Missing WASM Files: Ensure build.sh was run successfully and the WASM files exist in target/wasm32v1-none/release/.
Funding Error: If deployment fails due to insufficient funds, re-fund the default account:
```
stellar keys fund default --network local
```


#### 3. Invoke Contract Functions (invoke_example.sh)
The invoke_example.sh script invokes example functions on the CourseRegistry contract to create and retrieve courses.
Run the script:

```
chmod +x scripts/invoke_examples.sh
./scripts/invoke_examples.sh
```
What it does:

Reads the course_registry_contract ID from contract.json.
Retrieves the default account’s public key for the --creator argument using stellar keys public-key default.

Create First Course:
Function: create_course
Parameters: --title "title" --description "A description" --price 1000 --category null --language null --thumbnail_url null
Creates a course with the default account as the creator.


Create Second Course:
Function: create_course
Parameters: --title "A new title" --description "A new descriptiom" --price 2000 --category null --language null --thumbnail_url null
Creates another course with different title and description.


Get Courses by Instructor:
Function: get_courses_by_instructor
Parameter: --instructor <default account address>
Retrieves all courses created by the default account.


## Example Workflow

Build the contracts:
./build.sh


Deploy the contracts:
./deploy_contract.sh


Check contract IDs:
cat contract.json


Invoke contract functions:
./invoke_example.sh


### Manual invocation (e.g., to create another course):
stellar contract invoke \
  --id $(jq -r '.course_registry_contract' contract.json) \
  --source-account default \
  --network local \
  -- create_course \
  --creator $(stellar keys public-key default) \
  --title "Advanced Course" \
  --description "Advanced blockchain" \
  --price 2000 \
  --category null \
  --language null \
  --thumbnail_url null


### Additional Notes

The scripts assume you’re in the skillcert_contracts repository root directory.
Ensure the default account is funded before invoking transactions. Use stellar keys fund default --network local if needed.

For further assistance, check the Stellar CLI documentation (stellar --help) or the Soroban documentation at soroban.stellar.org.
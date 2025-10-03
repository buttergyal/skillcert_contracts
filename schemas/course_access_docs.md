Env Meta: AAAAAAAAABYAAAAA
 • Protocol Version: 22

Contract Meta:
 • rsver: 1.90.0-nightly
 • rssdkver: 22.0.8#f46e9e0610213bbb72285566f9dd960ff96d03d8

Contract Spec:
 • Error: Error
     Cases:
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(UserAlreadyHasAccess),
            value: 1,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(UserNoAccessCourse),
            value: 2,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(Unauthorized),
            value: 3,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(NameRequired),
            value: 4,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(EmailRequired),
            value: 5,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(CountryRequired),
            value: 6,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(InvalidCourseId),
            value: 7,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(InvalidUser),
            value: 8,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(EmptyCourseId),
            value: 9,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(InvalidTransferData),
            value: 10,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(SameUserTransfer),
            value: 11,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(Initialized),
            value: 12,
        }

 • Error: VersioningError
     Docs: Errors that can occur during contract versioning operations
     Cases:
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(Invalid version format),
            name: StringM(InvalidVersion),
            value: 1,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(Version not found in history),
            name: StringM(VersionNotFound),
            value: 2,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(Migration not compatible),
            name: StringM(MigrationNotCompatible),
            value: 3,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(Migration already completed),
            name: StringM(MigrationAlreadyCompleted),
            value: 4,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(Unauthorized migration attempt),
            name: StringM(UnauthorizedMigration),
            value: 5,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(Migration failed),
            name: StringM(MigrationFailed),
            value: 6,
        }

 • Struct: CourseAccess
     Docs: Represents access permission for a user to a specific course.
          
          This struct defines the relationship between a user and a course
          they have been granted access to.
     Fields:
      • course_id: String
        StringM(The unique identifier of the course)
      • user: Address
        StringM(The address of the user who has access)

 • Struct: UserCourses
     Docs: Contains all courses that a specific user has access to.
          
          This struct is used to efficiently query and return all courses
          accessible by a particular user.
     Fields:
      • courses: Vec(
            ScSpecTypeVec {
                element_type: String,
            },
        )
        StringM(List of course IDs the user has access to)
      • user: Address
        StringM(The address of the user)

 • Union: DataKey
     Docs: Storage keys for different data types in the contract.
          
          This enum defines the various keys used to store and retrieve
          data from the contract's persistent storage.
     Cases:
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing course access: (course_id, user) -> CourseAccess),
                name: StringM(CourseAccess),
                type_: VecM(
                    [
                        String,
                        Address,
                    ],
                ),
            },
        )
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing user profile: user -> UserProfile),
                name: StringM(UserProfile),
                type_: VecM(
                    [
                        Address,
                    ],
                ),
            },
        )
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing courses per user: user -> UserCourses),
                name: StringM(UserCourses),
                type_: VecM(
                    [
                        Address,
                    ],
                ),
            },
        )
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing users per course: course_id -> CourseUsers),
                name: StringM(CourseUsers),
                type_: VecM(
                    [
                        String,
                    ],
                ),
            },
        )

 • Struct: UserProfile
     Docs: Represents a user's profile information.
          
          This struct contains all the personal and professional information
          that users can store on-chain as part of their profile.
     Fields:
      • country: String
        StringM(The user's country of residence)
      • email: String
        StringM(The user's email address)
      • goals: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(Optional learning goals or objectives)
      • name: String
        StringM(The user's full name)
      • profession: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(Optional profession or job title)

 • Struct: CourseUsers
     Docs: Contains all users who have access to a specific course.
          
          This struct is used to efficiently query and return all users
          who have been granted access to a particular course.
     Fields:
      • course: String
        StringM(The unique identifier of the course)
      • users: Vec(
            ScSpecTypeVec {
                element_type: Address,
            },
        )
        StringM(List of user addresses who have access to the course)

 • Function: initialize
     Docs: One-time constructor to set owner and config addresses.
           
           Initializes the contract with the necessary external contract addresses.
           This function can only be called once during contract deployment.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `caller` - The address of the contract deployer/owner
           * `user_mgmt_addr` - Address of the user management contract
           * `course_registry_addr` - Address of the course registry contract
           
           # Panics
           
           * Fails if the contract has already been initialized
           * If any of the provided addresses are invalid
           
           # Examples
           
           ```rust
           // Initialize contract during deployment
           contract.initialize(
           env.clone(),
           deployer_address,
           user_mgmt_contract_address,
           course_registry_contract_address
           );
           ```
           
           # Edge Cases
           
           * **Double initialization**: Will panic if called more than once
           * **Invalid addresses**: Contract addresses must be valid
           * **Deployment only**: Should only be called during contract deployment
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user_mgmt_addr),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(course_registry_addr),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [],
     )

 • Function: grant_access
     Docs: Grant access to a specific user for a given course.
           
           Allows a user to access a specific course. Only authorized users
           (course creators or admins) can grant access.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `course_id` - The unique identifier of the course
           * `user` - The address of the user to grant access to
           
           # Panics
           
           * If course doesn't exist
           * If caller is not authorized (not course creator or admin)
           * If user already has access
           
           # Examples
           
           ```rust
           // Course creator granting access
           contract.grant_access(
           env.clone(),
           "course_123".try_into().unwrap(),
           student_address
           );
           
           // Admin granting access
           contract.grant_access(
           env.clone(),
           "course_456".try_into().unwrap(),
           student_address
           );
           ```
           
           # Edge Cases
           
           * **Already has access**: Will panic if user already has access
           * **Non-existent course**: Will panic if course doesn't exist
           * **Permission denied**: Only course creators and admins can grant access
           * **User validation**: User address must be valid
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(course_id),
                 type_: String,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [],
     )

 • Function: revoke_access
     Docs: Revoke access for a specific user from a course.
           
           Removes a user's access to a specific course. Only authorized users
           (course creators or admins) can revoke access.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `course_id` - The unique identifier of the course
           * `user` - The address of the user to revoke access from
           
           # Returns
           
           Returns `true` if access was successfully revoked, `false` otherwise.
           
           # Panics
           
           * If course doesn't exist
           * If caller is not authorized (not course creator or admin)
           
           # Examples
           
           ```rust
           // Revoke access from a user
           let success = contract.revoke_access(
           env.clone(),
           "course_123".try_into().unwrap(),
           student_address
           );
           
           if success {
           println!("Access revoked successfully");
           } else {
           println!("User didn't have access");
           }
           ```
           
           # Edge Cases
           
           * **No access to revoke**: Returns `false` if user didn't have access
           * **Non-existent course**: Will panic if course doesn't exist
           * **Permission denied**: Only course creators and admins can revoke access
           * **Idempotent**: Safe to call multiple
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(course_id),
                 type_: String,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [
             Bool,
         ],
     )

 • Function: save_user_profile
     Docs: Save or update a user's profile on-chain.
           
           Stores user profile information in the contract storage.
           This includes personal and professional information.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `name` - The user's full name
           * `email` - The user's email address
           * `profession` - Optional profession/job title
           * `goals` - Optional learning goals or objectives
           * `country` - The user's country of residence
           
           # Panics
           
           * If name, email, or country are empty
           * If email format is invalid
           
           # Examples
           
           ```rust
           // Save user profile
           contract.save_user_profile(
           env.clone(),
           "John Doe".try_into().unwrap(),
           "john@example.com".try_into().unwrap(),
           Some("Software Developer".try_into().unwrap()),
           Some("Learn Rust programming".try_into().unwrap()),
           "US".try_into().unwrap()
           );
           
           // Save minimal profile
           contract.save_user_profile(
           env.clone(),
           "Jane Smith".try_into().unwrap(),
           "jane@example.com".try_into().unwrap(),
           None,
           None,
           "CA".try_into().unwrap()
           );
           ```
           
           # Edge Cases
           
           * **Empty required fields**: Name, email, and coun
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(name),
                 type_: String,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(email),
                 type_: String,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(profession),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: String,
                     },
                 ),
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(goals),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: String,
                     },
                 ),
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(country),
                 type_: String,
             },
         ],
     )
     Output: VecM(
         [],
     )

 • Function: list_user_courses
     Docs: List all courses a user has access to.
           
           Retrieves all courses that the specified user is enrolled in
           or has been granted access to.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `user` - The address of the user to query
           
           # Returns
           
           Returns a `UserCourses` struct containing the list of accessible courses.
           
           # Examples
           
           ```rust
           // Get user's accessible courses
           let user_courses = contract.list_user_courses(env.clone(), user_address);
           
           for course_id in user_courses.course_ids {
           println!("User has access to course: {}", course_id);
           }
           ```
           
           # Edge Cases
           
           * **No access**: Returns empty list if user has no course access
           * **Non-existent user**: Returns empty list for non-existent users
           * **Public access**: Anyone can query user courses
           * **Revoked courses**: Only includes currently accessible courses
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [
             Udt(
                 ScSpecTypeUdt {
                     name: StringM(UserCourses),
                 },
             ),
         ],
     )

 • Function: list_course_access
     Docs: List all users who have access to a course.
           
           Retrieves all users who have been granted access to the specified course.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `course_id` - The unique identifier of the course
           
           # Returns
           
           Returns a `CourseUsers` struct containing the list of users with access.
           
           # Examples
           
           ```rust
           // Get all users with access to a course
           let course_users = contract.list_course_access(env.clone(), "course_123".try_into().unwrap());
           
           for user in course_users.users {
           println!("User with access: {}", user);
           }
           ```
           
           # Edge Cases
           
           * **No users**: Returns empty list if no users have access
           * **Non-existent course**: Returns empty list for non-existent courses
           * **Public access**: Anyone can query course access
           * **Real-time data**: Always returns current access status
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(course_id),
                 type_: String,
             },
         ],
     )
     Output: VecM(
         [
             Udt(
                 ScSpecTypeUdt {
                     name: StringM(CourseUsers),
                 },
             ),
         ],
     )

 • Function: revoke_all_access
     Docs: Revoke all user access for a course.
           
           Removes access for all users from the specified course.
           Only admin or course creator is allowed to perform this operation.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `user` - The address of the user requesting the operation
           * `course_id` - The unique identifier of the course
           
           # Returns
           
           Returns the number of users affected by the revocation and emits an event.
           
           # Panics
           
           * If course doesn't exist
           * If caller is not authorized (not course creator or admin)
           
           # Examples
           
           ```rust
           // Revoke all access for a course
           let affected_users = contract.revoke_all_access(
           env.clone(),
           admin_address,
           "course_123".try_into().unwrap()
           );
           
           println!("Revoked access for {} users", affected_users);
           ```
           
           # Edge Cases
           
           * **No users**: Returns 0 if no users had access
           * **Non-existent course**: Will panic if course doesn't exist
           * **Permission denied**: Only course creators and admins can perform this
           * **Bulk operation**: Efficiently removes all access in one transaction
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(course_id),
                 type_: String,
             },
         ],
     )
     Output: VecM(
         [
             U32,
         ],
     )

 • Function: set_config
     Docs: Configure external contract addresses used for auth checks.
           
           Updates the addresses of external contracts that this contract
           depends on for authentication and authorization checks.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `caller` - The address of the user making the configuration change
           * `user_mgmt_addr` - Address of the user management contract
           * `course_registry_addr` - Address of the course registry contract
           
           # Panics
           
           * If caller is not the contract owner
           * If any of the provided addresses are invalid
           
           # Storage
           
           Stores the addresses in instance storage keys: ("user_mgmt_addr",) and ("course_registry_addr",)
           
           # Examples
           
           ```rust
           // Update contract addresses
           contract.set_config(
           env.clone(),
           contract_owner_address,
           new_user_mgmt_address,
           new_course_registry_address
           );
           ```
           
           # Edge Cases
           
           * **Owner only**: Only contract owner can update addresses
           * **Invalid addresses**: Will panic if addresses are invalid
           * **Runtime updates**: Can be called after contract deployment
           * **Immediate effect**: Change
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user_mgmt_addr),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(course_registry_addr),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [],
     )

 • Function: get_contract_version
     Docs: Get the current contract version
           
           Returns the semantic version of the current contract deployment.
           This is useful for tracking contract upgrades and compatibility.
           
           # Arguments
           * `_env` - The Soroban environment (unused)
           
           # Returns
           * `String` - The current contract version
     Inputs: VecM(
         [],
     )
     Output: VecM(
         [
             String,
         ],
     )

 • Function: get_version_history
     Docs: Get contract version history
           
           Returns a list of all versions that have been deployed for this contract.
           This helps track the evolution of the contract over time.
           
           # Arguments
           * `env` - The Soroban environment
           
           # Returns
           * `Vec<String>` - Vector of version strings in chronological order
     Inputs: VecM(
         [],
     )
     Output: VecM(
         [
             Vec(
                 ScSpecTypeVec {
                     element_type: String,
                 },
             ),
         ],
     )

 • Function: is_version_compatible
     Docs: Check compatibility between contract versions
           
           Determines if data from one version can be safely used with another version.
           This is crucial for migration processes and backward compatibility.
           
           # Arguments
           * `env` - The Soroban environment
           * `from_version` - The source version to check compatibility from
           * `to_version` - The target version to check compatibility to
           
           # Returns
           * `bool` - True if the versions are compatible, false otherwise
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(from_version),
                 type_: String,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(to_version),
                 type_: String,
             },
         ],
     )
     Output: VecM(
         [
             Bool,
         ],
     )

 • Function: migrate_access_data
     Docs: Migrate access data between contract versions
           
           Performs data migration from one contract version to another.
           This function handles the transformation of course access data structures
           when upgrading contract versions.
           
           # Arguments
           * `env` - The Soroban environment
           * `caller` - The address performing the migration (must be admin)
           * `from_version` - The source version to migrate from
           * `to_version` - The target version to migrate to
           
           # Returns
           * `bool` - True if migration was successful, false otherwise
           
           # Events
           Emits a migration event upon successful completion
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(from_version),
                 type_: String,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(to_version),
                 type_: String,
             },
         ],
     )
     Output: VecM(
         [
             Bool,
         ],
     )

 • Function: get_migration_status
     Docs: Get migration status for the current contract
           
           Returns information about the current migration status and any
           pending migrations that need to be completed.
           
           # Arguments
           * `env` - The Soroban environment
           
           # Returns
           * `String` - Migration status information
     Inputs: VecM(
         [],
     )
     Output: VecM(
         [
             String,
         ],
     )

 • Function: transfer_course
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(course_id),
                 type_: String,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(from),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(to),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [],
     )



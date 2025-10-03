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
            name: StringM(AlreadInitialized),
            value: 1,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(InvalidMaxPageSize),
            value: 2,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(SystemNotInitialized),
            value: 3,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(AccessDenied),
            value: 4,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(SuperAdminNotRegular),
            value: 5,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(OperationFailed),
            value: 6,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(MaxAdminsReached),
            value: 7,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(CannotRemoveSuperAdmin),
            value: 8,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(UserProfileExists),
            value: 9,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(NameRequired),
            value: 10,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(EmailRequired),
            value: 11,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(CountryRequired),
            value: 12,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(InvalidEmailFormat),
            value: 15,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(EmailAlreadyExists),
            value: 16,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(InvalidField),
            value: 17,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(InvalidProfilePicURL),
            value: 19,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(UserNotFound),
            value: 20,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(UserProfileNotFound),
            value: 21,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(InactiveUser),
            value: 22,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(PageParamTooLarge),
            value: 23,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(InvalidTitleLength),
            value: 24,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(PasswordMismatch),
            value: 25,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(RateLimitExceeded),
            value: 26,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(RateLimitNotConfigured),
            value: 27,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(PasswordTooShort),
            value: 28,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(PasswordTooLong),
            value: 29,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(PasswordMissingUppercase),
            value: 30,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(PasswordMissingLowercase),
            value: 31,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(PasswordMissingDigit),
            value: 32,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(PasswordMissingSpecialChar),
            value: 33,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(RequiredFieldMissing),
            value: 34,
        }
      • ScSpecUdtErrorEnumCaseV0 {
            doc: StringM(),
            name: StringM(Unauthorized),
            value: 35,
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

 • Struct: UserProfile
     Fields:
      • contact_email: String
        StringM(User's contact email address (required, must be unique))
      • country: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's country of residence (optional))
      • full_name: String
        StringM(User's full name (required))
      • profession: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's profession or job title (optional))
      • profile_picture_url: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's profile picture URL (optional))
      • purpose: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's learning goals or purpose (optional))

 • Union: DataKey
     Docs: Data keys for contract storage
          
          Currently includes only UserProfile keyed by user Address
     Cases:
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(),
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
                doc: StringM(),
                name: StringM(EmailIndex),
                type_: VecM(
                    [
                        String,
                    ],
                ),
            },
        )

 • Struct: UserProfile
     Docs: User profile information matching UI definition.
          
          This struct contains user profile data with required and optional fields
          as defined by the user interface requirements.
     Fields:
      • contact_email: String
        StringM(User's contact email address (required, must be unique))
      • country: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's country of residence (optional))
      • full_name: String
        StringM(User's full name (required))
      • profession: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's profession or job title (optional))
      • profile_picture_url: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's profile picture URL (optional))
      • purpose: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's learning goals or purpose (optional))

 • Struct: ProfileUpdateParams
     Docs: Struct for profile update parameters
          Only includes fields that can be updated
     Fields:
      • country: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's country of residence)
      • full_name: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's full name (optional update))
      • profession: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's profession or job title)
      • profile_picture_url: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's profile picture URL)
      • purpose: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's learning goals or purpose)

 • Union: UserRole
     Docs: User roles in the SkillCert platform.
          
          Defines the different types of users and their permission levels.
     Cases:
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Regular platform user who can enroll in courses),
                name: StringM(Student),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(User who can create and manage courses),
                name: StringM(Instructor),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Platform administrator with elevated privileges),
                name: StringM(Admin),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Super administrator with full system access),
                name: StringM(SuperAdmin),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Content moderator with course content permissions),
                name: StringM(Moderator),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Support staff with user assistance permissions),
                name: StringM(Support),
            },
        )

 • Union: Permission
     Docs: Granular permissions for RBAC system.
          
          Defines specific actions that can be granted or denied to users.
     Cases:
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can view user profiles),
                name: StringM(ViewUsers),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can edit user profiles (own or others)),
                name: StringM(EditUsers),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can delete/deactivate users),
                name: StringM(DeleteUsers),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can create new user accounts),
                name: StringM(CreateUsers),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can view course details),
                name: StringM(ViewCourses),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can create new courses),
                name: StringM(CreateCourses),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can edit course content),
                name: StringM(EditCourses),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can delete courses),
                name: StringM(DeleteCourses),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can manage course access (grant/revoke)),
                name: StringM(ManageCourseAccess),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can manage system configuration),
                name: StringM(ManageSystem),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can manage admin roles),
                name: StringM(ManageAdmins),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can view system analytics),
                name: StringM(ViewAnalytics),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can moderate content),
                name: StringM(ModerateContent),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can provide user support),
                name: StringM(ProvideSupport),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Can view support tickets),
                name: StringM(ViewSupport),
            },
        )

 • Struct: RolePermissions
     Docs: Role-based permissions mapping.
          
          Defines which permissions are granted to each role by default.
     Fields:
      • permissions: Vec(
            ScSpecTypeVec {
                element_type: Udt(
                    ScSpecTypeUdt {
                        name: StringM(Permission),
                    },
                ),
            },
        )
        StringM(List of permissions granted to this role)
      • role: Udt(
            ScSpecTypeUdt {
                name: StringM(UserRole),
            },
        )
        StringM(The role this permission set applies to)

 • Struct: UserPermissions
     Docs: User-specific permission overrides.
          
          Allows granting or revoking specific permissions to individual users.
     Fields:
      • granted_permissions: Vec(
            ScSpecTypeVec {
                element_type: Udt(
                    ScSpecTypeUdt {
                        name: StringM(Permission),
                    },
                ),
            },
        )
        StringM(Additional permissions granted beyond role defaults)
      • revoked_permissions: Vec(
            ScSpecTypeVec {
                element_type: Udt(
                    ScSpecTypeUdt {
                        name: StringM(Permission),
                    },
                ),
            },
        )
        StringM(Permissions explicitly revoked from role defaults)
      • user: Address
        StringM(The user address)

 • Union: UserStatus
     Docs: User account status.
          
          Represents the current state of a user's account.
     Cases:
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(User account is active and functional),
                name: StringM(Active),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(User account is deactivated),
                name: StringM(Inactive),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(User account is temporarily suspended),
                name: StringM(Suspended),
            },
        )

 • Struct: LightProfile
     Docs: Lightweight user profile for listing operations.
          
          Contains essential user information for efficient querying and display in user lists.
     Fields:
      • country: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's country of residence)
      • full_name: String
        StringM(User's full name)
      • profession: Option(
            ScSpecTypeOption {
                value_type: String,
            },
        )
        StringM(User's profession or job title)
      • role: Udt(
            ScSpecTypeUdt {
                name: StringM(UserRole),
            },
        )
        StringM(User's role in the platform)
      • status: Udt(
            ScSpecTypeUdt {
                name: StringM(UserStatus),
            },
        )
        StringM(User's account status)
      • user_address: Address
        StringM(User's blockchain address)

 • Struct: RateLimitConfig
     Docs: Rate limiting configuration for user operations.
          
          Tracks rate limiting settings and current usage for spam protection.
     Fields:
      • max_operations_per_window: U32
        StringM(Maximum operations allowed per window)
      • window_seconds: U64
        StringM(Time window for rate limiting in seconds)

 • Struct: RateLimitData
     Docs: Rate limiting tracking data for a specific address.
          
          Stores the current usage count and window start time for rate limiting.
     Fields:
      • count: U32
        StringM(Current count of operations in this window)
      • window_start: U64
        StringM(Timestamp when the current window started)

 • Struct: AdminConfig
     Docs: Administrative configuration for the user management system.
          
          Contains system-wide settings and administrative information.
     Fields:
      • initialized: Bool
        StringM(Whether the system has been initialized)
      • max_page_size: U32
        StringM(Maximum allowed page size for queries)
      • rate_limit_config: Udt(
            ScSpecTypeUdt {
                name: StringM(RateLimitConfig),
            },
        )
        StringM(Rate limiting configuration for user creation)
      • super_admin: Address
        StringM(Address of the super administrator)
      • total_user_count: U32
        StringM(Total number of registered users)

 • Struct: UserBackupData
     Docs: Backup data structure for user management system.
          
          Contains all user data and system configuration for backup and recovery operations.
     Fields:
      • admin_config: Udt(
            ScSpecTypeUdt {
                name: StringM(AdminConfig),
            },
        )
        StringM(Administrative configuration)
      • admins: Vec(
            ScSpecTypeVec {
                element_type: Address,
            },
        )
        StringM(List of admin addresses)
      • backup_timestamp: U64
        StringM(Backup timestamp)
      • backup_version: String
        StringM(Backup version for compatibility)
      • email_mappings: Map(
            ScSpecTypeMap {
                key_type: String,
                value_type: Address,
            },
        )
        StringM(Email to address mapping for uniqueness)
      • light_profiles: Map(
            ScSpecTypeMap {
                key_type: Address,
                value_type: Udt(
                    ScSpecTypeUdt {
                        name: StringM(LightProfile),
                    },
                ),
            },
        )
        StringM(All lightweight profiles for efficient queries)
      • user_profiles: Map(
            ScSpecTypeMap {
                key_type: Address,
                value_type: Udt(
                    ScSpecTypeUdt {
                        name: StringM(UserProfile),
                    },
                ),
            },
        )
        StringM(All user profiles in the system)
      • users_index: Vec(
            ScSpecTypeVec {
                element_type: Address,
            },
        )
        StringM(List of all registered user addresses)

 • Struct: PaginationParams
     Docs: Pagination parameters for cursor-based pagination.
          
          Used to implement efficient pagination that avoids gas limit issues
          with large datasets by using cursor-based navigation.
     Fields:
      • cursor: Option(
            ScSpecTypeOption {
                value_type: Address,
            },
        )
        StringM(Cursor for pagination (address of the last item from previous page))
      • limit: U32
        StringM(Maximum number of items to return per page)

 • Struct: PaginatedLightProfiles
     Docs: Pagination result with metadata for efficient navigation.
          
          Contains the paginated data along with pagination metadata
          to enable cursor-based navigation.
     Fields:
      • data: Vec(
            ScSpecTypeVec {
                element_type: Udt(
                    ScSpecTypeUdt {
                        name: StringM(LightProfile),
                    },
                ),
            },
        )
        StringM(The paginated data items)
      • has_more: Bool
        StringM(Whether there are more pages available)
      • next_cursor: Option(
            ScSpecTypeOption {
                value_type: Address,
            },
        )
        StringM(Cursor for the next page (None if this is the last page))
      • total_count: Option(
            ScSpecTypeOption {
                value_type: U32,
            },
        )
        StringM(Total count of items matching the filter (optional, may be expensive to compute))

 • Union: DataKey
     Docs: Storage keys for different data types in the user management contract.
          
          This enum defines the various keys used to store and retrieve
          user data from the contract's persistent storage.
     Cases:
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing complete user profiles: user_address -> UserProfile),
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
                doc: StringM(Key for storing admin flags: address -> bool),
                name: StringM(Admin),
                type_: VecM(
                    [
                        Address,
                    ],
                ),
            },
        )
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing lightweight user profiles: user_address -> LightProfile),
                name: StringM(UserProfileLight),
                type_: VecM(
                    [
                        Address,
                    ],
                ),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Key for storing the list of all registered user addresses),
                name: StringM(UsersIndex),
            },
        )
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for email to address mapping to ensure email uniqueness: email -> Address),
                name: StringM(EmailIndex),
                type_: VecM(
                    [
                        String,
                    ],
                ),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Key for storing the list of admin addresses),
                name: StringM(Admins),
            },
        )
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing user role assignments: user_address -> UserRole),
                name: StringM(UserRole),
                type_: VecM(
                    [
                        Address,
                    ],
                ),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Key for storing administrative configuration),
                name: StringM(AdminConfig),
            },
        )
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing rate limiting data per address: address -> RateLimitData),
                name: StringM(RateLimit),
                type_: VecM(
                    [
                        Address,
                    ],
                ),
            },
        )
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing role-based permissions: role -> RolePermissions),
                name: StringM(RolePermissions),
                type_: VecM(
                    [
                        Udt(
                            ScSpecTypeUdt {
                                name: StringM(UserRole),
                            },
                        ),
                    ],
                ),
            },
        )
      • TupleV0(
            ScSpecUdtUnionCaseTupleV0 {
                doc: StringM(Key for storing user-specific permission overrides: user_address -> UserPermissions),
                name: StringM(UserPermissions),
                type_: VecM(
                    [
                        Address,
                    ],
                ),
            },
        )
      • VoidV0(
            ScSpecUdtUnionCaseVoidV0 {
                doc: StringM(Key for storing default role permissions configuration),
                name: StringM(DefaultRolePermissions),
            },
        )

 • Function: get_user_profile
     Docs: Retrieve a user profile for the authenticated user.
           
           This function fetches the complete user profile associated with the provided
           blockchain address. The user must be authenticated; otherwise, the function
           will panic.
           
           ### Arguments
           
           * `env` - The Soroban environment.
           * `user` - The address of the user whose profile is being requested.
           
           ### Returns
           
           Returns the `UserProfile` corresponding to the authenticated user.
           
           ### Panics
           
           * If the user is not authenticated (`require_auth` fails).
           * If the user profile does not exist (`UserNotFound` error).
           
           ### Examples
           
           ```rust
           // Assuming the user is authenticated in the environment
           let profile = contract.get_user_profile(env.clone(), my_address);
           println!("User full name: {}", profile.full_name);
           ```
           
           ### Notes
           
           * Only the user themselves can fetch their profile; there is no admin override
           in this function.
           * If the profile is not found in storage, the function will panic with
           `UserNotFound`.
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
             Result(
                 ScSpecTypeResult {
                     ok_type: Udt(
                         ScSpecTypeUdt {
                             name: StringM(UserProfile),
                         },
                     ),
                     error_type: Error,
                 },
             ),
         ],
     )

 • Function: get_user_by_id
     Docs: Retrieve a user profile by their address.
           
           This function fetches a complete user profile using the user's blockchain address.
           Access may be restricted based on the requester's permissions.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `requester` - The address of the user requesting the profile
           * `user_id` - The address of the user whose profile is being requested
           
           # Returns
           
           Returns the requested `UserProfile`.
           
           # Panics
           
           * If the user profile doesn't exist
           * If the requester doesn't have permission to view the profile
           * If the requester is not the user themselves or an admin
           
           # Examples
           
           ```rust
           // Get your own profile
           let my_profile = contract.get_user_by_id(env.clone(), my_address, my_address);
           
           // Admin getting any user's profile
           let user_profile = contract.get_user_by_id(env.clone(), admin_address, user_address);
           ```
           
           # Edge Cases
           
           * **Non-existent user**: Will panic with appropriate error message
           * **Inactive user**: Returns profile but status will be `UserStatus::Inactive`
           * **Permission denied**:
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(requester),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user_id),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [
             Udt(
                 ScSpecTypeUdt {
                     name: StringM(UserProfile),
                 },
             ),
         ],
     )

 • Function: create_user_profile
     Docs: Create a new user profile
           
           Creates a new user profile using a UserProfile struct.
           Validates mandatory fields (full_name and contact_email) and saves the profile.
           
           # Arguments
           * `env` - Soroban environment
           * `user` - Address of the user whose profile is being created
           * `profile` - UserProfile struct containing all profile data
           
           # Returns
           * `UserProfile` - The created user profile
           
           # Panics
           * If mandatory fields (full_name, contact_email) are missing
           * If user profile already exists
           * If email format is invalid
           * If validation rules are violated
           
           # Events
           Emits a user creation event upon successful creation
           
           # Examples
           
           ```rust
           let profile = UserProfile {
           full_name: "John Doe".try_into().unwrap(),
           contact_email: "john@example.com".try_into().unwrap(),
           role: UserRole::Student,
           status: UserStatus::Active,
           country: Some("US".try_into().unwrap()),
           ..Default::default()
           };
           
           let created_profile = contract.create_user_profile(env, user_address, profile);
           ```
           
           # Edge Cases
           
           * **Duplicate profile**: Will panic if user al
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(profile),
                 type_: Udt(
                     ScSpecTypeUdt {
                         name: StringM(UserProfile),
                     },
                 ),
             },
         ],
     )
     Output: VecM(
         [
             Udt(
                 ScSpecTypeUdt {
                     name: StringM(UserProfile),
                 },
             ),
         ],
     )

 • Function: edit_user_profile
     Docs: Edit an existing user profile
           
           Updates an existing user profile with new values for allowed fields.
           Only the user themselves or administrators can perform updates.
           Email and role fields cannot be updated through this function.
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address of the user performing the update
           * `user_id` - Address of the user whose profile is being updated
           * `updates` - ProfileUpdateParams containing fields to update
           
           # Returns
           * `UserProfile` - The updated user profile
           
           # Panics
           * If caller authentication fails
           * If user profile doesn't exist
           * If caller lacks permission to edit
           * If any field validation fails
           * If user is inactive
           
           # Events
           Emits a user update event upon successful profile update
           
           # Examples
           
           ```rust
           let updates = ProfileUpdateParams {
           full_name: Some("Jane Doe".try_into().unwrap()),
           country: Some("CA".try_into().unwrap()),
           bio: Some("Updated bio".try_into().unwrap()),
           ..Default::default()
           };
           
           let updated_profile = contract.edit_user_profile(env, caller_addres
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user_id),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(updates),
                 type_: Udt(
                     ScSpecTypeUdt {
                         name: StringM(ProfileUpdateParams),
                     },
                 ),
             },
         ],
     )
     Output: VecM(
         [
             Udt(
                 ScSpecTypeUdt {
                     name: StringM(UserProfile),
                 },
             ),
         ],
     )

 • Function: is_admin
     Docs: Check if an address has admin privileges.
           
           This function is used by other contracts to verify admin status
           for cross-contract authorization checks.
           
           # Arguments
           
           * `env` - The Soroban environment
           * `who` - The address to check for admin privileges
           
           # Returns
           
           Returns `true` if the address has admin privileges, `false` otherwise.
           
           # Examples
           
           ```rust
           // Check if user is admin
           let is_admin = contract.is_admin(env.clone(), user_address);
           if is_admin {
           // Perform admin operations
           }
           
           // Cross-contract admin check
           let can_perform_action = contract.is_admin(env.clone(), caller_address);
           ```
           
           # Edge Cases
           
           * **System not initialized**: Returns `false` if admin system hasn't been set up
           * **Non-existent user**: Returns `false` for addresses that don't exist
           * **Regular users**: Always returns `false` for non-admin users
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(who),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [
             Bool,
         ],
     )

 • Function: delete_user
     Docs: Delete (deactivate) a user account
           
           Performs a soft delete by marking the user as inactive instead of permanent deletion.
           Only admins or the user themselves can trigger deletion.
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address performing the deletion (must be admin or the user themselves)
           * `user_id` - Address of the user to be deactivated
           
           # Panics
           * If caller authentication fails
           * If user doesn't exist
           * If caller is neither admin nor the user themselves
           * If user is already inactive
           
           # Events
           Emits a user deactivation event upon successful deletion
           
           # Examples
           
           ```rust
           // User deleting their own account
           contract.delete_user(env.clone(), user_address, user_address);
           
           // Admin deleting another user's account
           contract.delete_user(env.clone(), admin_address, user_to_delete);
           ```
           
           # Edge Cases
           
           * **Already inactive**: Will panic if trying to delete an already inactive user
           * **Permission denied**: Non-admin users can only delete their own accounts
           * **Data preservation**: User data is preserved
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(user_id),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [],
     )

 • Function: list_all_users
     Docs: Lists all registered users with pagination and filtering (admin-only)
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address performing the call (must be admin)
           * `page` - Zero-based page index
           * `page_size` - Number of items per page (must be > 0)
           * `role_filter` - Optional role filter
           * `country_filter` - Optional country filter
           * `status_filter` - Optional status filter
           
           # Returns
           * `Vec<LightProfile>` - Filtered and paginated lightweight user profiles
           
           # Panics
           * If caller is not an admin
           * If page_size is 0 or exceeds maximum allowed
           * If system is not initialized
           
           # Examples
           
           ```rust
           // Get first page with 10 users
           let users = contract.list_all_users(
           env.clone(),
           admin_address,
           0,  // page 0
           10, // page size
           None, None, None // no filters
           );
           
           // Filter by role and country
           let students = contract.list_all_users(
           env.clone(),
           admin_address,
           0, 20,
           Some(UserRole::Student),
           Some("US".try_into().unwrap()),
           None
           );
           ```
           
           # Edge Cases
           
           * **Empty results**: Returns empty vector if no users match filter
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(page),
                 type_: U32,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(page_size),
                 type_: U32,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(role_filter),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: Udt(
                             ScSpecTypeUdt {
                                 name: StringM(UserRole),
                             },
                         ),
                     },
                 ),
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(country_filter),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: String,
                     },
                 ),
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(status_filter),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: Udt(
                             ScSpecTypeUdt {
                                 name: StringM(UserStatus),
                             },
                         ),
                     },
                 ),
             },
         ],
     )
     Output: VecM(
         [
             Vec(
                 ScSpecTypeVec {
                     element_type: Udt(
                         ScSpecTypeUdt {
                             name: StringM(LightProfile),
                         },
                     ),
                 },
             ),
         ],
     )

 • Function: list_all_users_advanced
     Docs: Lists all registered users with advanced filtering including text search (admin-only).
           
           This is the new version that supports text search functionality.
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address performing the call (must be admin)
           * `page` - Zero-based page index
           * `page_size` - Number of items per page
           * `role_filter` - Optional role filter
           * `country_filter` - Optional country filter
           * `status_filter` - Optional status filter
           * `search_text` - Optional text search in name and profession
           
           # Returns
           * `Vec<LightProfile>` - Filtered and paginated lightweight user profiles
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(page),
                 type_: U32,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(page_size),
                 type_: U32,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(role_filter),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: Udt(
                             ScSpecTypeUdt {
                                 name: StringM(UserRole),
                             },
                         ),
                     },
                 ),
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(country_filter),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: String,
                     },
                 ),
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(status_filter),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: Udt(
                             ScSpecTypeUdt {
                                 name: StringM(UserStatus),
                             },
                         ),
                     },
                 ),
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(search_text),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: String,
                     },
                 ),
             },
         ],
     )
     Output: VecM(
         [
             Vec(
                 ScSpecTypeVec {
                     element_type: Udt(
                         ScSpecTypeUdt {
                             name: StringM(LightProfile),
                         },
                     ),
                 },
             ),
         ],
     )

 • Function: list_all_users_cursor
     Docs: Lists all registered users with cursor-based pagination and filtering (admin-only)
           
           This function implements efficient cursor-based pagination to avoid gas limit issues
           when dealing with large datasets. It returns a PaginatedResult with metadata for
           efficient navigation.
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address performing the call (must be admin)
           * `pagination` - Pagination parameters including cursor and limit
           * `role_filter` - Optional filter for user role
           * `status_filter` - Optional filter for user status
           
           # Returns
           * `PaginatedLightProfiles` - Paginated results with navigation metadata
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(pagination),
                 type_: Udt(
                     ScSpecTypeUdt {
                         name: StringM(PaginationParams),
                     },
                 ),
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(role_filter),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: Udt(
                             ScSpecTypeUdt {
                                 name: StringM(UserRole),
                             },
                         ),
                     },
                 ),
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(status_filter),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: Udt(
                             ScSpecTypeUdt {
                                 name: StringM(UserStatus),
                             },
                         ),
                     },
                 ),
             },
         ],
     )
     Output: VecM(
         [
             Udt(
                 ScSpecTypeUdt {
                     name: StringM(PaginatedLightProfiles),
                 },
             ),
         ],
     )

 • Function: initialize_system
     Docs: Initialize the admin system (one-time only)
           
           # Arguments
           * `env` - Soroban environment
           * `initializer` - Address performing the initialization
           * `super_admin` - Address that will become the super admin
           * `max_page_size` - Optional maximum page size (default: 100, max: 1000)
           
           # Returns
           * `AdminConfig` - The created admin configuration
           
           # Panics
           * If system has already been initialized
           * If max_page_size exceeds 1000
           
           # Examples
           
           ```rust
           // Initialize with default settings
           let config = contract.initialize_system(
           env.clone(),
           deployer_address,
           super_admin_address,
           None
           );
           
           // Initialize with custom page size
           let config = contract.initialize_system(
           env.clone(),
           deployer_address,
           super_admin_address,
           Some(500)
           );
           ```
           
           # Edge Cases
           
           * **Double initialization**: Will panic if called more than once
           * **Invalid page size**: Will panic if max_page_size > 1000
           * **Super admin privileges**: Super admin cannot be removed after initialization
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(initializer),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(super_admin),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(max_page_size),
                 type_: Option(
                     ScSpecTypeOption {
                         value_type: U32,
                     },
                 ),
             },
         ],
     )
     Output: VecM(
         [
             Udt(
                 ScSpecTypeUdt {
                     name: StringM(AdminConfig),
                 },
             ),
         ],
     )

 • Function: add_admin
     Docs: Add a new admin (super admin only)
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address performing the call (must be super admin)
           * `new_admin` - Address to be added as admin
           
           # Panics
           * If caller is not the super admin
           * If system is not initialized
           * If new_admin is already an admin
           
           # Examples
           
           ```rust
           // Super admin adding a new admin
           contract.add_admin(env.clone(), super_admin_address, new_admin_address);
           ```
           
           # Edge Cases
           
           * **Already admin**: Will panic if trying to add an existing admin
           * **Self-promotion**: Super admin cannot add themselves (redundant)
           * **Non-existent user**: Can add admin privileges to any address
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(new_admin),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [],
     )

 • Function: remove_admin
     Docs: Remove an admin (super admin only)
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address performing the call (must be super admin)
           * `admin_to_remove` - Address to be removed from admins
           
           # Panics
           * If caller is not the super admin
           * If system is not initialized
           * If admin_to_remove is not an admin
           * If trying to remove the super admin
           
           # Examples
           
           ```rust
           // Super admin removing another admin
           contract.remove_admin(env.clone(), super_admin_address, admin_to_remove);
           ```
           
           # Edge Cases
           
           * **Super admin protection**: Cannot remove the super admin
           * **Non-admin**: Will panic if trying to remove a non-admin address
           * **Self-removal**: Super admin cannot remove themselves
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(admin_to_remove),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [],
     )

 • Function: get_admins
     Docs: Get list of all admins (admin only)
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address performing the call (must be admin)
           
           # Returns
           * `Vec<Address>` - List of all admin addresses including super admin
           
           # Panics
           * If caller is not an admin
           * If system is not initialized
           
           # Examples
           
           ```rust
           // Get all admin addresses
           let admins = contract.get_admins(env.clone(), admin_address);
           for admin in admins {
           // Process each admin address
           }
           ```
           
           # Edge Cases
           
           * **Empty list**: Returns vector with only super admin if no other admins exist
           * **Admin only**: Only admins can view the admin list
           * **Order**: Super admin is typically first in the list
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [
             Vec(
                 ScSpecTypeVec {
                     element_type: Address,
                 },
             ),
         ],
     )

 • Function: is_system_initialized
     Docs: Check if the system is initialized
           
           # Arguments
           * `env` - Soroban environment
           
           # Returns
           * `bool` - True if system is initialized
           
           # Examples
           
           ```rust
           // Check if admin system is ready
           let is_initialized = contract.is_system_initialized(env.clone());
           if !is_initialized {
           // Initialize the system first
           contract.initialize_system(env, deployer, super_admin, None);
           }
           ```
           
           # Edge Cases
           
           * **Fresh deployment**: Returns `false` for newly deployed contracts
           * **Public access**: Anyone can check initialization status
           * **One-time check**: Once initialized, always returns `true`
     Inputs: VecM(
         [],
     )
     Output: VecM(
         [
             Bool,
         ],
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

 • Function: export_user_data
     Docs: Export all user data for backup purposes (admin only)
           
           This function exports all user profiles and administrative data
           for backup and recovery purposes. Only admins can perform this operation.
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address performing the export (must be admin)
           
           # Returns
           * `UserBackupData` - Complete backup data structure
           
           # Panics
           * If caller is not an admin
           * If system is not initialized
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
         ],
     )
     Output: VecM(
         [
             Udt(
                 ScSpecTypeUdt {
                     name: StringM(UserBackupData),
                 },
             ),
         ],
     )

 • Function: import_user_data
     Docs: Import user data from backup (admin only)
           
           This function imports user data from a backup structure.
           Only admins can perform this operation. This will overwrite existing data.
           
           # Arguments
           * `env` - Soroban environment
           * `caller` - Address performing the import (must be admin)
           * `backup_data` - Backup data structure to import
           
           # Returns
           * `u32` - Number of users imported
           
           # Panics
           * If caller is not an admin
           * If backup data is invalid
           * If import operation fails
     Inputs: VecM(
         [
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(caller),
                 type_: Address,
             },
             ScSpecFunctionInputV0 {
                 doc: StringM(),
                 name: StringM(backup_data),
                 type_: Udt(
                     ScSpecTypeUdt {
                         name: StringM(UserBackupData),
                     },
                 ),
             },
         ],
     )
     Output: VecM(
         [
             U32,
         ],
     )



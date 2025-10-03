import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}




export const Errors = {
  1: {message:"AlreadInitialized"},
  2: {message:"InvalidMaxPageSize"},
  3: {message:"SystemNotInitialized"},
  4: {message:"AccessDenied"},
  5: {message:"SuperAdminNotRegular"},
  6: {message:"OperationFailed"},
  7: {message:"MaxAdminsReached"},
  8: {message:"CannotRemoveSuperAdmin"},
  9: {message:"UserProfileExists"},
  10: {message:"NameRequired"},
  11: {message:"EmailRequired"},
  12: {message:"CountryRequired"},
  15: {message:"InvalidEmailFormat"},
  16: {message:"EmailAlreadyExists"},
  17: {message:"InvalidField"},
  19: {message:"InvalidProfilePicURL"},
  20: {message:"UserNotFound"},
  21: {message:"UserProfileNotFound"},
  22: {message:"InactiveUser"},
  23: {message:"PageParamTooLarge"},
  24: {message:"InvalidTitleLength"},
  25: {message:"PasswordMismatch"},
  26: {message:"RateLimitExceeded"},
  27: {message:"RateLimitNotConfigured"},
  28: {message:"PasswordTooShort"},
  29: {message:"PasswordTooLong"},
  30: {message:"PasswordMissingUppercase"},
  31: {message:"PasswordMissingLowercase"},
  32: {message:"PasswordMissingDigit"},
  33: {message:"PasswordMissingSpecialChar"},
  34: {message:"RequiredFieldMissing"},
  35: {message:"Unauthorized"}
}

/**
 * Errors that can occur during contract versioning operations
 */
export const VersioningError = {
  /**
   * Invalid version format
   */
  1: {message:"InvalidVersion"},
  /**
   * Version not found in history
   */
  2: {message:"VersionNotFound"},
  /**
   * Migration not compatible
   */
  3: {message:"MigrationNotCompatible"},
  /**
   * Migration already completed
   */
  4: {message:"MigrationAlreadyCompleted"},
  /**
   * Unauthorized migration attempt
   */
  5: {message:"UnauthorizedMigration"},
  /**
   * Migration failed
   */
  6: {message:"MigrationFailed"}
}


export interface UserProfile {
  /**
 * User's contact email address (required, must be unique)
 */
contact_email: string;
  /**
 * User's country of residence (optional)
 */
country: Option<string>;
  /**
 * User's full name (required)
 */
full_name: string;
  /**
 * User's profession or job title (optional)
 */
profession: Option<string>;
  /**
 * User's profile picture URL (optional)
 */
profile_picture_url: Option<string>;
  /**
 * User's learning goals or purpose (optional)
 */
purpose: Option<string>;
}

/**
 * Data keys for contract storage
 * 
 * Currently includes only UserProfile keyed by user Address
 */
export type DataKey = {tag: "UserProfile", values: readonly [string]} | {tag: "EmailIndex", values: readonly [string]};


/**
 * User profile information matching UI definition.
 * 
 * This struct contains user profile data with required and optional fields
 * as defined by the user interface requirements.
 */
export interface UserProfile {
  /**
 * User's contact email address (required, must be unique)
 */
contact_email: string;
  /**
 * User's country of residence (optional)
 */
country: Option<string>;
  /**
 * User's full name (required)
 */
full_name: string;
  /**
 * User's profession or job title (optional)
 */
profession: Option<string>;
  /**
 * User's profile picture URL (optional)
 */
profile_picture_url: Option<string>;
  /**
 * User's learning goals or purpose (optional)
 */
purpose: Option<string>;
}


/**
 * Struct for profile update parameters
 * Only includes fields that can be updated
 */
export interface ProfileUpdateParams {
  /**
 * User's country of residence
 */
country: Option<string>;
  /**
 * User's full name (optional update)
 */
full_name: Option<string>;
  /**
 * User's profession or job title
 */
profession: Option<string>;
  /**
 * User's profile picture URL
 */
profile_picture_url: Option<string>;
  /**
 * User's learning goals or purpose
 */
purpose: Option<string>;
}

/**
 * User roles in the SkillCert platform.
 * 
 * Defines the different types of users and their permission levels.
 */
export type UserRole = {tag: "Student", values: void} | {tag: "Instructor", values: void} | {tag: "Admin", values: void} | {tag: "SuperAdmin", values: void} | {tag: "Moderator", values: void} | {tag: "Support", values: void};

/**
 * Granular permissions for RBAC system.
 * 
 * Defines specific actions that can be granted or denied to users.
 */
export type Permission = {tag: "ViewUsers", values: void} | {tag: "EditUsers", values: void} | {tag: "DeleteUsers", values: void} | {tag: "CreateUsers", values: void} | {tag: "ViewCourses", values: void} | {tag: "CreateCourses", values: void} | {tag: "EditCourses", values: void} | {tag: "DeleteCourses", values: void} | {tag: "ManageCourseAccess", values: void} | {tag: "ManageSystem", values: void} | {tag: "ManageAdmins", values: void} | {tag: "ViewAnalytics", values: void} | {tag: "ModerateContent", values: void} | {tag: "ProvideSupport", values: void} | {tag: "ViewSupport", values: void};


/**
 * Role-based permissions mapping.
 * 
 * Defines which permissions are granted to each role by default.
 */
export interface RolePermissions {
  /**
 * List of permissions granted to this role
 */
permissions: Array<Permission>;
  /**
 * The role this permission set applies to
 */
role: UserRole;
}


/**
 * User-specific permission overrides.
 * 
 * Allows granting or revoking specific permissions to individual users.
 */
export interface UserPermissions {
  /**
 * Additional permissions granted beyond role defaults
 */
granted_permissions: Array<Permission>;
  /**
 * Permissions explicitly revoked from role defaults
 */
revoked_permissions: Array<Permission>;
  /**
 * The user address
 */
user: string;
}

/**
 * User account status.
 * 
 * Represents the current state of a user's account.
 */
export type UserStatus = {tag: "Active", values: void} | {tag: "Inactive", values: void} | {tag: "Suspended", values: void};


/**
 * Lightweight user profile for listing operations.
 * 
 * Contains essential user information for efficient querying and display in user lists.
 */
export interface LightProfile {
  /**
 * User's country of residence
 */
country: Option<string>;
  /**
 * User's full name
 */
full_name: string;
  /**
 * User's profession or job title
 */
profession: Option<string>;
  /**
 * User's role in the platform
 */
role: UserRole;
  /**
 * User's account status
 */
status: UserStatus;
  /**
 * User's blockchain address
 */
user_address: string;
}


/**
 * Rate limiting configuration for user operations.
 * 
 * Tracks rate limiting settings and current usage for spam protection.
 */
export interface RateLimitConfig {
  /**
 * Maximum operations allowed per window
 */
max_operations_per_window: u32;
  /**
 * Time window for rate limiting in seconds
 */
window_seconds: u64;
}


/**
 * Rate limiting tracking data for a specific address.
 * 
 * Stores the current usage count and window start time for rate limiting.
 */
export interface RateLimitData {
  /**
 * Current count of operations in this window
 */
count: u32;
  /**
 * Timestamp when the current window started
 */
window_start: u64;
}


/**
 * Administrative configuration for the user management system.
 * 
 * Contains system-wide settings and administrative information.
 */
export interface AdminConfig {
  /**
 * Whether the system has been initialized
 */
initialized: boolean;
  /**
 * Maximum allowed page size for queries
 */
max_page_size: u32;
  /**
 * Rate limiting configuration for user creation
 */
rate_limit_config: RateLimitConfig;
  /**
 * Address of the super administrator
 */
super_admin: string;
  /**
 * Total number of registered users
 */
total_user_count: u32;
}


/**
 * Backup data structure for user management system.
 * 
 * Contains all user data and system configuration for backup and recovery operations.
 */
export interface UserBackupData {
  /**
 * Administrative configuration
 */
admin_config: AdminConfig;
  /**
 * List of admin addresses
 */
admins: Array<string>;
  /**
 * Backup timestamp
 */
backup_timestamp: u64;
  /**
 * Backup version for compatibility
 */
backup_version: string;
  /**
 * Email to address mapping for uniqueness
 */
email_mappings: Map<string, string>;
  /**
 * All lightweight profiles for efficient queries
 */
light_profiles: Map<string, LightProfile>;
  /**
 * All user profiles in the system
 */
user_profiles: Map<string, UserProfile>;
  /**
 * List of all registered user addresses
 */
users_index: Array<string>;
}


/**
 * Pagination parameters for cursor-based pagination.
 * 
 * Used to implement efficient pagination that avoids gas limit issues
 * with large datasets by using cursor-based navigation.
 */
export interface PaginationParams {
  /**
 * Cursor for pagination (address of the last item from previous page)
 */
cursor: Option<string>;
  /**
 * Maximum number of items to return per page
 */
limit: u32;
}


/**
 * Pagination result with metadata for efficient navigation.
 * 
 * Contains the paginated data along with pagination metadata
 * to enable cursor-based navigation.
 */
export interface PaginatedLightProfiles {
  /**
 * The paginated data items
 */
data: Array<LightProfile>;
  /**
 * Whether there are more pages available
 */
has_more: boolean;
  /**
 * Cursor for the next page (None if this is the last page)
 */
next_cursor: Option<string>;
  /**
 * Total count of items matching the filter (optional, may be expensive to compute)
 */
total_count: Option<u32>;
}

/**
 * Storage keys for different data types in the user management contract.
 * 
 * This enum defines the various keys used to store and retrieve
 * user data from the contract's persistent storage.
 */
export type DataKey = {tag: "UserProfile", values: readonly [string]} | {tag: "Admin", values: readonly [string]} | {tag: "UserProfileLight", values: readonly [string]} | {tag: "UsersIndex", values: void} | {tag: "EmailIndex", values: readonly [string]} | {tag: "Admins", values: void} | {tag: "UserRole", values: readonly [string]} | {tag: "AdminConfig", values: void} | {tag: "RateLimit", values: readonly [string]} | {tag: "RolePermissions", values: readonly [UserRole]} | {tag: "UserPermissions", values: readonly [string]} | {tag: "DefaultRolePermissions", values: void};

export interface Client {
  /**
   * Construct and simulate a get_user_profile transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Retrieve a user profile for the authenticated user.
   * 
   * This function fetches the complete user profile associated with the provided
   * blockchain address. The user must be authenticated; otherwise, the function
   * will panic.
   * 
   * ### Arguments
   * 
   * * `env` - The Soroban environment.
   * * `user` - The address of the user whose profile is being requested.
   * 
   * ### Returns
   * 
   * Returns the `UserProfile` corresponding to the authenticated user.
   * 
   * ### Panics
   * 
   * * If the user is not authenticated (`require_auth` fails).
   * * If the user profile does not exist (`UserNotFound` error).
   * 
   * ### Examples
   * 
   * ```rust
   * // Assuming the user is authenticated in the environment
   * let profile = contract.get_user_profile(env.clone(), my_address);
   * println!("User full name: {}", profile.full_name);
   * ```
   * 
   * ### Notes
   * 
   * * Only the user themselves can fetch their profile; there is no admin override
   * in this function.
   * * If the profile is not found in storage, the function will panic with
   * `UserNotFound`.
   */
  get_user_profile: ({user}: {user: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<UserProfile>>>

  /**
   * Construct and simulate a get_user_by_id transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Retrieve a user profile by their address.
   * 
   * This function fetches a complete user profile using the user's blockchain address.
   * Access may be restricted based on the requester's permissions.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `requester` - The address of the user requesting the profile
   * * `user_id` - The address of the user whose profile is being requested
   * 
   * # Returns
   * 
   * Returns the requested `UserProfile`.
   * 
   * # Panics
   * 
   * * If the user profile doesn't exist
   * * If the requester doesn't have permission to view the profile
   * * If the requester is not the user themselves or an admin
   * 
   * # Examples
   * 
   * ```rust
   * // Get your own profile
   * let my_profile = contract.get_user_by_id(env.clone(), my_address, my_address);
   * 
   * // Admin getting any user's profile
   * let user_profile = contract.get_user_by_id(env.clone(), admin_address, user_address);
   * ```
   * 
   * # Edge Cases
   * 
   * * **Non-existent user**: Will panic with appropriate error message
   * * **Inactive user**: Returns profile but status will be `UserStatus::Inactive`
   * * **Permission denied**:
   */
  get_user_by_id: ({requester, user_id}: {requester: string, user_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<UserProfile>>

  /**
   * Construct and simulate a create_user_profile transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Create a new user profile
   * 
   * Creates a new user profile using a UserProfile struct.
   * Validates mandatory fields (full_name and contact_email) and saves the profile.
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `user` - Address of the user whose profile is being created
   * * `profile` - UserProfile struct containing all profile data
   * 
   * # Returns
   * * `UserProfile` - The created user profile
   * 
   * # Panics
   * * If mandatory fields (full_name, contact_email) are missing
   * * If user profile already exists
   * * If email format is invalid
   * * If validation rules are violated
   * 
   * # Events
   * Emits a user creation event upon successful creation
   * 
   * # Examples
   * 
   * ```rust
   * let profile = UserProfile {
   * full_name: "John Doe".try_into().unwrap(),
   * contact_email: "john@example.com".try_into().unwrap(),
   * role: UserRole::Student,
   * status: UserStatus::Active,
   * country: Some("US".try_into().unwrap()),
   * ..Default::default()
   * };
   * 
   * let created_profile = contract.create_user_profile(env, user_address, profile);
   * ```
   * 
   * # Edge Cases
   * 
   * * **Duplicate profile**: Will panic if user al
   */
  create_user_profile: ({user, profile}: {user: string, profile: UserProfile}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<UserProfile>>

  /**
   * Construct and simulate a edit_user_profile transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Edit an existing user profile
   * 
   * Updates an existing user profile with new values for allowed fields.
   * Only the user themselves or administrators can perform updates.
   * Email and role fields cannot be updated through this function.
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address of the user performing the update
   * * `user_id` - Address of the user whose profile is being updated
   * * `updates` - ProfileUpdateParams containing fields to update
   * 
   * # Returns
   * * `UserProfile` - The updated user profile
   * 
   * # Panics
   * * If caller authentication fails
   * * If user profile doesn't exist
   * * If caller lacks permission to edit
   * * If any field validation fails
   * * If user is inactive
   * 
   * # Events
   * Emits a user update event upon successful profile update
   * 
   * # Examples
   * 
   * ```rust
   * let updates = ProfileUpdateParams {
   * full_name: Some("Jane Doe".try_into().unwrap()),
   * country: Some("CA".try_into().unwrap()),
   * bio: Some("Updated bio".try_into().unwrap()),
   * ..Default::default()
   * };
   * 
   * let updated_profile = contract.edit_user_profile(env, caller_addres
   */
  edit_user_profile: ({caller, user_id, updates}: {caller: string, user_id: string, updates: ProfileUpdateParams}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<UserProfile>>

  /**
   * Construct and simulate a is_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if an address has admin privileges.
   * 
   * This function is used by other contracts to verify admin status
   * for cross-contract authorization checks.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `who` - The address to check for admin privileges
   * 
   * # Returns
   * 
   * Returns `true` if the address has admin privileges, `false` otherwise.
   * 
   * # Examples
   * 
   * ```rust
   * // Check if user is admin
   * let is_admin = contract.is_admin(env.clone(), user_address);
   * if is_admin {
   * // Perform admin operations
   * }
   * 
   * // Cross-contract admin check
   * let can_perform_action = contract.is_admin(env.clone(), caller_address);
   * ```
   * 
   * # Edge Cases
   * 
   * * **System not initialized**: Returns `false` if admin system hasn't been set up
   * * **Non-existent user**: Returns `false` for addresses that don't exist
   * * **Regular users**: Always returns `false` for non-admin users
   */
  is_admin: ({who}: {who: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a delete_user transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Delete (deactivate) a user account
   * 
   * Performs a soft delete by marking the user as inactive instead of permanent deletion.
   * Only admins or the user themselves can trigger deletion.
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the deletion (must be admin or the user themselves)
   * * `user_id` - Address of the user to be deactivated
   * 
   * # Panics
   * * If caller authentication fails
   * * If user doesn't exist
   * * If caller is neither admin nor the user themselves
   * * If user is already inactive
   * 
   * # Events
   * Emits a user deactivation event upon successful deletion
   * 
   * # Examples
   * 
   * ```rust
   * // User deleting their own account
   * contract.delete_user(env.clone(), user_address, user_address);
   * 
   * // Admin deleting another user's account
   * contract.delete_user(env.clone(), admin_address, user_to_delete);
   * ```
   * 
   * # Edge Cases
   * 
   * * **Already inactive**: Will panic if trying to delete an already inactive user
   * * **Permission denied**: Non-admin users can only delete their own accounts
   * * **Data preservation**: User data is preserved
   */
  delete_user: ({caller, user_id}: {caller: string, user_id: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a list_all_users transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Lists all registered users with pagination and filtering (admin-only)
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the call (must be admin)
   * * `page` - Zero-based page index
   * * `page_size` - Number of items per page (must be > 0)
   * * `role_filter` - Optional role filter
   * * `country_filter` - Optional country filter
   * * `status_filter` - Optional status filter
   * 
   * # Returns
   * * `Vec<LightProfile>` - Filtered and paginated lightweight user profiles
   * 
   * # Panics
   * * If caller is not an admin
   * * If page_size is 0 or exceeds maximum allowed
   * * If system is not initialized
   * 
   * # Examples
   * 
   * ```rust
   * // Get first page with 10 users
   * let users = contract.list_all_users(
   * env.clone(),
   * admin_address,
   * 0,  // page 0
   * 10, // page size
   * None, None, None // no filters
   * );
   * 
   * // Filter by role and country
   * let students = contract.list_all_users(
   * env.clone(),
   * admin_address,
   * 0, 20,
   * Some(UserRole::Student),
   * Some("US".try_into().unwrap()),
   * None
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Empty results**: Returns empty vector if no users match filter
   */
  list_all_users: ({caller, page, page_size, role_filter, country_filter, status_filter}: {caller: string, page: u32, page_size: u32, role_filter: Option<UserRole>, country_filter: Option<string>, status_filter: Option<UserStatus>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<LightProfile>>>

  /**
   * Construct and simulate a list_all_users_advanced transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Lists all registered users with advanced filtering including text search (admin-only).
   * 
   * This is the new version that supports text search functionality.
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the call (must be admin)
   * * `page` - Zero-based page index
   * * `page_size` - Number of items per page
   * * `role_filter` - Optional role filter
   * * `country_filter` - Optional country filter
   * * `status_filter` - Optional status filter
   * * `search_text` - Optional text search in name and profession
   * 
   * # Returns
   * * `Vec<LightProfile>` - Filtered and paginated lightweight user profiles
   */
  list_all_users_advanced: ({caller, page, page_size, role_filter, country_filter, status_filter, search_text}: {caller: string, page: u32, page_size: u32, role_filter: Option<UserRole>, country_filter: Option<string>, status_filter: Option<UserStatus>, search_text: Option<string>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<LightProfile>>>

  /**
   * Construct and simulate a list_all_users_cursor transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Lists all registered users with cursor-based pagination and filtering (admin-only)
   * 
   * This function implements efficient cursor-based pagination to avoid gas limit issues
   * when dealing with large datasets. It returns a PaginatedResult with metadata for
   * efficient navigation.
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the call (must be admin)
   * * `pagination` - Pagination parameters including cursor and limit
   * * `role_filter` - Optional filter for user role
   * * `status_filter` - Optional filter for user status
   * 
   * # Returns
   * * `PaginatedLightProfiles` - Paginated results with navigation metadata
   */
  list_all_users_cursor: ({caller, pagination, role_filter, status_filter}: {caller: string, pagination: PaginationParams, role_filter: Option<UserRole>, status_filter: Option<UserStatus>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<PaginatedLightProfiles>>

  /**
   * Construct and simulate a initialize_system transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Initialize the admin system (one-time only)
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `initializer` - Address performing the initialization
   * * `super_admin` - Address that will become the super admin
   * * `max_page_size` - Optional maximum page size (default: 100, max: 1000)
   * 
   * # Returns
   * * `AdminConfig` - The created admin configuration
   * 
   * # Panics
   * * If system has already been initialized
   * * If max_page_size exceeds 1000
   * 
   * # Examples
   * 
   * ```rust
   * // Initialize with default settings
   * let config = contract.initialize_system(
   * env.clone(),
   * deployer_address,
   * super_admin_address,
   * None
   * );
   * 
   * // Initialize with custom page size
   * let config = contract.initialize_system(
   * env.clone(),
   * deployer_address,
   * super_admin_address,
   * Some(500)
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Double initialization**: Will panic if called more than once
   * * **Invalid page size**: Will panic if max_page_size > 1000
   * * **Super admin privileges**: Super admin cannot be removed after initialization
   */
  initialize_system: ({initializer, super_admin, max_page_size}: {initializer: string, super_admin: string, max_page_size: Option<u32>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<AdminConfig>>

  /**
   * Construct and simulate a add_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Add a new admin (super admin only)
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the call (must be super admin)
   * * `new_admin` - Address to be added as admin
   * 
   * # Panics
   * * If caller is not the super admin
   * * If system is not initialized
   * * If new_admin is already an admin
   * 
   * # Examples
   * 
   * ```rust
   * // Super admin adding a new admin
   * contract.add_admin(env.clone(), super_admin_address, new_admin_address);
   * ```
   * 
   * # Edge Cases
   * 
   * * **Already admin**: Will panic if trying to add an existing admin
   * * **Self-promotion**: Super admin cannot add themselves (redundant)
   * * **Non-existent user**: Can add admin privileges to any address
   */
  add_admin: ({caller, new_admin}: {caller: string, new_admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a remove_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Remove an admin (super admin only)
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the call (must be super admin)
   * * `admin_to_remove` - Address to be removed from admins
   * 
   * # Panics
   * * If caller is not the super admin
   * * If system is not initialized
   * * If admin_to_remove is not an admin
   * * If trying to remove the super admin
   * 
   * # Examples
   * 
   * ```rust
   * // Super admin removing another admin
   * contract.remove_admin(env.clone(), super_admin_address, admin_to_remove);
   * ```
   * 
   * # Edge Cases
   * 
   * * **Super admin protection**: Cannot remove the super admin
   * * **Non-admin**: Will panic if trying to remove a non-admin address
   * * **Self-removal**: Super admin cannot remove themselves
   */
  remove_admin: ({caller, admin_to_remove}: {caller: string, admin_to_remove: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a get_admins transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get list of all admins (admin only)
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the call (must be admin)
   * 
   * # Returns
   * * `Vec<Address>` - List of all admin addresses including super admin
   * 
   * # Panics
   * * If caller is not an admin
   * * If system is not initialized
   * 
   * # Examples
   * 
   * ```rust
   * // Get all admin addresses
   * let admins = contract.get_admins(env.clone(), admin_address);
   * for admin in admins {
   * // Process each admin address
   * }
   * ```
   * 
   * # Edge Cases
   * 
   * * **Empty list**: Returns vector with only super admin if no other admins exist
   * * **Admin only**: Only admins can view the admin list
   * * **Order**: Super admin is typically first in the list
   */
  get_admins: ({caller}: {caller: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<string>>>

  /**
   * Construct and simulate a is_system_initialized transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if the system is initialized
   * 
   * # Arguments
   * * `env` - Soroban environment
   * 
   * # Returns
   * * `bool` - True if system is initialized
   * 
   * # Examples
   * 
   * ```rust
   * // Check if admin system is ready
   * let is_initialized = contract.is_system_initialized(env.clone());
   * if !is_initialized {
   * // Initialize the system first
   * contract.initialize_system(env, deployer, super_admin, None);
   * }
   * ```
   * 
   * # Edge Cases
   * 
   * * **Fresh deployment**: Returns `false` for newly deployed contracts
   * * **Public access**: Anyone can check initialization status
   * * **One-time check**: Once initialized, always returns `true`
   */
  is_system_initialized: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a get_contract_version transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get the current contract version
   * 
   * Returns the semantic version of the current contract deployment.
   * This is useful for tracking contract upgrades and compatibility.
   * 
   * # Arguments
   * * `_env` - The Soroban environment (unused)
   * 
   * # Returns
   * * `String` - The current contract version
   */
  get_contract_version: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a export_user_data transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Export all user data for backup purposes (admin only)
   * 
   * This function exports all user profiles and administrative data
   * for backup and recovery purposes. Only admins can perform this operation.
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the export (must be admin)
   * 
   * # Returns
   * * `UserBackupData` - Complete backup data structure
   * 
   * # Panics
   * * If caller is not an admin
   * * If system is not initialized
   */
  export_user_data: ({caller}: {caller: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<UserBackupData>>

  /**
   * Construct and simulate a import_user_data transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Import user data from backup (admin only)
   * 
   * This function imports user data from a backup structure.
   * Only admins can perform this operation. This will overwrite existing data.
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the import (must be admin)
   * * `backup_data` - Backup data structure to import
   * 
   * # Returns
   * * `u32` - Number of users imported
   * 
   * # Panics
   * * If caller is not an admin
   * * If backup data is invalid
   * * If import operation fails
   */
  import_user_data: ({caller, backup_data}: {caller: string, backup_data: UserBackupData}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAABAAAAAAAAAAAAAAABUVycm9yAAAAAAAAIAAAAAAAAAARQWxyZWFkSW5pdGlhbGl6ZWQAAAAAAAABAAAAAAAAABJJbnZhbGlkTWF4UGFnZVNpemUAAAAAAAIAAAAAAAAAFFN5c3RlbU5vdEluaXRpYWxpemVkAAAAAwAAAAAAAAAMQWNjZXNzRGVuaWVkAAAABAAAAAAAAAAUU3VwZXJBZG1pbk5vdFJlZ3VsYXIAAAAFAAAAAAAAAA9PcGVyYXRpb25GYWlsZWQAAAAABgAAAAAAAAAQTWF4QWRtaW5zUmVhY2hlZAAAAAcAAAAAAAAAFkNhbm5vdFJlbW92ZVN1cGVyQWRtaW4AAAAAAAgAAAAAAAAAEVVzZXJQcm9maWxlRXhpc3RzAAAAAAAACQAAAAAAAAAMTmFtZVJlcXVpcmVkAAAACgAAAAAAAAANRW1haWxSZXF1aXJlZAAAAAAAAAsAAAAAAAAAD0NvdW50cnlSZXF1aXJlZAAAAAAMAAAAAAAAABJJbnZhbGlkRW1haWxGb3JtYXQAAAAAAA8AAAAAAAAAEkVtYWlsQWxyZWFkeUV4aXN0cwAAAAAAEAAAAAAAAAAMSW52YWxpZEZpZWxkAAAAEQAAAAAAAAAUSW52YWxpZFByb2ZpbGVQaWNVUkwAAAATAAAAAAAAAAxVc2VyTm90Rm91bmQAAAAUAAAAAAAAABNVc2VyUHJvZmlsZU5vdEZvdW5kAAAAABUAAAAAAAAADEluYWN0aXZlVXNlcgAAABYAAAAAAAAAEVBhZ2VQYXJhbVRvb0xhcmdlAAAAAAAAFwAAAAAAAAASSW52YWxpZFRpdGxlTGVuZ3RoAAAAAAAYAAAAAAAAABBQYXNzd29yZE1pc21hdGNoAAAAGQAAAAAAAAARUmF0ZUxpbWl0RXhjZWVkZWQAAAAAAAAaAAAAAAAAABZSYXRlTGltaXROb3RDb25maWd1cmVkAAAAAAAbAAAAAAAAABBQYXNzd29yZFRvb1Nob3J0AAAAHAAAAAAAAAAPUGFzc3dvcmRUb29Mb25nAAAAAB0AAAAAAAAAGFBhc3N3b3JkTWlzc2luZ1VwcGVyY2FzZQAAAB4AAAAAAAAAGFBhc3N3b3JkTWlzc2luZ0xvd2VyY2FzZQAAAB8AAAAAAAAAFFBhc3N3b3JkTWlzc2luZ0RpZ2l0AAAAIAAAAAAAAAAaUGFzc3dvcmRNaXNzaW5nU3BlY2lhbENoYXIAAAAAACEAAAAAAAAAFFJlcXVpcmVkRmllbGRNaXNzaW5nAAAAIgAAAAAAAAAMVW5hdXRob3JpemVkAAAAIw==",
        "AAAABAAAADtFcnJvcnMgdGhhdCBjYW4gb2NjdXIgZHVyaW5nIGNvbnRyYWN0IHZlcnNpb25pbmcgb3BlcmF0aW9ucwAAAAAAAAAAD1ZlcnNpb25pbmdFcnJvcgAAAAAGAAAAFkludmFsaWQgdmVyc2lvbiBmb3JtYXQAAAAAAA5JbnZhbGlkVmVyc2lvbgAAAAAAAQAAABxWZXJzaW9uIG5vdCBmb3VuZCBpbiBoaXN0b3J5AAAAD1ZlcnNpb25Ob3RGb3VuZAAAAAACAAAAGE1pZ3JhdGlvbiBub3QgY29tcGF0aWJsZQAAABZNaWdyYXRpb25Ob3RDb21wYXRpYmxlAAAAAAADAAAAG01pZ3JhdGlvbiBhbHJlYWR5IGNvbXBsZXRlZAAAAAAZTWlncmF0aW9uQWxyZWFkeUNvbXBsZXRlZAAAAAAAAAQAAAAeVW5hdXRob3JpemVkIG1pZ3JhdGlvbiBhdHRlbXB0AAAAAAAVVW5hdXRob3JpemVkTWlncmF0aW9uAAAAAAAABQAAABBNaWdyYXRpb24gZmFpbGVkAAAAD01pZ3JhdGlvbkZhaWxlZAAAAAAG",
        "AAAAAQAAAAAAAAAAAAAAC1VzZXJQcm9maWxlAAAAAAYAAAA3VXNlcidzIGNvbnRhY3QgZW1haWwgYWRkcmVzcyAocmVxdWlyZWQsIG11c3QgYmUgdW5pcXVlKQAAAAANY29udGFjdF9lbWFpbAAAAAAAABAAAAAmVXNlcidzIGNvdW50cnkgb2YgcmVzaWRlbmNlIChvcHRpb25hbCkAAAAAAAdjb3VudHJ5AAAAA+gAAAAQAAAAG1VzZXIncyBmdWxsIG5hbWUgKHJlcXVpcmVkKQAAAAAJZnVsbF9uYW1lAAAAAAAAEAAAAClVc2VyJ3MgcHJvZmVzc2lvbiBvciBqb2IgdGl0bGUgKG9wdGlvbmFsKQAAAAAAAApwcm9mZXNzaW9uAAAAAAPoAAAAEAAAACVVc2VyJ3MgcHJvZmlsZSBwaWN0dXJlIFVSTCAob3B0aW9uYWwpAAAAAAAAE3Byb2ZpbGVfcGljdHVyZV91cmwAAAAD6AAAABAAAAArVXNlcidzIGxlYXJuaW5nIGdvYWxzIG9yIHB1cnBvc2UgKG9wdGlvbmFsKQAAAAAHcHVycG9zZQAAAAPoAAAAEA==",
        "AAAAAgAAAFlEYXRhIGtleXMgZm9yIGNvbnRyYWN0IHN0b3JhZ2UKCkN1cnJlbnRseSBpbmNsdWRlcyBvbmx5IFVzZXJQcm9maWxlIGtleWVkIGJ5IHVzZXIgQWRkcmVzcwAAAAAAAAAAAAAHRGF0YUtleQAAAAACAAAAAQAAAAAAAAALVXNlclByb2ZpbGUAAAAAAQAAABMAAAABAAAAAAAAAApFbWFpbEluZGV4AAAAAAABAAAAEA==",
        "AAAAAQAAAKlVc2VyIHByb2ZpbGUgaW5mb3JtYXRpb24gbWF0Y2hpbmcgVUkgZGVmaW5pdGlvbi4KClRoaXMgc3RydWN0IGNvbnRhaW5zIHVzZXIgcHJvZmlsZSBkYXRhIHdpdGggcmVxdWlyZWQgYW5kIG9wdGlvbmFsIGZpZWxkcwphcyBkZWZpbmVkIGJ5IHRoZSB1c2VyIGludGVyZmFjZSByZXF1aXJlbWVudHMuAAAAAAAAAAAAAAtVc2VyUHJvZmlsZQAAAAAGAAAAN1VzZXIncyBjb250YWN0IGVtYWlsIGFkZHJlc3MgKHJlcXVpcmVkLCBtdXN0IGJlIHVuaXF1ZSkAAAAADWNvbnRhY3RfZW1haWwAAAAAAAAQAAAAJlVzZXIncyBjb3VudHJ5IG9mIHJlc2lkZW5jZSAob3B0aW9uYWwpAAAAAAAHY291bnRyeQAAAAPoAAAAEAAAABtVc2VyJ3MgZnVsbCBuYW1lIChyZXF1aXJlZCkAAAAACWZ1bGxfbmFtZQAAAAAAABAAAAApVXNlcidzIHByb2Zlc3Npb24gb3Igam9iIHRpdGxlIChvcHRpb25hbCkAAAAAAAAKcHJvZmVzc2lvbgAAAAAD6AAAABAAAAAlVXNlcidzIHByb2ZpbGUgcGljdHVyZSBVUkwgKG9wdGlvbmFsKQAAAAAAABNwcm9maWxlX3BpY3R1cmVfdXJsAAAAA+gAAAAQAAAAK1VzZXIncyBsZWFybmluZyBnb2FscyBvciBwdXJwb3NlIChvcHRpb25hbCkAAAAAB3B1cnBvc2UAAAAD6AAAABA=",
        "AAAAAQAAAE1TdHJ1Y3QgZm9yIHByb2ZpbGUgdXBkYXRlIHBhcmFtZXRlcnMKT25seSBpbmNsdWRlcyBmaWVsZHMgdGhhdCBjYW4gYmUgdXBkYXRlZAAAAAAAAAAAAAATUHJvZmlsZVVwZGF0ZVBhcmFtcwAAAAAFAAAAG1VzZXIncyBjb3VudHJ5IG9mIHJlc2lkZW5jZQAAAAAHY291bnRyeQAAAAPoAAAAEAAAACJVc2VyJ3MgZnVsbCBuYW1lIChvcHRpb25hbCB1cGRhdGUpAAAAAAAJZnVsbF9uYW1lAAAAAAAD6AAAABAAAAAeVXNlcidzIHByb2Zlc3Npb24gb3Igam9iIHRpdGxlAAAAAAAKcHJvZmVzc2lvbgAAAAAD6AAAABAAAAAaVXNlcidzIHByb2ZpbGUgcGljdHVyZSBVUkwAAAAAABNwcm9maWxlX3BpY3R1cmVfdXJsAAAAA+gAAAAQAAAAIFVzZXIncyBsZWFybmluZyBnb2FscyBvciBwdXJwb3NlAAAAB3B1cnBvc2UAAAAD6AAAABA=",
        "AAAAAgAAAGhVc2VyIHJvbGVzIGluIHRoZSBTa2lsbENlcnQgcGxhdGZvcm0uCgpEZWZpbmVzIHRoZSBkaWZmZXJlbnQgdHlwZXMgb2YgdXNlcnMgYW5kIHRoZWlyIHBlcm1pc3Npb24gbGV2ZWxzLgAAAAAAAAAIVXNlclJvbGUAAAAGAAAAAAAAAC9SZWd1bGFyIHBsYXRmb3JtIHVzZXIgd2hvIGNhbiBlbnJvbGwgaW4gY291cnNlcwAAAAAHU3R1ZGVudAAAAAAAAAAAJlVzZXIgd2hvIGNhbiBjcmVhdGUgYW5kIG1hbmFnZSBjb3Vyc2VzAAAAAAAKSW5zdHJ1Y3RvcgAAAAAAAAAAAC9QbGF0Zm9ybSBhZG1pbmlzdHJhdG9yIHdpdGggZWxldmF0ZWQgcHJpdmlsZWdlcwAAAAAFQWRtaW4AAAAAAAAAAAAAK1N1cGVyIGFkbWluaXN0cmF0b3Igd2l0aCBmdWxsIHN5c3RlbSBhY2Nlc3MAAAAAClN1cGVyQWRtaW4AAAAAAAAAAAAxQ29udGVudCBtb2RlcmF0b3Igd2l0aCBjb3Vyc2UgY29udGVudCBwZXJtaXNzaW9ucwAAAAAAAAlNb2RlcmF0b3IAAAAAAAAAAAAALlN1cHBvcnQgc3RhZmYgd2l0aCB1c2VyIGFzc2lzdGFuY2UgcGVybWlzc2lvbnMAAAAAAAdTdXBwb3J0AA==",
        "AAAAAgAAAGdHcmFudWxhciBwZXJtaXNzaW9ucyBmb3IgUkJBQyBzeXN0ZW0uCgpEZWZpbmVzIHNwZWNpZmljIGFjdGlvbnMgdGhhdCBjYW4gYmUgZ3JhbnRlZCBvciBkZW5pZWQgdG8gdXNlcnMuAAAAAAAAAAAKUGVybWlzc2lvbgAAAAAADwAAAAAAAAAWQ2FuIHZpZXcgdXNlciBwcm9maWxlcwAAAAAACVZpZXdVc2VycwAAAAAAAAAAAAAmQ2FuIGVkaXQgdXNlciBwcm9maWxlcyAob3duIG9yIG90aGVycykAAAAAAAlFZGl0VXNlcnMAAAAAAAAAAAAAG0NhbiBkZWxldGUvZGVhY3RpdmF0ZSB1c2VycwAAAAALRGVsZXRlVXNlcnMAAAAAAAAAABxDYW4gY3JlYXRlIG5ldyB1c2VyIGFjY291bnRzAAAAC0NyZWF0ZVVzZXJzAAAAAAAAAAAXQ2FuIHZpZXcgY291cnNlIGRldGFpbHMAAAAAC1ZpZXdDb3Vyc2VzAAAAAAAAAAAWQ2FuIGNyZWF0ZSBuZXcgY291cnNlcwAAAAAADUNyZWF0ZUNvdXJzZXMAAAAAAAAAAAAAF0NhbiBlZGl0IGNvdXJzZSBjb250ZW50AAAAAAtFZGl0Q291cnNlcwAAAAAAAAAAEkNhbiBkZWxldGUgY291cnNlcwAAAAAADURlbGV0ZUNvdXJzZXMAAAAAAAAAAAAAJ0NhbiBtYW5hZ2UgY291cnNlIGFjY2VzcyAoZ3JhbnQvcmV2b2tlKQAAAAASTWFuYWdlQ291cnNlQWNjZXNzAAAAAAAAAAAAH0NhbiBtYW5hZ2Ugc3lzdGVtIGNvbmZpZ3VyYXRpb24AAAAADE1hbmFnZVN5c3RlbQAAAAAAAAAWQ2FuIG1hbmFnZSBhZG1pbiByb2xlcwAAAAAADE1hbmFnZUFkbWlucwAAAAAAAAAZQ2FuIHZpZXcgc3lzdGVtIGFuYWx5dGljcwAAAAAAAA1WaWV3QW5hbHl0aWNzAAAAAAAAAAAAABRDYW4gbW9kZXJhdGUgY29udGVudAAAAA9Nb2RlcmF0ZUNvbnRlbnQAAAAAAAAAABhDYW4gcHJvdmlkZSB1c2VyIHN1cHBvcnQAAAAOUHJvdmlkZVN1cHBvcnQAAAAAAAAAAAAYQ2FuIHZpZXcgc3VwcG9ydCB0aWNrZXRzAAAAC1ZpZXdTdXBwb3J0AA==",
        "AAAAAQAAAF9Sb2xlLWJhc2VkIHBlcm1pc3Npb25zIG1hcHBpbmcuCgpEZWZpbmVzIHdoaWNoIHBlcm1pc3Npb25zIGFyZSBncmFudGVkIHRvIGVhY2ggcm9sZSBieSBkZWZhdWx0LgAAAAAAAAAAD1JvbGVQZXJtaXNzaW9ucwAAAAACAAAAKExpc3Qgb2YgcGVybWlzc2lvbnMgZ3JhbnRlZCB0byB0aGlzIHJvbGUAAAALcGVybWlzc2lvbnMAAAAD6gAAB9AAAAAKUGVybWlzc2lvbgAAAAAAJ1RoZSByb2xlIHRoaXMgcGVybWlzc2lvbiBzZXQgYXBwbGllcyB0bwAAAAAEcm9sZQAAB9AAAAAIVXNlclJvbGU=",
        "AAAAAQAAAGpVc2VyLXNwZWNpZmljIHBlcm1pc3Npb24gb3ZlcnJpZGVzLgoKQWxsb3dzIGdyYW50aW5nIG9yIHJldm9raW5nIHNwZWNpZmljIHBlcm1pc3Npb25zIHRvIGluZGl2aWR1YWwgdXNlcnMuAAAAAAAAAAAAD1VzZXJQZXJtaXNzaW9ucwAAAAADAAAAM0FkZGl0aW9uYWwgcGVybWlzc2lvbnMgZ3JhbnRlZCBiZXlvbmQgcm9sZSBkZWZhdWx0cwAAAAATZ3JhbnRlZF9wZXJtaXNzaW9ucwAAAAPqAAAH0AAAAApQZXJtaXNzaW9uAAAAAAAxUGVybWlzc2lvbnMgZXhwbGljaXRseSByZXZva2VkIGZyb20gcm9sZSBkZWZhdWx0cwAAAAAAABNyZXZva2VkX3Blcm1pc3Npb25zAAAAA+oAAAfQAAAAClBlcm1pc3Npb24AAAAAABBUaGUgdXNlciBhZGRyZXNzAAAABHVzZXIAAAAT",
        "AAAAAgAAAEdVc2VyIGFjY291bnQgc3RhdHVzLgoKUmVwcmVzZW50cyB0aGUgY3VycmVudCBzdGF0ZSBvZiBhIHVzZXIncyBhY2NvdW50LgAAAAAAAAAAClVzZXJTdGF0dXMAAAAAAAMAAAAAAAAAJVVzZXIgYWNjb3VudCBpcyBhY3RpdmUgYW5kIGZ1bmN0aW9uYWwAAAAAAAAGQWN0aXZlAAAAAAAAAAAAG1VzZXIgYWNjb3VudCBpcyBkZWFjdGl2YXRlZAAAAAAISW5hY3RpdmUAAAAAAAAAJVVzZXIgYWNjb3VudCBpcyB0ZW1wb3JhcmlseSBzdXNwZW5kZWQAAAAAAAAJU3VzcGVuZGVkAAAA",
        "AAAAAQAAAIdMaWdodHdlaWdodCB1c2VyIHByb2ZpbGUgZm9yIGxpc3Rpbmcgb3BlcmF0aW9ucy4KCkNvbnRhaW5zIGVzc2VudGlhbCB1c2VyIGluZm9ybWF0aW9uIGZvciBlZmZpY2llbnQgcXVlcnlpbmcgYW5kIGRpc3BsYXkgaW4gdXNlciBsaXN0cy4AAAAAAAAAAAxMaWdodFByb2ZpbGUAAAAGAAAAG1VzZXIncyBjb3VudHJ5IG9mIHJlc2lkZW5jZQAAAAAHY291bnRyeQAAAAPoAAAAEAAAABBVc2VyJ3MgZnVsbCBuYW1lAAAACWZ1bGxfbmFtZQAAAAAAABAAAAAeVXNlcidzIHByb2Zlc3Npb24gb3Igam9iIHRpdGxlAAAAAAAKcHJvZmVzc2lvbgAAAAAD6AAAABAAAAAbVXNlcidzIHJvbGUgaW4gdGhlIHBsYXRmb3JtAAAAAARyb2xlAAAH0AAAAAhVc2VyUm9sZQAAABVVc2VyJ3MgYWNjb3VudCBzdGF0dXMAAAAAAAAGc3RhdHVzAAAAAAfQAAAAClVzZXJTdGF0dXMAAAAAABlVc2VyJ3MgYmxvY2tjaGFpbiBhZGRyZXNzAAAAAAAADHVzZXJfYWRkcmVzcwAAABM=",
        "AAAAAQAAAHZSYXRlIGxpbWl0aW5nIGNvbmZpZ3VyYXRpb24gZm9yIHVzZXIgb3BlcmF0aW9ucy4KClRyYWNrcyByYXRlIGxpbWl0aW5nIHNldHRpbmdzIGFuZCBjdXJyZW50IHVzYWdlIGZvciBzcGFtIHByb3RlY3Rpb24uAAAAAAAAAAAAD1JhdGVMaW1pdENvbmZpZwAAAAACAAAAJU1heGltdW0gb3BlcmF0aW9ucyBhbGxvd2VkIHBlciB3aW5kb3cAAAAAAAAZbWF4X29wZXJhdGlvbnNfcGVyX3dpbmRvdwAAAAAAAAQAAAAoVGltZSB3aW5kb3cgZm9yIHJhdGUgbGltaXRpbmcgaW4gc2Vjb25kcwAAAA53aW5kb3dfc2Vjb25kcwAAAAAABg==",
        "AAAAAQAAAHxSYXRlIGxpbWl0aW5nIHRyYWNraW5nIGRhdGEgZm9yIGEgc3BlY2lmaWMgYWRkcmVzcy4KClN0b3JlcyB0aGUgY3VycmVudCB1c2FnZSBjb3VudCBhbmQgd2luZG93IHN0YXJ0IHRpbWUgZm9yIHJhdGUgbGltaXRpbmcuAAAAAAAAAA1SYXRlTGltaXREYXRhAAAAAAAAAgAAACpDdXJyZW50IGNvdW50IG9mIG9wZXJhdGlvbnMgaW4gdGhpcyB3aW5kb3cAAAAAAAVjb3VudAAAAAAAAAQAAAApVGltZXN0YW1wIHdoZW4gdGhlIGN1cnJlbnQgd2luZG93IHN0YXJ0ZWQAAAAAAAAMd2luZG93X3N0YXJ0AAAABg==",
        "AAAAAQAAAHtBZG1pbmlzdHJhdGl2ZSBjb25maWd1cmF0aW9uIGZvciB0aGUgdXNlciBtYW5hZ2VtZW50IHN5c3RlbS4KCkNvbnRhaW5zIHN5c3RlbS13aWRlIHNldHRpbmdzIGFuZCBhZG1pbmlzdHJhdGl2ZSBpbmZvcm1hdGlvbi4AAAAAAAAAAAtBZG1pbkNvbmZpZwAAAAAFAAAAJ1doZXRoZXIgdGhlIHN5c3RlbSBoYXMgYmVlbiBpbml0aWFsaXplZAAAAAALaW5pdGlhbGl6ZWQAAAAAAQAAACVNYXhpbXVtIGFsbG93ZWQgcGFnZSBzaXplIGZvciBxdWVyaWVzAAAAAAAADW1heF9wYWdlX3NpemUAAAAAAAAEAAAALVJhdGUgbGltaXRpbmcgY29uZmlndXJhdGlvbiBmb3IgdXNlciBjcmVhdGlvbgAAAAAAABFyYXRlX2xpbWl0X2NvbmZpZwAAAAAAB9AAAAAPUmF0ZUxpbWl0Q29uZmlnAAAAACJBZGRyZXNzIG9mIHRoZSBzdXBlciBhZG1pbmlzdHJhdG9yAAAAAAALc3VwZXJfYWRtaW4AAAAAEwAAACBUb3RhbCBudW1iZXIgb2YgcmVnaXN0ZXJlZCB1c2VycwAAABB0b3RhbF91c2VyX2NvdW50AAAABA==",
        "AAAAAQAAAIZCYWNrdXAgZGF0YSBzdHJ1Y3R1cmUgZm9yIHVzZXIgbWFuYWdlbWVudCBzeXN0ZW0uCgpDb250YWlucyBhbGwgdXNlciBkYXRhIGFuZCBzeXN0ZW0gY29uZmlndXJhdGlvbiBmb3IgYmFja3VwIGFuZCByZWNvdmVyeSBvcGVyYXRpb25zLgAAAAAAAAAAAA5Vc2VyQmFja3VwRGF0YQAAAAAACAAAABxBZG1pbmlzdHJhdGl2ZSBjb25maWd1cmF0aW9uAAAADGFkbWluX2NvbmZpZwAAB9AAAAALQWRtaW5Db25maWcAAAAAF0xpc3Qgb2YgYWRtaW4gYWRkcmVzc2VzAAAAAAZhZG1pbnMAAAAAA+oAAAATAAAAEEJhY2t1cCB0aW1lc3RhbXAAAAAQYmFja3VwX3RpbWVzdGFtcAAAAAYAAAAgQmFja3VwIHZlcnNpb24gZm9yIGNvbXBhdGliaWxpdHkAAAAOYmFja3VwX3ZlcnNpb24AAAAAABAAAAAnRW1haWwgdG8gYWRkcmVzcyBtYXBwaW5nIGZvciB1bmlxdWVuZXNzAAAAAA5lbWFpbF9tYXBwaW5ncwAAAAAD7AAAABAAAAATAAAALkFsbCBsaWdodHdlaWdodCBwcm9maWxlcyBmb3IgZWZmaWNpZW50IHF1ZXJpZXMAAAAAAA5saWdodF9wcm9maWxlcwAAAAAD7AAAABMAAAfQAAAADExpZ2h0UHJvZmlsZQAAAB9BbGwgdXNlciBwcm9maWxlcyBpbiB0aGUgc3lzdGVtAAAAAA11c2VyX3Byb2ZpbGVzAAAAAAAD7AAAABMAAAfQAAAAC1VzZXJQcm9maWxlAAAAACVMaXN0IG9mIGFsbCByZWdpc3RlcmVkIHVzZXIgYWRkcmVzc2VzAAAAAAAAC3VzZXJzX2luZGV4AAAAA+oAAAAT",
        "AAAAAQAAAK1QYWdpbmF0aW9uIHBhcmFtZXRlcnMgZm9yIGN1cnNvci1iYXNlZCBwYWdpbmF0aW9uLgoKVXNlZCB0byBpbXBsZW1lbnQgZWZmaWNpZW50IHBhZ2luYXRpb24gdGhhdCBhdm9pZHMgZ2FzIGxpbWl0IGlzc3Vlcwp3aXRoIGxhcmdlIGRhdGFzZXRzIGJ5IHVzaW5nIGN1cnNvci1iYXNlZCBuYXZpZ2F0aW9uLgAAAAAAAAAAAAAQUGFnaW5hdGlvblBhcmFtcwAAAAIAAABDQ3Vyc29yIGZvciBwYWdpbmF0aW9uIChhZGRyZXNzIG9mIHRoZSBsYXN0IGl0ZW0gZnJvbSBwcmV2aW91cyBwYWdlKQAAAAAGY3Vyc29yAAAAAAPoAAAAEwAAACpNYXhpbXVtIG51bWJlciBvZiBpdGVtcyB0byByZXR1cm4gcGVyIHBhZ2UAAAAAAAVsaW1pdAAAAAAAAAQ=",
        "AAAAAQAAAJhQYWdpbmF0aW9uIHJlc3VsdCB3aXRoIG1ldGFkYXRhIGZvciBlZmZpY2llbnQgbmF2aWdhdGlvbi4KCkNvbnRhaW5zIHRoZSBwYWdpbmF0ZWQgZGF0YSBhbG9uZyB3aXRoIHBhZ2luYXRpb24gbWV0YWRhdGEKdG8gZW5hYmxlIGN1cnNvci1iYXNlZCBuYXZpZ2F0aW9uLgAAAAAAAAAWUGFnaW5hdGVkTGlnaHRQcm9maWxlcwAAAAAABAAAABhUaGUgcGFnaW5hdGVkIGRhdGEgaXRlbXMAAAAEZGF0YQAAA+oAAAfQAAAADExpZ2h0UHJvZmlsZQAAACZXaGV0aGVyIHRoZXJlIGFyZSBtb3JlIHBhZ2VzIGF2YWlsYWJsZQAAAAAACGhhc19tb3JlAAAAAQAAADhDdXJzb3IgZm9yIHRoZSBuZXh0IHBhZ2UgKE5vbmUgaWYgdGhpcyBpcyB0aGUgbGFzdCBwYWdlKQAAAAtuZXh0X2N1cnNvcgAAAAPoAAAAEwAAAFBUb3RhbCBjb3VudCBvZiBpdGVtcyBtYXRjaGluZyB0aGUgZmlsdGVyIChvcHRpb25hbCwgbWF5IGJlIGV4cGVuc2l2ZSB0byBjb21wdXRlKQAAAAt0b3RhbF9jb3VudAAAAAPoAAAABA==",
        "AAAAAgAAALdTdG9yYWdlIGtleXMgZm9yIGRpZmZlcmVudCBkYXRhIHR5cGVzIGluIHRoZSB1c2VyIG1hbmFnZW1lbnQgY29udHJhY3QuCgpUaGlzIGVudW0gZGVmaW5lcyB0aGUgdmFyaW91cyBrZXlzIHVzZWQgdG8gc3RvcmUgYW5kIHJldHJpZXZlCnVzZXIgZGF0YSBmcm9tIHRoZSBjb250cmFjdCdzIHBlcnNpc3RlbnQgc3RvcmFnZS4AAAAAAAAAAAdEYXRhS2V5AAAAAAwAAAABAAAAQ0tleSBmb3Igc3RvcmluZyBjb21wbGV0ZSB1c2VyIHByb2ZpbGVzOiB1c2VyX2FkZHJlc3MgLT4gVXNlclByb2ZpbGUAAAAAC1VzZXJQcm9maWxlAAAAAAEAAAATAAAAAQAAACxLZXkgZm9yIHN0b3JpbmcgYWRtaW4gZmxhZ3M6IGFkZHJlc3MgLT4gYm9vbAAAAAVBZG1pbgAAAAAAAAEAAAATAAAAAQAAAEdLZXkgZm9yIHN0b3JpbmcgbGlnaHR3ZWlnaHQgdXNlciBwcm9maWxlczogdXNlcl9hZGRyZXNzIC0+IExpZ2h0UHJvZmlsZQAAAAAQVXNlclByb2ZpbGVMaWdodAAAAAEAAAATAAAAAAAAADlLZXkgZm9yIHN0b3JpbmcgdGhlIGxpc3Qgb2YgYWxsIHJlZ2lzdGVyZWQgdXNlciBhZGRyZXNzZXMAAAAAAAAKVXNlcnNJbmRleAAAAAAAAQAAAE1LZXkgZm9yIGVtYWlsIHRvIGFkZHJlc3MgbWFwcGluZyB0byBlbnN1cmUgZW1haWwgdW5pcXVlbmVzczogZW1haWwgLT4gQWRkcmVzcwAAAAAAAApFbWFpbEluZGV4AAAAAAABAAAAEAAAAAAAAAArS2V5IGZvciBzdG9yaW5nIHRoZSBsaXN0IG9mIGFkbWluIGFkZHJlc3NlcwAAAAAGQWRtaW5zAAAAAAABAAAAP0tleSBmb3Igc3RvcmluZyB1c2VyIHJvbGUgYXNzaWdubWVudHM6IHVzZXJfYWRkcmVzcyAtPiBVc2VyUm9sZQAAAAAIVXNlclJvbGUAAAABAAAAEwAAAAAAAAAsS2V5IGZvciBzdG9yaW5nIGFkbWluaXN0cmF0aXZlIGNvbmZpZ3VyYXRpb24AAAALQWRtaW5Db25maWcAAAAAAQAAAEhLZXkgZm9yIHN0b3JpbmcgcmF0ZSBsaW1pdGluZyBkYXRhIHBlciBhZGRyZXNzOiBhZGRyZXNzIC0+IFJhdGVMaW1pdERhdGEAAAAJUmF0ZUxpbWl0AAAAAAAAAQAAABMAAAABAAAAP0tleSBmb3Igc3RvcmluZyByb2xlLWJhc2VkIHBlcm1pc3Npb25zOiByb2xlIC0+IFJvbGVQZXJtaXNzaW9ucwAAAAAPUm9sZVBlcm1pc3Npb25zAAAAAAEAAAfQAAAACFVzZXJSb2xlAAAAAQAAAFNLZXkgZm9yIHN0b3JpbmcgdXNlci1zcGVjaWZpYyBwZXJtaXNzaW9uIG92ZXJyaWRlczogdXNlcl9hZGRyZXNzIC0+IFVzZXJQZXJtaXNzaW9ucwAAAAAPVXNlclBlcm1pc3Npb25zAAAAAAEAAAATAAAAAAAAADZLZXkgZm9yIHN0b3JpbmcgZGVmYXVsdCByb2xlIHBlcm1pc3Npb25zIGNvbmZpZ3VyYXRpb24AAAAAABZEZWZhdWx0Um9sZVBlcm1pc3Npb25zAAA=",
        "AAAAAAAAA7RSZXRyaWV2ZSBhIHVzZXIgcHJvZmlsZSBmb3IgdGhlIGF1dGhlbnRpY2F0ZWQgdXNlci4KClRoaXMgZnVuY3Rpb24gZmV0Y2hlcyB0aGUgY29tcGxldGUgdXNlciBwcm9maWxlIGFzc29jaWF0ZWQgd2l0aCB0aGUgcHJvdmlkZWQKYmxvY2tjaGFpbiBhZGRyZXNzLiBUaGUgdXNlciBtdXN0IGJlIGF1dGhlbnRpY2F0ZWQ7IG90aGVyd2lzZSwgdGhlIGZ1bmN0aW9uCndpbGwgcGFuaWMuCgojIyMgQXJndW1lbnRzCgoqIGBlbnZgIC0gVGhlIFNvcm9iYW4gZW52aXJvbm1lbnQuCiogYHVzZXJgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIHVzZXIgd2hvc2UgcHJvZmlsZSBpcyBiZWluZyByZXF1ZXN0ZWQuCgojIyMgUmV0dXJucwoKUmV0dXJucyB0aGUgYFVzZXJQcm9maWxlYCBjb3JyZXNwb25kaW5nIHRvIHRoZSBhdXRoZW50aWNhdGVkIHVzZXIuCgojIyMgUGFuaWNzCgoqIElmIHRoZSB1c2VyIGlzIG5vdCBhdXRoZW50aWNhdGVkIChgcmVxdWlyZV9hdXRoYCBmYWlscykuCiogSWYgdGhlIHVzZXIgcHJvZmlsZSBkb2VzIG5vdCBleGlzdCAoYFVzZXJOb3RGb3VuZGAgZXJyb3IpLgoKIyMjIEV4YW1wbGVzCgpgYGBydXN0Ci8vIEFzc3VtaW5nIHRoZSB1c2VyIGlzIGF1dGhlbnRpY2F0ZWQgaW4gdGhlIGVudmlyb25tZW50CmxldCBwcm9maWxlID0gY29udHJhY3QuZ2V0X3VzZXJfcHJvZmlsZShlbnYuY2xvbmUoKSwgbXlfYWRkcmVzcyk7CnByaW50bG4hKCJVc2VyIGZ1bGwgbmFtZToge30iLCBwcm9maWxlLmZ1bGxfbmFtZSk7CmBgYAoKIyMjIE5vdGVzCgoqIE9ubHkgdGhlIHVzZXIgdGhlbXNlbHZlcyBjYW4gZmV0Y2ggdGhlaXIgcHJvZmlsZTsgdGhlcmUgaXMgbm8gYWRtaW4gb3ZlcnJpZGUKaW4gdGhpcyBmdW5jdGlvbi4KKiBJZiB0aGUgcHJvZmlsZSBpcyBub3QgZm91bmQgaW4gc3RvcmFnZSwgdGhlIGZ1bmN0aW9uIHdpbGwgcGFuaWMgd2l0aApgVXNlck5vdEZvdW5kYC4AAAAQZ2V0X3VzZXJfcHJvZmlsZQAAAAEAAAAAAAAABHVzZXIAAAATAAAAAQAAA+kAAAfQAAAAC1VzZXJQcm9maWxlAAAAAAM=",
        "AAAAAAAABABSZXRyaWV2ZSBhIHVzZXIgcHJvZmlsZSBieSB0aGVpciBhZGRyZXNzLgoKVGhpcyBmdW5jdGlvbiBmZXRjaGVzIGEgY29tcGxldGUgdXNlciBwcm9maWxlIHVzaW5nIHRoZSB1c2VyJ3MgYmxvY2tjaGFpbiBhZGRyZXNzLgpBY2Nlc3MgbWF5IGJlIHJlc3RyaWN0ZWQgYmFzZWQgb24gdGhlIHJlcXVlc3RlcidzIHBlcm1pc3Npb25zLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGByZXF1ZXN0ZXJgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIHVzZXIgcmVxdWVzdGluZyB0aGUgcHJvZmlsZQoqIGB1c2VyX2lkYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSB1c2VyIHdob3NlIHByb2ZpbGUgaXMgYmVpbmcgcmVxdWVzdGVkCgojIFJldHVybnMKClJldHVybnMgdGhlIHJlcXVlc3RlZCBgVXNlclByb2ZpbGVgLgoKIyBQYW5pY3MKCiogSWYgdGhlIHVzZXIgcHJvZmlsZSBkb2Vzbid0IGV4aXN0CiogSWYgdGhlIHJlcXVlc3RlciBkb2Vzbid0IGhhdmUgcGVybWlzc2lvbiB0byB2aWV3IHRoZSBwcm9maWxlCiogSWYgdGhlIHJlcXVlc3RlciBpcyBub3QgdGhlIHVzZXIgdGhlbXNlbHZlcyBvciBhbiBhZG1pbgoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBHZXQgeW91ciBvd24gcHJvZmlsZQpsZXQgbXlfcHJvZmlsZSA9IGNvbnRyYWN0LmdldF91c2VyX2J5X2lkKGVudi5jbG9uZSgpLCBteV9hZGRyZXNzLCBteV9hZGRyZXNzKTsKCi8vIEFkbWluIGdldHRpbmcgYW55IHVzZXIncyBwcm9maWxlCmxldCB1c2VyX3Byb2ZpbGUgPSBjb250cmFjdC5nZXRfdXNlcl9ieV9pZChlbnYuY2xvbmUoKSwgYWRtaW5fYWRkcmVzcywgdXNlcl9hZGRyZXNzKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipOb24tZXhpc3RlbnQgdXNlcioqOiBXaWxsIHBhbmljIHdpdGggYXBwcm9wcmlhdGUgZXJyb3IgbWVzc2FnZQoqICoqSW5hY3RpdmUgdXNlcioqOiBSZXR1cm5zIHByb2ZpbGUgYnV0IHN0YXR1cyB3aWxsIGJlIGBVc2VyU3RhdHVzOjpJbmFjdGl2ZWAKKiAqKlBlcm1pc3Npb24gZGVuaWVkKio6AAAADmdldF91c2VyX2J5X2lkAAAAAAACAAAAAAAAAAlyZXF1ZXN0ZXIAAAAAAAATAAAAAAAAAAd1c2VyX2lkAAAAABMAAAABAAAH0AAAAAtVc2VyUHJvZmlsZQA=",
        "AAAAAAAABABDcmVhdGUgYSBuZXcgdXNlciBwcm9maWxlCgpDcmVhdGVzIGEgbmV3IHVzZXIgcHJvZmlsZSB1c2luZyBhIFVzZXJQcm9maWxlIHN0cnVjdC4KVmFsaWRhdGVzIG1hbmRhdG9yeSBmaWVsZHMgKGZ1bGxfbmFtZSBhbmQgY29udGFjdF9lbWFpbCkgYW5kIHNhdmVzIHRoZSBwcm9maWxlLgoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgdXNlcmAgLSBBZGRyZXNzIG9mIHRoZSB1c2VyIHdob3NlIHByb2ZpbGUgaXMgYmVpbmcgY3JlYXRlZAoqIGBwcm9maWxlYCAtIFVzZXJQcm9maWxlIHN0cnVjdCBjb250YWluaW5nIGFsbCBwcm9maWxlIGRhdGEKCiMgUmV0dXJucwoqIGBVc2VyUHJvZmlsZWAgLSBUaGUgY3JlYXRlZCB1c2VyIHByb2ZpbGUKCiMgUGFuaWNzCiogSWYgbWFuZGF0b3J5IGZpZWxkcyAoZnVsbF9uYW1lLCBjb250YWN0X2VtYWlsKSBhcmUgbWlzc2luZwoqIElmIHVzZXIgcHJvZmlsZSBhbHJlYWR5IGV4aXN0cwoqIElmIGVtYWlsIGZvcm1hdCBpcyBpbnZhbGlkCiogSWYgdmFsaWRhdGlvbiBydWxlcyBhcmUgdmlvbGF0ZWQKCiMgRXZlbnRzCkVtaXRzIGEgdXNlciBjcmVhdGlvbiBldmVudCB1cG9uIHN1Y2Nlc3NmdWwgY3JlYXRpb24KCiMgRXhhbXBsZXMKCmBgYHJ1c3QKbGV0IHByb2ZpbGUgPSBVc2VyUHJvZmlsZSB7CmZ1bGxfbmFtZTogIkpvaG4gRG9lIi50cnlfaW50bygpLnVud3JhcCgpLApjb250YWN0X2VtYWlsOiAiam9obkBleGFtcGxlLmNvbSIudHJ5X2ludG8oKS51bndyYXAoKSwKcm9sZTogVXNlclJvbGU6OlN0dWRlbnQsCnN0YXR1czogVXNlclN0YXR1czo6QWN0aXZlLApjb3VudHJ5OiBTb21lKCJVUyIudHJ5X2ludG8oKS51bndyYXAoKSksCi4uRGVmYXVsdDo6ZGVmYXVsdCgpCn07CgpsZXQgY3JlYXRlZF9wcm9maWxlID0gY29udHJhY3QuY3JlYXRlX3VzZXJfcHJvZmlsZShlbnYsIHVzZXJfYWRkcmVzcywgcHJvZmlsZSk7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqRHVwbGljYXRlIHByb2ZpbGUqKjogV2lsbCBwYW5pYyBpZiB1c2VyIGFsAAAAE2NyZWF0ZV91c2VyX3Byb2ZpbGUAAAAAAgAAAAAAAAAEdXNlcgAAABMAAAAAAAAAB3Byb2ZpbGUAAAAH0AAAAAtVc2VyUHJvZmlsZQAAAAABAAAH0AAAAAtVc2VyUHJvZmlsZQA=",
        "AAAAAAAABABFZGl0IGFuIGV4aXN0aW5nIHVzZXIgcHJvZmlsZQoKVXBkYXRlcyBhbiBleGlzdGluZyB1c2VyIHByb2ZpbGUgd2l0aCBuZXcgdmFsdWVzIGZvciBhbGxvd2VkIGZpZWxkcy4KT25seSB0aGUgdXNlciB0aGVtc2VsdmVzIG9yIGFkbWluaXN0cmF0b3JzIGNhbiBwZXJmb3JtIHVwZGF0ZXMuCkVtYWlsIGFuZCByb2xlIGZpZWxkcyBjYW5ub3QgYmUgdXBkYXRlZCB0aHJvdWdoIHRoaXMgZnVuY3Rpb24uCgojIEFyZ3VtZW50cwoqIGBlbnZgIC0gU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjYWxsZXJgIC0gQWRkcmVzcyBvZiB0aGUgdXNlciBwZXJmb3JtaW5nIHRoZSB1cGRhdGUKKiBgdXNlcl9pZGAgLSBBZGRyZXNzIG9mIHRoZSB1c2VyIHdob3NlIHByb2ZpbGUgaXMgYmVpbmcgdXBkYXRlZAoqIGB1cGRhdGVzYCAtIFByb2ZpbGVVcGRhdGVQYXJhbXMgY29udGFpbmluZyBmaWVsZHMgdG8gdXBkYXRlCgojIFJldHVybnMKKiBgVXNlclByb2ZpbGVgIC0gVGhlIHVwZGF0ZWQgdXNlciBwcm9maWxlCgojIFBhbmljcwoqIElmIGNhbGxlciBhdXRoZW50aWNhdGlvbiBmYWlscwoqIElmIHVzZXIgcHJvZmlsZSBkb2Vzbid0IGV4aXN0CiogSWYgY2FsbGVyIGxhY2tzIHBlcm1pc3Npb24gdG8gZWRpdAoqIElmIGFueSBmaWVsZCB2YWxpZGF0aW9uIGZhaWxzCiogSWYgdXNlciBpcyBpbmFjdGl2ZQoKIyBFdmVudHMKRW1pdHMgYSB1c2VyIHVwZGF0ZSBldmVudCB1cG9uIHN1Y2Nlc3NmdWwgcHJvZmlsZSB1cGRhdGUKCiMgRXhhbXBsZXMKCmBgYHJ1c3QKbGV0IHVwZGF0ZXMgPSBQcm9maWxlVXBkYXRlUGFyYW1zIHsKZnVsbF9uYW1lOiBTb21lKCJKYW5lIERvZSIudHJ5X2ludG8oKS51bndyYXAoKSksCmNvdW50cnk6IFNvbWUoIkNBIi50cnlfaW50bygpLnVud3JhcCgpKSwKYmlvOiBTb21lKCJVcGRhdGVkIGJpbyIudHJ5X2ludG8oKS51bndyYXAoKSksCi4uRGVmYXVsdDo6ZGVmYXVsdCgpCn07CgpsZXQgdXBkYXRlZF9wcm9maWxlID0gY29udHJhY3QuZWRpdF91c2VyX3Byb2ZpbGUoZW52LCBjYWxsZXJfYWRkcmVzAAAAEWVkaXRfdXNlcl9wcm9maWxlAAAAAAAAAwAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAAd1c2VyX2lkAAAAABMAAAAAAAAAB3VwZGF0ZXMAAAAH0AAAABNQcm9maWxlVXBkYXRlUGFyYW1zAAAAAAEAAAfQAAAAC1VzZXJQcm9maWxlAA==",
        "AAAAAAAAAzZDaGVjayBpZiBhbiBhZGRyZXNzIGhhcyBhZG1pbiBwcml2aWxlZ2VzLgoKVGhpcyBmdW5jdGlvbiBpcyB1c2VkIGJ5IG90aGVyIGNvbnRyYWN0cyB0byB2ZXJpZnkgYWRtaW4gc3RhdHVzCmZvciBjcm9zcy1jb250cmFjdCBhdXRob3JpemF0aW9uIGNoZWNrcy4KCiMgQXJndW1lbnRzCgoqIGBlbnZgIC0gVGhlIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgd2hvYCAtIFRoZSBhZGRyZXNzIHRvIGNoZWNrIGZvciBhZG1pbiBwcml2aWxlZ2VzCgojIFJldHVybnMKClJldHVybnMgYHRydWVgIGlmIHRoZSBhZGRyZXNzIGhhcyBhZG1pbiBwcml2aWxlZ2VzLCBgZmFsc2VgIG90aGVyd2lzZS4KCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gQ2hlY2sgaWYgdXNlciBpcyBhZG1pbgpsZXQgaXNfYWRtaW4gPSBjb250cmFjdC5pc19hZG1pbihlbnYuY2xvbmUoKSwgdXNlcl9hZGRyZXNzKTsKaWYgaXNfYWRtaW4gewovLyBQZXJmb3JtIGFkbWluIG9wZXJhdGlvbnMKfQoKLy8gQ3Jvc3MtY29udHJhY3QgYWRtaW4gY2hlY2sKbGV0IGNhbl9wZXJmb3JtX2FjdGlvbiA9IGNvbnRyYWN0LmlzX2FkbWluKGVudi5jbG9uZSgpLCBjYWxsZXJfYWRkcmVzcyk7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqU3lzdGVtIG5vdCBpbml0aWFsaXplZCoqOiBSZXR1cm5zIGBmYWxzZWAgaWYgYWRtaW4gc3lzdGVtIGhhc24ndCBiZWVuIHNldCB1cAoqICoqTm9uLWV4aXN0ZW50IHVzZXIqKjogUmV0dXJucyBgZmFsc2VgIGZvciBhZGRyZXNzZXMgdGhhdCBkb24ndCBleGlzdAoqICoqUmVndWxhciB1c2VycyoqOiBBbHdheXMgcmV0dXJucyBgZmFsc2VgIGZvciBub24tYWRtaW4gdXNlcnMAAAAAAAhpc19hZG1pbgAAAAEAAAAAAAAAA3dobwAAAAATAAAAAQAAAAE=",
        "AAAAAAAABABEZWxldGUgKGRlYWN0aXZhdGUpIGEgdXNlciBhY2NvdW50CgpQZXJmb3JtcyBhIHNvZnQgZGVsZXRlIGJ5IG1hcmtpbmcgdGhlIHVzZXIgYXMgaW5hY3RpdmUgaW5zdGVhZCBvZiBwZXJtYW5lbnQgZGVsZXRpb24uCk9ubHkgYWRtaW5zIG9yIHRoZSB1c2VyIHRoZW1zZWx2ZXMgY2FuIHRyaWdnZXIgZGVsZXRpb24uCgojIEFyZ3VtZW50cwoqIGBlbnZgIC0gU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjYWxsZXJgIC0gQWRkcmVzcyBwZXJmb3JtaW5nIHRoZSBkZWxldGlvbiAobXVzdCBiZSBhZG1pbiBvciB0aGUgdXNlciB0aGVtc2VsdmVzKQoqIGB1c2VyX2lkYCAtIEFkZHJlc3Mgb2YgdGhlIHVzZXIgdG8gYmUgZGVhY3RpdmF0ZWQKCiMgUGFuaWNzCiogSWYgY2FsbGVyIGF1dGhlbnRpY2F0aW9uIGZhaWxzCiogSWYgdXNlciBkb2Vzbid0IGV4aXN0CiogSWYgY2FsbGVyIGlzIG5laXRoZXIgYWRtaW4gbm9yIHRoZSB1c2VyIHRoZW1zZWx2ZXMKKiBJZiB1c2VyIGlzIGFscmVhZHkgaW5hY3RpdmUKCiMgRXZlbnRzCkVtaXRzIGEgdXNlciBkZWFjdGl2YXRpb24gZXZlbnQgdXBvbiBzdWNjZXNzZnVsIGRlbGV0aW9uCgojIEV4YW1wbGVzCgpgYGBydXN0Ci8vIFVzZXIgZGVsZXRpbmcgdGhlaXIgb3duIGFjY291bnQKY29udHJhY3QuZGVsZXRlX3VzZXIoZW52LmNsb25lKCksIHVzZXJfYWRkcmVzcywgdXNlcl9hZGRyZXNzKTsKCi8vIEFkbWluIGRlbGV0aW5nIGFub3RoZXIgdXNlcidzIGFjY291bnQKY29udHJhY3QuZGVsZXRlX3VzZXIoZW52LmNsb25lKCksIGFkbWluX2FkZHJlc3MsIHVzZXJfdG9fZGVsZXRlKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipBbHJlYWR5IGluYWN0aXZlKio6IFdpbGwgcGFuaWMgaWYgdHJ5aW5nIHRvIGRlbGV0ZSBhbiBhbHJlYWR5IGluYWN0aXZlIHVzZXIKKiAqKlBlcm1pc3Npb24gZGVuaWVkKio6IE5vbi1hZG1pbiB1c2VycyBjYW4gb25seSBkZWxldGUgdGhlaXIgb3duIGFjY291bnRzCiogKipEYXRhIHByZXNlcnZhdGlvbioqOiBVc2VyIGRhdGEgaXMgcHJlc2VydmVkAAAAC2RlbGV0ZV91c2VyAAAAAAIAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAAHdXNlcl9pZAAAAAATAAAAAA==",
        "AAAAAAAABABMaXN0cyBhbGwgcmVnaXN0ZXJlZCB1c2VycyB3aXRoIHBhZ2luYXRpb24gYW5kIGZpbHRlcmluZyAoYWRtaW4tb25seSkKCiMgQXJndW1lbnRzCiogYGVudmAgLSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNhbGxlcmAgLSBBZGRyZXNzIHBlcmZvcm1pbmcgdGhlIGNhbGwgKG11c3QgYmUgYWRtaW4pCiogYHBhZ2VgIC0gWmVyby1iYXNlZCBwYWdlIGluZGV4CiogYHBhZ2Vfc2l6ZWAgLSBOdW1iZXIgb2YgaXRlbXMgcGVyIHBhZ2UgKG11c3QgYmUgPiAwKQoqIGByb2xlX2ZpbHRlcmAgLSBPcHRpb25hbCByb2xlIGZpbHRlcgoqIGBjb3VudHJ5X2ZpbHRlcmAgLSBPcHRpb25hbCBjb3VudHJ5IGZpbHRlcgoqIGBzdGF0dXNfZmlsdGVyYCAtIE9wdGlvbmFsIHN0YXR1cyBmaWx0ZXIKCiMgUmV0dXJucwoqIGBWZWM8TGlnaHRQcm9maWxlPmAgLSBGaWx0ZXJlZCBhbmQgcGFnaW5hdGVkIGxpZ2h0d2VpZ2h0IHVzZXIgcHJvZmlsZXMKCiMgUGFuaWNzCiogSWYgY2FsbGVyIGlzIG5vdCBhbiBhZG1pbgoqIElmIHBhZ2Vfc2l6ZSBpcyAwIG9yIGV4Y2VlZHMgbWF4aW11bSBhbGxvd2VkCiogSWYgc3lzdGVtIGlzIG5vdCBpbml0aWFsaXplZAoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBHZXQgZmlyc3QgcGFnZSB3aXRoIDEwIHVzZXJzCmxldCB1c2VycyA9IGNvbnRyYWN0Lmxpc3RfYWxsX3VzZXJzKAplbnYuY2xvbmUoKSwKYWRtaW5fYWRkcmVzcywKMCwgIC8vIHBhZ2UgMAoxMCwgLy8gcGFnZSBzaXplCk5vbmUsIE5vbmUsIE5vbmUgLy8gbm8gZmlsdGVycwopOwoKLy8gRmlsdGVyIGJ5IHJvbGUgYW5kIGNvdW50cnkKbGV0IHN0dWRlbnRzID0gY29udHJhY3QubGlzdF9hbGxfdXNlcnMoCmVudi5jbG9uZSgpLAphZG1pbl9hZGRyZXNzLAowLCAyMCwKU29tZShVc2VyUm9sZTo6U3R1ZGVudCksClNvbWUoIlVTIi50cnlfaW50bygpLnVud3JhcCgpKSwKTm9uZQopOwpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKkVtcHR5IHJlc3VsdHMqKjogUmV0dXJucyBlbXB0eSB2ZWN0b3IgaWYgbm8gdXNlcnMgbWF0Y2ggZmlsdGVyAAAADmxpc3RfYWxsX3VzZXJzAAAAAAAGAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAABHBhZ2UAAAAEAAAAAAAAAAlwYWdlX3NpemUAAAAAAAAEAAAAAAAAAAtyb2xlX2ZpbHRlcgAAAAPoAAAH0AAAAAhVc2VyUm9sZQAAAAAAAAAOY291bnRyeV9maWx0ZXIAAAAAA+gAAAAQAAAAAAAAAA1zdGF0dXNfZmlsdGVyAAAAAAAD6AAAB9AAAAAKVXNlclN0YXR1cwAAAAAAAQAAA+oAAAfQAAAADExpZ2h0UHJvZmlsZQ==",
        "AAAAAAAAAldMaXN0cyBhbGwgcmVnaXN0ZXJlZCB1c2VycyB3aXRoIGFkdmFuY2VkIGZpbHRlcmluZyBpbmNsdWRpbmcgdGV4dCBzZWFyY2ggKGFkbWluLW9ubHkpLgoKVGhpcyBpcyB0aGUgbmV3IHZlcnNpb24gdGhhdCBzdXBwb3J0cyB0ZXh0IHNlYXJjaCBmdW5jdGlvbmFsaXR5LgoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgY2FsbGVyYCAtIEFkZHJlc3MgcGVyZm9ybWluZyB0aGUgY2FsbCAobXVzdCBiZSBhZG1pbikKKiBgcGFnZWAgLSBaZXJvLWJhc2VkIHBhZ2UgaW5kZXgKKiBgcGFnZV9zaXplYCAtIE51bWJlciBvZiBpdGVtcyBwZXIgcGFnZQoqIGByb2xlX2ZpbHRlcmAgLSBPcHRpb25hbCByb2xlIGZpbHRlcgoqIGBjb3VudHJ5X2ZpbHRlcmAgLSBPcHRpb25hbCBjb3VudHJ5IGZpbHRlcgoqIGBzdGF0dXNfZmlsdGVyYCAtIE9wdGlvbmFsIHN0YXR1cyBmaWx0ZXIKKiBgc2VhcmNoX3RleHRgIC0gT3B0aW9uYWwgdGV4dCBzZWFyY2ggaW4gbmFtZSBhbmQgcHJvZmVzc2lvbgoKIyBSZXR1cm5zCiogYFZlYzxMaWdodFByb2ZpbGU+YCAtIEZpbHRlcmVkIGFuZCBwYWdpbmF0ZWQgbGlnaHR3ZWlnaHQgdXNlciBwcm9maWxlcwAAAAAXbGlzdF9hbGxfdXNlcnNfYWR2YW5jZWQAAAAABwAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAARwYWdlAAAABAAAAAAAAAAJcGFnZV9zaXplAAAAAAAABAAAAAAAAAALcm9sZV9maWx0ZXIAAAAD6AAAB9AAAAAIVXNlclJvbGUAAAAAAAAADmNvdW50cnlfZmlsdGVyAAAAAAPoAAAAEAAAAAAAAAANc3RhdHVzX2ZpbHRlcgAAAAAAA+gAAAfQAAAAClVzZXJTdGF0dXMAAAAAAAAAAAALc2VhcmNoX3RleHQAAAAD6AAAABAAAAABAAAD6gAAB9AAAAAMTGlnaHRQcm9maWxl",
        "AAAAAAAAAmxMaXN0cyBhbGwgcmVnaXN0ZXJlZCB1c2VycyB3aXRoIGN1cnNvci1iYXNlZCBwYWdpbmF0aW9uIGFuZCBmaWx0ZXJpbmcgKGFkbWluLW9ubHkpCgpUaGlzIGZ1bmN0aW9uIGltcGxlbWVudHMgZWZmaWNpZW50IGN1cnNvci1iYXNlZCBwYWdpbmF0aW9uIHRvIGF2b2lkIGdhcyBsaW1pdCBpc3N1ZXMKd2hlbiBkZWFsaW5nIHdpdGggbGFyZ2UgZGF0YXNldHMuIEl0IHJldHVybnMgYSBQYWdpbmF0ZWRSZXN1bHQgd2l0aCBtZXRhZGF0YSBmb3IKZWZmaWNpZW50IG5hdmlnYXRpb24uCgojIEFyZ3VtZW50cwoqIGBlbnZgIC0gU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjYWxsZXJgIC0gQWRkcmVzcyBwZXJmb3JtaW5nIHRoZSBjYWxsIChtdXN0IGJlIGFkbWluKQoqIGBwYWdpbmF0aW9uYCAtIFBhZ2luYXRpb24gcGFyYW1ldGVycyBpbmNsdWRpbmcgY3Vyc29yIGFuZCBsaW1pdAoqIGByb2xlX2ZpbHRlcmAgLSBPcHRpb25hbCBmaWx0ZXIgZm9yIHVzZXIgcm9sZQoqIGBzdGF0dXNfZmlsdGVyYCAtIE9wdGlvbmFsIGZpbHRlciBmb3IgdXNlciBzdGF0dXMKCiMgUmV0dXJucwoqIGBQYWdpbmF0ZWRMaWdodFByb2ZpbGVzYCAtIFBhZ2luYXRlZCByZXN1bHRzIHdpdGggbmF2aWdhdGlvbiBtZXRhZGF0YQAAABVsaXN0X2FsbF91c2Vyc19jdXJzb3IAAAAAAAAEAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAACnBhZ2luYXRpb24AAAAAB9AAAAAQUGFnaW5hdGlvblBhcmFtcwAAAAAAAAALcm9sZV9maWx0ZXIAAAAD6AAAB9AAAAAIVXNlclJvbGUAAAAAAAAADXN0YXR1c19maWx0ZXIAAAAAAAPoAAAH0AAAAApVc2VyU3RhdHVzAAAAAAABAAAH0AAAABZQYWdpbmF0ZWRMaWdodFByb2ZpbGVzAAA=",
        "AAAAAAAAA7BJbml0aWFsaXplIHRoZSBhZG1pbiBzeXN0ZW0gKG9uZS10aW1lIG9ubHkpCgojIEFyZ3VtZW50cwoqIGBlbnZgIC0gU29yb2JhbiBlbnZpcm9ubWVudAoqIGBpbml0aWFsaXplcmAgLSBBZGRyZXNzIHBlcmZvcm1pbmcgdGhlIGluaXRpYWxpemF0aW9uCiogYHN1cGVyX2FkbWluYCAtIEFkZHJlc3MgdGhhdCB3aWxsIGJlY29tZSB0aGUgc3VwZXIgYWRtaW4KKiBgbWF4X3BhZ2Vfc2l6ZWAgLSBPcHRpb25hbCBtYXhpbXVtIHBhZ2Ugc2l6ZSAoZGVmYXVsdDogMTAwLCBtYXg6IDEwMDApCgojIFJldHVybnMKKiBgQWRtaW5Db25maWdgIC0gVGhlIGNyZWF0ZWQgYWRtaW4gY29uZmlndXJhdGlvbgoKIyBQYW5pY3MKKiBJZiBzeXN0ZW0gaGFzIGFscmVhZHkgYmVlbiBpbml0aWFsaXplZAoqIElmIG1heF9wYWdlX3NpemUgZXhjZWVkcyAxMDAwCgojIEV4YW1wbGVzCgpgYGBydXN0Ci8vIEluaXRpYWxpemUgd2l0aCBkZWZhdWx0IHNldHRpbmdzCmxldCBjb25maWcgPSBjb250cmFjdC5pbml0aWFsaXplX3N5c3RlbSgKZW52LmNsb25lKCksCmRlcGxveWVyX2FkZHJlc3MsCnN1cGVyX2FkbWluX2FkZHJlc3MsCk5vbmUKKTsKCi8vIEluaXRpYWxpemUgd2l0aCBjdXN0b20gcGFnZSBzaXplCmxldCBjb25maWcgPSBjb250cmFjdC5pbml0aWFsaXplX3N5c3RlbSgKZW52LmNsb25lKCksCmRlcGxveWVyX2FkZHJlc3MsCnN1cGVyX2FkbWluX2FkZHJlc3MsClNvbWUoNTAwKQopOwpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKkRvdWJsZSBpbml0aWFsaXphdGlvbioqOiBXaWxsIHBhbmljIGlmIGNhbGxlZCBtb3JlIHRoYW4gb25jZQoqICoqSW52YWxpZCBwYWdlIHNpemUqKjogV2lsbCBwYW5pYyBpZiBtYXhfcGFnZV9zaXplID4gMTAwMAoqICoqU3VwZXIgYWRtaW4gcHJpdmlsZWdlcyoqOiBTdXBlciBhZG1pbiBjYW5ub3QgYmUgcmVtb3ZlZCBhZnRlciBpbml0aWFsaXphdGlvbgAAABFpbml0aWFsaXplX3N5c3RlbQAAAAAAAAMAAAAAAAAAC2luaXRpYWxpemVyAAAAABMAAAAAAAAAC3N1cGVyX2FkbWluAAAAABMAAAAAAAAADW1heF9wYWdlX3NpemUAAAAAAAPoAAAABAAAAAEAAAfQAAAAC0FkbWluQ29uZmlnAA==",
        "AAAAAAAAAoNBZGQgYSBuZXcgYWRtaW4gKHN1cGVyIGFkbWluIG9ubHkpCgojIEFyZ3VtZW50cwoqIGBlbnZgIC0gU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjYWxsZXJgIC0gQWRkcmVzcyBwZXJmb3JtaW5nIHRoZSBjYWxsIChtdXN0IGJlIHN1cGVyIGFkbWluKQoqIGBuZXdfYWRtaW5gIC0gQWRkcmVzcyB0byBiZSBhZGRlZCBhcyBhZG1pbgoKIyBQYW5pY3MKKiBJZiBjYWxsZXIgaXMgbm90IHRoZSBzdXBlciBhZG1pbgoqIElmIHN5c3RlbSBpcyBub3QgaW5pdGlhbGl6ZWQKKiBJZiBuZXdfYWRtaW4gaXMgYWxyZWFkeSBhbiBhZG1pbgoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBTdXBlciBhZG1pbiBhZGRpbmcgYSBuZXcgYWRtaW4KY29udHJhY3QuYWRkX2FkbWluKGVudi5jbG9uZSgpLCBzdXBlcl9hZG1pbl9hZGRyZXNzLCBuZXdfYWRtaW5fYWRkcmVzcyk7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqQWxyZWFkeSBhZG1pbioqOiBXaWxsIHBhbmljIGlmIHRyeWluZyB0byBhZGQgYW4gZXhpc3RpbmcgYWRtaW4KKiAqKlNlbGYtcHJvbW90aW9uKio6IFN1cGVyIGFkbWluIGNhbm5vdCBhZGQgdGhlbXNlbHZlcyAocmVkdW5kYW50KQoqICoqTm9uLWV4aXN0ZW50IHVzZXIqKjogQ2FuIGFkZCBhZG1pbiBwcml2aWxlZ2VzIHRvIGFueSBhZGRyZXNzAAAAAAlhZGRfYWRtaW4AAAAAAAACAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAACW5ld19hZG1pbgAAAAAAABMAAAAA",
        "AAAAAAAAAqxSZW1vdmUgYW4gYWRtaW4gKHN1cGVyIGFkbWluIG9ubHkpCgojIEFyZ3VtZW50cwoqIGBlbnZgIC0gU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjYWxsZXJgIC0gQWRkcmVzcyBwZXJmb3JtaW5nIHRoZSBjYWxsIChtdXN0IGJlIHN1cGVyIGFkbWluKQoqIGBhZG1pbl90b19yZW1vdmVgIC0gQWRkcmVzcyB0byBiZSByZW1vdmVkIGZyb20gYWRtaW5zCgojIFBhbmljcwoqIElmIGNhbGxlciBpcyBub3QgdGhlIHN1cGVyIGFkbWluCiogSWYgc3lzdGVtIGlzIG5vdCBpbml0aWFsaXplZAoqIElmIGFkbWluX3RvX3JlbW92ZSBpcyBub3QgYW4gYWRtaW4KKiBJZiB0cnlpbmcgdG8gcmVtb3ZlIHRoZSBzdXBlciBhZG1pbgoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBTdXBlciBhZG1pbiByZW1vdmluZyBhbm90aGVyIGFkbWluCmNvbnRyYWN0LnJlbW92ZV9hZG1pbihlbnYuY2xvbmUoKSwgc3VwZXJfYWRtaW5fYWRkcmVzcywgYWRtaW5fdG9fcmVtb3ZlKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipTdXBlciBhZG1pbiBwcm90ZWN0aW9uKio6IENhbm5vdCByZW1vdmUgdGhlIHN1cGVyIGFkbWluCiogKipOb24tYWRtaW4qKjogV2lsbCBwYW5pYyBpZiB0cnlpbmcgdG8gcmVtb3ZlIGEgbm9uLWFkbWluIGFkZHJlc3MKKiAqKlNlbGYtcmVtb3ZhbCoqOiBTdXBlciBhZG1pbiBjYW5ub3QgcmVtb3ZlIHRoZW1zZWx2ZXMAAAAMcmVtb3ZlX2FkbWluAAAAAgAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAA9hZG1pbl90b19yZW1vdmUAAAAAEwAAAAA=",
        "AAAAAAAAApFHZXQgbGlzdCBvZiBhbGwgYWRtaW5zIChhZG1pbiBvbmx5KQoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgY2FsbGVyYCAtIEFkZHJlc3MgcGVyZm9ybWluZyB0aGUgY2FsbCAobXVzdCBiZSBhZG1pbikKCiMgUmV0dXJucwoqIGBWZWM8QWRkcmVzcz5gIC0gTGlzdCBvZiBhbGwgYWRtaW4gYWRkcmVzc2VzIGluY2x1ZGluZyBzdXBlciBhZG1pbgoKIyBQYW5pY3MKKiBJZiBjYWxsZXIgaXMgbm90IGFuIGFkbWluCiogSWYgc3lzdGVtIGlzIG5vdCBpbml0aWFsaXplZAoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBHZXQgYWxsIGFkbWluIGFkZHJlc3NlcwpsZXQgYWRtaW5zID0gY29udHJhY3QuZ2V0X2FkbWlucyhlbnYuY2xvbmUoKSwgYWRtaW5fYWRkcmVzcyk7CmZvciBhZG1pbiBpbiBhZG1pbnMgewovLyBQcm9jZXNzIGVhY2ggYWRtaW4gYWRkcmVzcwp9CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqRW1wdHkgbGlzdCoqOiBSZXR1cm5zIHZlY3RvciB3aXRoIG9ubHkgc3VwZXIgYWRtaW4gaWYgbm8gb3RoZXIgYWRtaW5zIGV4aXN0CiogKipBZG1pbiBvbmx5Kio6IE9ubHkgYWRtaW5zIGNhbiB2aWV3IHRoZSBhZG1pbiBsaXN0CiogKipPcmRlcioqOiBTdXBlciBhZG1pbiBpcyB0eXBpY2FsbHkgZmlyc3QgaW4gdGhlIGxpc3QAAAAAAAAKZ2V0X2FkbWlucwAAAAAAAQAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAQAAA+oAAAAT",
        "AAAAAAAAAkBDaGVjayBpZiB0aGUgc3lzdGVtIGlzIGluaXRpYWxpemVkCgojIEFyZ3VtZW50cwoqIGBlbnZgIC0gU29yb2JhbiBlbnZpcm9ubWVudAoKIyBSZXR1cm5zCiogYGJvb2xgIC0gVHJ1ZSBpZiBzeXN0ZW0gaXMgaW5pdGlhbGl6ZWQKCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gQ2hlY2sgaWYgYWRtaW4gc3lzdGVtIGlzIHJlYWR5CmxldCBpc19pbml0aWFsaXplZCA9IGNvbnRyYWN0LmlzX3N5c3RlbV9pbml0aWFsaXplZChlbnYuY2xvbmUoKSk7CmlmICFpc19pbml0aWFsaXplZCB7Ci8vIEluaXRpYWxpemUgdGhlIHN5c3RlbSBmaXJzdApjb250cmFjdC5pbml0aWFsaXplX3N5c3RlbShlbnYsIGRlcGxveWVyLCBzdXBlcl9hZG1pbiwgTm9uZSk7Cn0KYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipGcmVzaCBkZXBsb3ltZW50Kio6IFJldHVybnMgYGZhbHNlYCBmb3IgbmV3bHkgZGVwbG95ZWQgY29udHJhY3RzCiogKipQdWJsaWMgYWNjZXNzKio6IEFueW9uZSBjYW4gY2hlY2sgaW5pdGlhbGl6YXRpb24gc3RhdHVzCiogKipPbmUtdGltZSBjaGVjayoqOiBPbmNlIGluaXRpYWxpemVkLCBhbHdheXMgcmV0dXJucyBgdHJ1ZWAAAAAVaXNfc3lzdGVtX2luaXRpYWxpemVkAAAAAAAAAAAAAAEAAAAB",
        "AAAAAAAAARFHZXQgdGhlIGN1cnJlbnQgY29udHJhY3QgdmVyc2lvbgoKUmV0dXJucyB0aGUgc2VtYW50aWMgdmVyc2lvbiBvZiB0aGUgY3VycmVudCBjb250cmFjdCBkZXBsb3ltZW50LgpUaGlzIGlzIHVzZWZ1bCBmb3IgdHJhY2tpbmcgY29udHJhY3QgdXBncmFkZXMgYW5kIGNvbXBhdGliaWxpdHkuCgojIEFyZ3VtZW50cwoqIGBfZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50ICh1bnVzZWQpCgojIFJldHVybnMKKiBgU3RyaW5nYCAtIFRoZSBjdXJyZW50IGNvbnRyYWN0IHZlcnNpb24AAAAAAAAUZ2V0X2NvbnRyYWN0X3ZlcnNpb24AAAAAAAAAAQAAABA=",
        "AAAAAAAAAapFeHBvcnQgYWxsIHVzZXIgZGF0YSBmb3IgYmFja3VwIHB1cnBvc2VzIChhZG1pbiBvbmx5KQoKVGhpcyBmdW5jdGlvbiBleHBvcnRzIGFsbCB1c2VyIHByb2ZpbGVzIGFuZCBhZG1pbmlzdHJhdGl2ZSBkYXRhCmZvciBiYWNrdXAgYW5kIHJlY292ZXJ5IHB1cnBvc2VzLiBPbmx5IGFkbWlucyBjYW4gcGVyZm9ybSB0aGlzIG9wZXJhdGlvbi4KCiMgQXJndW1lbnRzCiogYGVudmAgLSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNhbGxlcmAgLSBBZGRyZXNzIHBlcmZvcm1pbmcgdGhlIGV4cG9ydCAobXVzdCBiZSBhZG1pbikKCiMgUmV0dXJucwoqIGBVc2VyQmFja3VwRGF0YWAgLSBDb21wbGV0ZSBiYWNrdXAgZGF0YSBzdHJ1Y3R1cmUKCiMgUGFuaWNzCiogSWYgY2FsbGVyIGlzIG5vdCBhbiBhZG1pbgoqIElmIHN5c3RlbSBpcyBub3QgaW5pdGlhbGl6ZWQAAAAAABBleHBvcnRfdXNlcl9kYXRhAAAAAQAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAQAAB9AAAAAOVXNlckJhY2t1cERhdGEAAA==",
        "AAAAAAAAAdJJbXBvcnQgdXNlciBkYXRhIGZyb20gYmFja3VwIChhZG1pbiBvbmx5KQoKVGhpcyBmdW5jdGlvbiBpbXBvcnRzIHVzZXIgZGF0YSBmcm9tIGEgYmFja3VwIHN0cnVjdHVyZS4KT25seSBhZG1pbnMgY2FuIHBlcmZvcm0gdGhpcyBvcGVyYXRpb24uIFRoaXMgd2lsbCBvdmVyd3JpdGUgZXhpc3RpbmcgZGF0YS4KCiMgQXJndW1lbnRzCiogYGVudmAgLSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNhbGxlcmAgLSBBZGRyZXNzIHBlcmZvcm1pbmcgdGhlIGltcG9ydCAobXVzdCBiZSBhZG1pbikKKiBgYmFja3VwX2RhdGFgIC0gQmFja3VwIGRhdGEgc3RydWN0dXJlIHRvIGltcG9ydAoKIyBSZXR1cm5zCiogYHUzMmAgLSBOdW1iZXIgb2YgdXNlcnMgaW1wb3J0ZWQKCiMgUGFuaWNzCiogSWYgY2FsbGVyIGlzIG5vdCBhbiBhZG1pbgoqIElmIGJhY2t1cCBkYXRhIGlzIGludmFsaWQKKiBJZiBpbXBvcnQgb3BlcmF0aW9uIGZhaWxzAAAAAAAQaW1wb3J0X3VzZXJfZGF0YQAAAAIAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAALYmFja3VwX2RhdGEAAAAH0AAAAA5Vc2VyQmFja3VwRGF0YQAAAAAAAQAAAAQ=" ]),
      options
    )
  }
  public readonly fromJSON = {
    get_user_profile: this.txFromJSON<Result<UserProfile>>,
        get_user_by_id: this.txFromJSON<UserProfile>,
        create_user_profile: this.txFromJSON<UserProfile>,
        edit_user_profile: this.txFromJSON<UserProfile>,
        is_admin: this.txFromJSON<boolean>,
        delete_user: this.txFromJSON<null>,
        list_all_users: this.txFromJSON<Array<LightProfile>>,
        list_all_users_advanced: this.txFromJSON<Array<LightProfile>>,
        list_all_users_cursor: this.txFromJSON<PaginatedLightProfiles>,
        initialize_system: this.txFromJSON<AdminConfig>,
        add_admin: this.txFromJSON<null>,
        remove_admin: this.txFromJSON<null>,
        get_admins: this.txFromJSON<Array<string>>,
        is_system_initialized: this.txFromJSON<boolean>,
        get_contract_version: this.txFromJSON<string>,
        export_user_data: this.txFromJSON<UserBackupData>,
        import_user_data: this.txFromJSON<u32>
  }
}
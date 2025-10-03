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
  1: {message:"UserAlreadyHasAccess"},
  2: {message:"UserNoAccessCourse"},
  3: {message:"Unauthorized"},
  4: {message:"NameRequired"},
  5: {message:"EmailRequired"},
  6: {message:"CountryRequired"},
  7: {message:"InvalidCourseId"},
  8: {message:"InvalidUser"},
  9: {message:"EmptyCourseId"},
  10: {message:"InvalidTransferData"},
  11: {message:"SameUserTransfer"},
  12: {message:"Initialized"}
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


/**
 * Represents access permission for a user to a specific course.
 * 
 * This struct defines the relationship between a user and a course
 * they have been granted access to.
 */
export interface CourseAccess {
  /**
 * The unique identifier of the course
 */
course_id: string;
  /**
 * The address of the user who has access
 */
user: string;
}


/**
 * Contains all courses that a specific user has access to.
 * 
 * This struct is used to efficiently query and return all courses
 * accessible by a particular user.
 */
export interface UserCourses {
  /**
 * List of course IDs the user has access to
 */
courses: Array<string>;
  /**
 * The address of the user
 */
user: string;
}

/**
 * Storage keys for different data types in the contract.
 * 
 * This enum defines the various keys used to store and retrieve
 * data from the contract's persistent storage.
 */
export type DataKey = {tag: "CourseAccess", values: readonly [string, string]} | {tag: "UserProfile", values: readonly [string]} | {tag: "UserCourses", values: readonly [string]} | {tag: "CourseUsers", values: readonly [string]};


/**
 * Represents a user's profile information.
 * 
 * This struct contains all the personal and professional information
 * that users can store on-chain as part of their profile.
 */
export interface UserProfile {
  /**
 * The user's country of residence
 */
country: string;
  /**
 * The user's email address
 */
email: string;
  /**
 * Optional learning goals or objectives
 */
goals: Option<string>;
  /**
 * The user's full name
 */
name: string;
  /**
 * Optional profession or job title
 */
profession: Option<string>;
}


/**
 * Contains all users who have access to a specific course.
 * 
 * This struct is used to efficiently query and return all users
 * who have been granted access to a particular course.
 */
export interface CourseUsers {
  /**
 * The unique identifier of the course
 */
course: string;
  /**
 * List of user addresses who have access to the course
 */
users: Array<string>;
}

export interface Client {
  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * One-time constructor to set owner and config addresses.
   * 
   * Initializes the contract with the necessary external contract addresses.
   * This function can only be called once during contract deployment.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `caller` - The address of the contract deployer/owner
   * * `user_mgmt_addr` - Address of the user management contract
   * * `course_registry_addr` - Address of the course registry contract
   * 
   * # Panics
   * 
   * * Fails if the contract has already been initialized
   * * If any of the provided addresses are invalid
   * 
   * # Examples
   * 
   * ```rust
   * // Initialize contract during deployment
   * contract.initialize(
   * env.clone(),
   * deployer_address,
   * user_mgmt_contract_address,
   * course_registry_contract_address
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Double initialization**: Will panic if called more than once
   * * **Invalid addresses**: Contract addresses must be valid
   * * **Deployment only**: Should only be called during contract deployment
   */
  initialize: ({caller, user_mgmt_addr, course_registry_addr}: {caller: string, user_mgmt_addr: string, course_registry_addr: string}, options?: {
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
   * Construct and simulate a grant_access transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Grant access to a specific user for a given course.
   * 
   * Allows a user to access a specific course. Only authorized users
   * (course creators or admins) can grant access.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `course_id` - The unique identifier of the course
   * * `user` - The address of the user to grant access to
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If caller is not authorized (not course creator or admin)
   * * If user already has access
   * 
   * # Examples
   * 
   * ```rust
   * // Course creator granting access
   * contract.grant_access(
   * env.clone(),
   * "course_123".try_into().unwrap(),
   * student_address
   * );
   * 
   * // Admin granting access
   * contract.grant_access(
   * env.clone(),
   * "course_456".try_into().unwrap(),
   * student_address
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Already has access**: Will panic if user already has access
   * * **Non-existent course**: Will panic if course doesn't exist
   * * **Permission denied**: Only course creators and admins can grant access
   * * **User validation**: User address must be valid
   */
  grant_access: ({course_id, user}: {course_id: string, user: string}, options?: {
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
   * Construct and simulate a revoke_access transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Revoke access for a specific user from a course.
   * 
   * Removes a user's access to a specific course. Only authorized users
   * (course creators or admins) can revoke access.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `course_id` - The unique identifier of the course
   * * `user` - The address of the user to revoke access from
   * 
   * # Returns
   * 
   * Returns `true` if access was successfully revoked, `false` otherwise.
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If caller is not authorized (not course creator or admin)
   * 
   * # Examples
   * 
   * ```rust
   * // Revoke access from a user
   * let success = contract.revoke_access(
   * env.clone(),
   * "course_123".try_into().unwrap(),
   * student_address
   * );
   * 
   * if success {
   * println!("Access revoked successfully");
   * } else {
   * println!("User didn't have access");
   * }
   * ```
   * 
   * # Edge Cases
   * 
   * * **No access to revoke**: Returns `false` if user didn't have access
   * * **Non-existent course**: Will panic if course doesn't exist
   * * **Permission denied**: Only course creators and admins can revoke access
   * * **Idempotent**: Safe to call multiple 
   */
  revoke_access: ({course_id, user}: {course_id: string, user: string}, options?: {
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
   * Construct and simulate a save_user_profile transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Save or update a user's profile on-chain.
   * 
   * Stores user profile information in the contract storage.
   * This includes personal and professional information.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `name` - The user's full name
   * * `email` - The user's email address
   * * `profession` - Optional profession/job title
   * * `goals` - Optional learning goals or objectives
   * * `country` - The user's country of residence
   * 
   * # Panics
   * 
   * * If name, email, or country are empty
   * * If email format is invalid
   * 
   * # Examples
   * 
   * ```rust
   * // Save user profile
   * contract.save_user_profile(
   * env.clone(),
   * "John Doe".try_into().unwrap(),
   * "john@example.com".try_into().unwrap(),
   * Some("Software Developer".try_into().unwrap()),
   * Some("Learn Rust programming".try_into().unwrap()),
   * "US".try_into().unwrap()
   * );
   * 
   * // Save minimal profile
   * contract.save_user_profile(
   * env.clone(),
   * "Jane Smith".try_into().unwrap(),
   * "jane@example.com".try_into().unwrap(),
   * None,
   * None,
   * "CA".try_into().unwrap()
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Empty required fields**: Name, email, and coun
   */
  save_user_profile: ({name, email, profession, goals, country}: {name: string, email: string, profession: Option<string>, goals: Option<string>, country: string}, options?: {
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
   * Construct and simulate a list_user_courses transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * List all courses a user has access to.
   * 
   * Retrieves all courses that the specified user is enrolled in
   * or has been granted access to.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `user` - The address of the user to query
   * 
   * # Returns
   * 
   * Returns a `UserCourses` struct containing the list of accessible courses.
   * 
   * # Examples
   * 
   * ```rust
   * // Get user's accessible courses
   * let user_courses = contract.list_user_courses(env.clone(), user_address);
   * 
   * for course_id in user_courses.course_ids {
   * println!("User has access to course: {}", course_id);
   * }
   * ```
   * 
   * # Edge Cases
   * 
   * * **No access**: Returns empty list if user has no course access
   * * **Non-existent user**: Returns empty list for non-existent users
   * * **Public access**: Anyone can query user courses
   * * **Revoked courses**: Only includes currently accessible courses
   */
  list_user_courses: ({user}: {user: string}, options?: {
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
  }) => Promise<AssembledTransaction<UserCourses>>

  /**
   * Construct and simulate a list_course_access transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * List all users who have access to a course.
   * 
   * Retrieves all users who have been granted access to the specified course.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `course_id` - The unique identifier of the course
   * 
   * # Returns
   * 
   * Returns a `CourseUsers` struct containing the list of users with access.
   * 
   * # Examples
   * 
   * ```rust
   * // Get all users with access to a course
   * let course_users = contract.list_course_access(env.clone(), "course_123".try_into().unwrap());
   * 
   * for user in course_users.users {
   * println!("User with access: {}", user);
   * }
   * ```
   * 
   * # Edge Cases
   * 
   * * **No users**: Returns empty list if no users have access
   * * **Non-existent course**: Returns empty list for non-existent courses
   * * **Public access**: Anyone can query course access
   * * **Real-time data**: Always returns current access status
   */
  list_course_access: ({course_id}: {course_id: string}, options?: {
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
  }) => Promise<AssembledTransaction<CourseUsers>>

  /**
   * Construct and simulate a revoke_all_access transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Revoke all user access for a course.
   * 
   * Removes access for all users from the specified course.
   * Only admin or course creator is allowed to perform this operation.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `user` - The address of the user requesting the operation
   * * `course_id` - The unique identifier of the course
   * 
   * # Returns
   * 
   * Returns the number of users affected by the revocation and emits an event.
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If caller is not authorized (not course creator or admin)
   * 
   * # Examples
   * 
   * ```rust
   * // Revoke all access for a course
   * let affected_users = contract.revoke_all_access(
   * env.clone(),
   * admin_address,
   * "course_123".try_into().unwrap()
   * );
   * 
   * println!("Revoked access for {} users", affected_users);
   * ```
   * 
   * # Edge Cases
   * 
   * * **No users**: Returns 0 if no users had access
   * * **Non-existent course**: Will panic if course doesn't exist
   * * **Permission denied**: Only course creators and admins can perform this
   * * **Bulk operation**: Efficiently removes all access in one transaction
   */
  revoke_all_access: ({user, course_id}: {user: string, course_id: string}, options?: {
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

  /**
   * Construct and simulate a set_config transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Configure external contract addresses used for auth checks.
   * 
   * Updates the addresses of external contracts that this contract
   * depends on for authentication and authorization checks.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `caller` - The address of the user making the configuration change
   * * `user_mgmt_addr` - Address of the user management contract
   * * `course_registry_addr` - Address of the course registry contract
   * 
   * # Panics
   * 
   * * If caller is not the contract owner
   * * If any of the provided addresses are invalid
   * 
   * # Storage
   * 
   * Stores the addresses in instance storage keys: ("user_mgmt_addr",) and ("course_registry_addr",)
   * 
   * # Examples
   * 
   * ```rust
   * // Update contract addresses
   * contract.set_config(
   * env.clone(),
   * contract_owner_address,
   * new_user_mgmt_address,
   * new_course_registry_address
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Owner only**: Only contract owner can update addresses
   * * **Invalid addresses**: Will panic if addresses are invalid
   * * **Runtime updates**: Can be called after contract deployment
   * * **Immediate effect**: Change
   */
  set_config: ({caller, user_mgmt_addr, course_registry_addr}: {caller: string, user_mgmt_addr: string, course_registry_addr: string}, options?: {
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
   * Construct and simulate a get_version_history transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get contract version history
   * 
   * Returns a list of all versions that have been deployed for this contract.
   * This helps track the evolution of the contract over time.
   * 
   * # Arguments
   * * `env` - The Soroban environment
   * 
   * # Returns
   * * `Vec<String>` - Vector of version strings in chronological order
   */
  get_version_history: (options?: {
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
   * Construct and simulate a is_version_compatible transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check compatibility between contract versions
   * 
   * Determines if data from one version can be safely used with another version.
   * This is crucial for migration processes and backward compatibility.
   * 
   * # Arguments
   * * `env` - The Soroban environment
   * * `from_version` - The source version to check compatibility from
   * * `to_version` - The target version to check compatibility to
   * 
   * # Returns
   * * `bool` - True if the versions are compatible, false otherwise
   */
  is_version_compatible: ({from_version, to_version}: {from_version: string, to_version: string}, options?: {
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
   * Construct and simulate a migrate_access_data transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Migrate access data between contract versions
   * 
   * Performs data migration from one contract version to another.
   * This function handles the transformation of course access data structures
   * when upgrading contract versions.
   * 
   * # Arguments
   * * `env` - The Soroban environment
   * * `caller` - The address performing the migration (must be admin)
   * * `from_version` - The source version to migrate from
   * * `to_version` - The target version to migrate to
   * 
   * # Returns
   * * `bool` - True if migration was successful, false otherwise
   * 
   * # Events
   * Emits a migration event upon successful completion
   */
  migrate_access_data: ({caller, from_version, to_version}: {caller: string, from_version: string, to_version: string}, options?: {
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
   * Construct and simulate a get_migration_status transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get migration status for the current contract
   * 
   * Returns information about the current migration status and any
   * pending migrations that need to be completed.
   * 
   * # Arguments
   * * `env` - The Soroban environment
   * 
   * # Returns
   * * `String` - Migration status information
   */
  get_migration_status: (options?: {
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
   * Construct and simulate a transfer_course transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  transfer_course: ({course_id, from, to}: {course_id: string, from: string, to: string}, options?: {
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
      new ContractSpec([ "AAAABAAAAAAAAAAAAAAABUVycm9yAAAAAAAADAAAAAAAAAAUVXNlckFscmVhZHlIYXNBY2Nlc3MAAAABAAAAAAAAABJVc2VyTm9BY2Nlc3NDb3Vyc2UAAAAAAAIAAAAAAAAADFVuYXV0aG9yaXplZAAAAAMAAAAAAAAADE5hbWVSZXF1aXJlZAAAAAQAAAAAAAAADUVtYWlsUmVxdWlyZWQAAAAAAAAFAAAAAAAAAA9Db3VudHJ5UmVxdWlyZWQAAAAABgAAAAAAAAAPSW52YWxpZENvdXJzZUlkAAAAAAcAAAAAAAAAC0ludmFsaWRVc2VyAAAAAAgAAAAAAAAADUVtcHR5Q291cnNlSWQAAAAAAAAJAAAAAAAAABNJbnZhbGlkVHJhbnNmZXJEYXRhAAAAAAoAAAAAAAAAEFNhbWVVc2VyVHJhbnNmZXIAAAALAAAAAAAAAAtJbml0aWFsaXplZAAAAAAM",
        "AAAABAAAADtFcnJvcnMgdGhhdCBjYW4gb2NjdXIgZHVyaW5nIGNvbnRyYWN0IHZlcnNpb25pbmcgb3BlcmF0aW9ucwAAAAAAAAAAD1ZlcnNpb25pbmdFcnJvcgAAAAAGAAAAFkludmFsaWQgdmVyc2lvbiBmb3JtYXQAAAAAAA5JbnZhbGlkVmVyc2lvbgAAAAAAAQAAABxWZXJzaW9uIG5vdCBmb3VuZCBpbiBoaXN0b3J5AAAAD1ZlcnNpb25Ob3RGb3VuZAAAAAACAAAAGE1pZ3JhdGlvbiBub3QgY29tcGF0aWJsZQAAABZNaWdyYXRpb25Ob3RDb21wYXRpYmxlAAAAAAADAAAAG01pZ3JhdGlvbiBhbHJlYWR5IGNvbXBsZXRlZAAAAAAZTWlncmF0aW9uQWxyZWFkeUNvbXBsZXRlZAAAAAAAAAQAAAAeVW5hdXRob3JpemVkIG1pZ3JhdGlvbiBhdHRlbXB0AAAAAAAVVW5hdXRob3JpemVkTWlncmF0aW9uAAAAAAAABQAAABBNaWdyYXRpb24gZmFpbGVkAAAAD01pZ3JhdGlvbkZhaWxlZAAAAAAG",
        "AAAAAQAAAKFSZXByZXNlbnRzIGFjY2VzcyBwZXJtaXNzaW9uIGZvciBhIHVzZXIgdG8gYSBzcGVjaWZpYyBjb3Vyc2UuCgpUaGlzIHN0cnVjdCBkZWZpbmVzIHRoZSByZWxhdGlvbnNoaXAgYmV0d2VlbiBhIHVzZXIgYW5kIGEgY291cnNlCnRoZXkgaGF2ZSBiZWVuIGdyYW50ZWQgYWNjZXNzIHRvLgAAAAAAAAAAAAAMQ291cnNlQWNjZXNzAAAAAgAAACNUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZQAAAAAJY291cnNlX2lkAAAAAAAAEAAAACZUaGUgYWRkcmVzcyBvZiB0aGUgdXNlciB3aG8gaGFzIGFjY2VzcwAAAAAABHVzZXIAAAAT",
        "AAAAAQAAAJpDb250YWlucyBhbGwgY291cnNlcyB0aGF0IGEgc3BlY2lmaWMgdXNlciBoYXMgYWNjZXNzIHRvLgoKVGhpcyBzdHJ1Y3QgaXMgdXNlZCB0byBlZmZpY2llbnRseSBxdWVyeSBhbmQgcmV0dXJuIGFsbCBjb3Vyc2VzCmFjY2Vzc2libGUgYnkgYSBwYXJ0aWN1bGFyIHVzZXIuAAAAAAAAAAAAC1VzZXJDb3Vyc2VzAAAAAAIAAAApTGlzdCBvZiBjb3Vyc2UgSURzIHRoZSB1c2VyIGhhcyBhY2Nlc3MgdG8AAAAAAAAHY291cnNlcwAAAAPqAAAAEAAAABdUaGUgYWRkcmVzcyBvZiB0aGUgdXNlcgAAAAAEdXNlcgAAABM=",
        "AAAAAgAAAKJTdG9yYWdlIGtleXMgZm9yIGRpZmZlcmVudCBkYXRhIHR5cGVzIGluIHRoZSBjb250cmFjdC4KClRoaXMgZW51bSBkZWZpbmVzIHRoZSB2YXJpb3VzIGtleXMgdXNlZCB0byBzdG9yZSBhbmQgcmV0cmlldmUKZGF0YSBmcm9tIHRoZSBjb250cmFjdCdzIHBlcnNpc3RlbnQgc3RvcmFnZS4AAAAAAAAAAAAHRGF0YUtleQAAAAAEAAAAAQAAAEBLZXkgZm9yIHN0b3JpbmcgY291cnNlIGFjY2VzczogKGNvdXJzZV9pZCwgdXNlcikgLT4gQ291cnNlQWNjZXNzAAAADENvdXJzZUFjY2VzcwAAAAIAAAAQAAAAEwAAAAEAAAAxS2V5IGZvciBzdG9yaW5nIHVzZXIgcHJvZmlsZTogdXNlciAtPiBVc2VyUHJvZmlsZQAAAAAAAAtVc2VyUHJvZmlsZQAAAAABAAAAEwAAAAEAAAA1S2V5IGZvciBzdG9yaW5nIGNvdXJzZXMgcGVyIHVzZXI6IHVzZXIgLT4gVXNlckNvdXJzZXMAAAAAAAALVXNlckNvdXJzZXMAAAAAAQAAABMAAAABAAAAOktleSBmb3Igc3RvcmluZyB1c2VycyBwZXIgY291cnNlOiBjb3Vyc2VfaWQgLT4gQ291cnNlVXNlcnMAAAAAAAtDb3Vyc2VVc2VycwAAAAABAAAAEA==",
        "AAAAAQAAAKRSZXByZXNlbnRzIGEgdXNlcidzIHByb2ZpbGUgaW5mb3JtYXRpb24uCgpUaGlzIHN0cnVjdCBjb250YWlucyBhbGwgdGhlIHBlcnNvbmFsIGFuZCBwcm9mZXNzaW9uYWwgaW5mb3JtYXRpb24KdGhhdCB1c2VycyBjYW4gc3RvcmUgb24tY2hhaW4gYXMgcGFydCBvZiB0aGVpciBwcm9maWxlLgAAAAAAAAALVXNlclByb2ZpbGUAAAAABQAAAB9UaGUgdXNlcidzIGNvdW50cnkgb2YgcmVzaWRlbmNlAAAAAAdjb3VudHJ5AAAAABAAAAAYVGhlIHVzZXIncyBlbWFpbCBhZGRyZXNzAAAABWVtYWlsAAAAAAAAEAAAACVPcHRpb25hbCBsZWFybmluZyBnb2FscyBvciBvYmplY3RpdmVzAAAAAAAABWdvYWxzAAAAAAAD6AAAABAAAAAUVGhlIHVzZXIncyBmdWxsIG5hbWUAAAAEbmFtZQAAABAAAAAgT3B0aW9uYWwgcHJvZmVzc2lvbiBvciBqb2IgdGl0bGUAAAAKcHJvZmVzc2lvbgAAAAAD6AAAABA=",
        "AAAAAQAAAKxDb250YWlucyBhbGwgdXNlcnMgd2hvIGhhdmUgYWNjZXNzIHRvIGEgc3BlY2lmaWMgY291cnNlLgoKVGhpcyBzdHJ1Y3QgaXMgdXNlZCB0byBlZmZpY2llbnRseSBxdWVyeSBhbmQgcmV0dXJuIGFsbCB1c2Vycwp3aG8gaGF2ZSBiZWVuIGdyYW50ZWQgYWNjZXNzIHRvIGEgcGFydGljdWxhciBjb3Vyc2UuAAAAAAAAAAtDb3Vyc2VVc2VycwAAAAACAAAAI1RoZSB1bmlxdWUgaWRlbnRpZmllciBvZiB0aGUgY291cnNlAAAAAAZjb3Vyc2UAAAAAABAAAAA0TGlzdCBvZiB1c2VyIGFkZHJlc3NlcyB3aG8gaGF2ZSBhY2Nlc3MgdG8gdGhlIGNvdXJzZQAAAAV1c2VycwAAAAAAA+oAAAAT",
        "AAAAAAAAA6JPbmUtdGltZSBjb25zdHJ1Y3RvciB0byBzZXQgb3duZXIgYW5kIGNvbmZpZyBhZGRyZXNzZXMuCgpJbml0aWFsaXplcyB0aGUgY29udHJhY3Qgd2l0aCB0aGUgbmVjZXNzYXJ5IGV4dGVybmFsIGNvbnRyYWN0IGFkZHJlc3Nlcy4KVGhpcyBmdW5jdGlvbiBjYW4gb25seSBiZSBjYWxsZWQgb25jZSBkdXJpbmcgY29udHJhY3QgZGVwbG95bWVudC4KCiMgQXJndW1lbnRzCgoqIGBlbnZgIC0gVGhlIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgY2FsbGVyYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBjb250cmFjdCBkZXBsb3llci9vd25lcgoqIGB1c2VyX21nbXRfYWRkcmAgLSBBZGRyZXNzIG9mIHRoZSB1c2VyIG1hbmFnZW1lbnQgY29udHJhY3QKKiBgY291cnNlX3JlZ2lzdHJ5X2FkZHJgIC0gQWRkcmVzcyBvZiB0aGUgY291cnNlIHJlZ2lzdHJ5IGNvbnRyYWN0CgojIFBhbmljcwoKKiBGYWlscyBpZiB0aGUgY29udHJhY3QgaGFzIGFscmVhZHkgYmVlbiBpbml0aWFsaXplZAoqIElmIGFueSBvZiB0aGUgcHJvdmlkZWQgYWRkcmVzc2VzIGFyZSBpbnZhbGlkCgojIEV4YW1wbGVzCgpgYGBydXN0Ci8vIEluaXRpYWxpemUgY29udHJhY3QgZHVyaW5nIGRlcGxveW1lbnQKY29udHJhY3QuaW5pdGlhbGl6ZSgKZW52LmNsb25lKCksCmRlcGxveWVyX2FkZHJlc3MsCnVzZXJfbWdtdF9jb250cmFjdF9hZGRyZXNzLApjb3Vyc2VfcmVnaXN0cnlfY29udHJhY3RfYWRkcmVzcwopOwpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKkRvdWJsZSBpbml0aWFsaXphdGlvbioqOiBXaWxsIHBhbmljIGlmIGNhbGxlZCBtb3JlIHRoYW4gb25jZQoqICoqSW52YWxpZCBhZGRyZXNzZXMqKjogQ29udHJhY3QgYWRkcmVzc2VzIG11c3QgYmUgdmFsaWQKKiAqKkRlcGxveW1lbnQgb25seSoqOiBTaG91bGQgb25seSBiZSBjYWxsZWQgZHVyaW5nIGNvbnRyYWN0IGRlcGxveW1lbnQAAAAAAAppbml0aWFsaXplAAAAAAADAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAADnVzZXJfbWdtdF9hZGRyAAAAAAATAAAAAAAAABRjb3Vyc2VfcmVnaXN0cnlfYWRkcgAAABMAAAAA",
        "AAAAAAAAA8tHcmFudCBhY2Nlc3MgdG8gYSBzcGVjaWZpYyB1c2VyIGZvciBhIGdpdmVuIGNvdXJzZS4KCkFsbG93cyBhIHVzZXIgdG8gYWNjZXNzIGEgc3BlY2lmaWMgY291cnNlLiBPbmx5IGF1dGhvcml6ZWQgdXNlcnMKKGNvdXJzZSBjcmVhdG9ycyBvciBhZG1pbnMpIGNhbiBncmFudCBhY2Nlc3MuCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZQoqIGB1c2VyYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSB1c2VyIHRvIGdyYW50IGFjY2VzcyB0bwoKIyBQYW5pY3MKCiogSWYgY291cnNlIGRvZXNuJ3QgZXhpc3QKKiBJZiBjYWxsZXIgaXMgbm90IGF1dGhvcml6ZWQgKG5vdCBjb3Vyc2UgY3JlYXRvciBvciBhZG1pbikKKiBJZiB1c2VyIGFscmVhZHkgaGFzIGFjY2VzcwoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBDb3Vyc2UgY3JlYXRvciBncmFudGluZyBhY2Nlc3MKY29udHJhY3QuZ3JhbnRfYWNjZXNzKAplbnYuY2xvbmUoKSwKImNvdXJzZV8xMjMiLnRyeV9pbnRvKCkudW53cmFwKCksCnN0dWRlbnRfYWRkcmVzcwopOwoKLy8gQWRtaW4gZ3JhbnRpbmcgYWNjZXNzCmNvbnRyYWN0LmdyYW50X2FjY2VzcygKZW52LmNsb25lKCksCiJjb3Vyc2VfNDU2Ii50cnlfaW50bygpLnVud3JhcCgpLApzdHVkZW50X2FkZHJlc3MKKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipBbHJlYWR5IGhhcyBhY2Nlc3MqKjogV2lsbCBwYW5pYyBpZiB1c2VyIGFscmVhZHkgaGFzIGFjY2VzcwoqICoqTm9uLWV4aXN0ZW50IGNvdXJzZSoqOiBXaWxsIHBhbmljIGlmIGNvdXJzZSBkb2Vzbid0IGV4aXN0CiogKipQZXJtaXNzaW9uIGRlbmllZCoqOiBPbmx5IGNvdXJzZSBjcmVhdG9ycyBhbmQgYWRtaW5zIGNhbiBncmFudCBhY2Nlc3MKKiAqKlVzZXIgdmFsaWRhdGlvbioqOiBVc2VyIGFkZHJlc3MgbXVzdCBiZSB2YWxpZAAAAAAMZ3JhbnRfYWNjZXNzAAAAAgAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAAAAAAEdXNlcgAAABMAAAAA",
        "AAAAAAAABABSZXZva2UgYWNjZXNzIGZvciBhIHNwZWNpZmljIHVzZXIgZnJvbSBhIGNvdXJzZS4KClJlbW92ZXMgYSB1c2VyJ3MgYWNjZXNzIHRvIGEgc3BlY2lmaWMgY291cnNlLiBPbmx5IGF1dGhvcml6ZWQgdXNlcnMKKGNvdXJzZSBjcmVhdG9ycyBvciBhZG1pbnMpIGNhbiByZXZva2UgYWNjZXNzLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjb3Vyc2VfaWRgIC0gVGhlIHVuaXF1ZSBpZGVudGlmaWVyIG9mIHRoZSBjb3Vyc2UKKiBgdXNlcmAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgdXNlciB0byByZXZva2UgYWNjZXNzIGZyb20KCiMgUmV0dXJucwoKUmV0dXJucyBgdHJ1ZWAgaWYgYWNjZXNzIHdhcyBzdWNjZXNzZnVsbHkgcmV2b2tlZCwgYGZhbHNlYCBvdGhlcndpc2UuCgojIFBhbmljcwoKKiBJZiBjb3Vyc2UgZG9lc24ndCBleGlzdAoqIElmIGNhbGxlciBpcyBub3QgYXV0aG9yaXplZCAobm90IGNvdXJzZSBjcmVhdG9yIG9yIGFkbWluKQoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBSZXZva2UgYWNjZXNzIGZyb20gYSB1c2VyCmxldCBzdWNjZXNzID0gY29udHJhY3QucmV2b2tlX2FjY2VzcygKZW52LmNsb25lKCksCiJjb3Vyc2VfMTIzIi50cnlfaW50bygpLnVud3JhcCgpLApzdHVkZW50X2FkZHJlc3MKKTsKCmlmIHN1Y2Nlc3MgewpwcmludGxuISgiQWNjZXNzIHJldm9rZWQgc3VjY2Vzc2Z1bGx5Iik7Cn0gZWxzZSB7CnByaW50bG4hKCJVc2VyIGRpZG4ndCBoYXZlIGFjY2VzcyIpOwp9CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqTm8gYWNjZXNzIHRvIHJldm9rZSoqOiBSZXR1cm5zIGBmYWxzZWAgaWYgdXNlciBkaWRuJ3QgaGF2ZSBhY2Nlc3MKKiAqKk5vbi1leGlzdGVudCBjb3Vyc2UqKjogV2lsbCBwYW5pYyBpZiBjb3Vyc2UgZG9lc24ndCBleGlzdAoqICoqUGVybWlzc2lvbiBkZW5pZWQqKjogT25seSBjb3Vyc2UgY3JlYXRvcnMgYW5kIGFkbWlucyBjYW4gcmV2b2tlIGFjY2VzcwoqICoqSWRlbXBvdGVudCoqOiBTYWZlIHRvIGNhbGwgbXVsdGlwbGUgAAAADXJldm9rZV9hY2Nlc3MAAAAAAAACAAAAAAAAAAljb3Vyc2VfaWQAAAAAAAAQAAAAAAAAAAR1c2VyAAAAEwAAAAEAAAAB",
        "AAAAAAAABABTYXZlIG9yIHVwZGF0ZSBhIHVzZXIncyBwcm9maWxlIG9uLWNoYWluLgoKU3RvcmVzIHVzZXIgcHJvZmlsZSBpbmZvcm1hdGlvbiBpbiB0aGUgY29udHJhY3Qgc3RvcmFnZS4KVGhpcyBpbmNsdWRlcyBwZXJzb25hbCBhbmQgcHJvZmVzc2lvbmFsIGluZm9ybWF0aW9uLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGBuYW1lYCAtIFRoZSB1c2VyJ3MgZnVsbCBuYW1lCiogYGVtYWlsYCAtIFRoZSB1c2VyJ3MgZW1haWwgYWRkcmVzcwoqIGBwcm9mZXNzaW9uYCAtIE9wdGlvbmFsIHByb2Zlc3Npb24vam9iIHRpdGxlCiogYGdvYWxzYCAtIE9wdGlvbmFsIGxlYXJuaW5nIGdvYWxzIG9yIG9iamVjdGl2ZXMKKiBgY291bnRyeWAgLSBUaGUgdXNlcidzIGNvdW50cnkgb2YgcmVzaWRlbmNlCgojIFBhbmljcwoKKiBJZiBuYW1lLCBlbWFpbCwgb3IgY291bnRyeSBhcmUgZW1wdHkKKiBJZiBlbWFpbCBmb3JtYXQgaXMgaW52YWxpZAoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBTYXZlIHVzZXIgcHJvZmlsZQpjb250cmFjdC5zYXZlX3VzZXJfcHJvZmlsZSgKZW52LmNsb25lKCksCiJKb2huIERvZSIudHJ5X2ludG8oKS51bndyYXAoKSwKImpvaG5AZXhhbXBsZS5jb20iLnRyeV9pbnRvKCkudW53cmFwKCksClNvbWUoIlNvZnR3YXJlIERldmVsb3BlciIudHJ5X2ludG8oKS51bndyYXAoKSksClNvbWUoIkxlYXJuIFJ1c3QgcHJvZ3JhbW1pbmciLnRyeV9pbnRvKCkudW53cmFwKCkpLAoiVVMiLnRyeV9pbnRvKCkudW53cmFwKCkKKTsKCi8vIFNhdmUgbWluaW1hbCBwcm9maWxlCmNvbnRyYWN0LnNhdmVfdXNlcl9wcm9maWxlKAplbnYuY2xvbmUoKSwKIkphbmUgU21pdGgiLnRyeV9pbnRvKCkudW53cmFwKCksCiJqYW5lQGV4YW1wbGUuY29tIi50cnlfaW50bygpLnVud3JhcCgpLApOb25lLApOb25lLAoiQ0EiLnRyeV9pbnRvKCkudW53cmFwKCkKKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipFbXB0eSByZXF1aXJlZCBmaWVsZHMqKjogTmFtZSwgZW1haWwsIGFuZCBjb3VuAAAAEXNhdmVfdXNlcl9wcm9maWxlAAAAAAAABQAAAAAAAAAEbmFtZQAAABAAAAAAAAAABWVtYWlsAAAAAAAAEAAAAAAAAAAKcHJvZmVzc2lvbgAAAAAD6AAAABAAAAAAAAAABWdvYWxzAAAAAAAD6AAAABAAAAAAAAAAB2NvdW50cnkAAAAAEAAAAAA=",
        "AAAAAAAAAyVMaXN0IGFsbCBjb3Vyc2VzIGEgdXNlciBoYXMgYWNjZXNzIHRvLgoKUmV0cmlldmVzIGFsbCBjb3Vyc2VzIHRoYXQgdGhlIHNwZWNpZmllZCB1c2VyIGlzIGVucm9sbGVkIGluCm9yIGhhcyBiZWVuIGdyYW50ZWQgYWNjZXNzIHRvLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGB1c2VyYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSB1c2VyIHRvIHF1ZXJ5CgojIFJldHVybnMKClJldHVybnMgYSBgVXNlckNvdXJzZXNgIHN0cnVjdCBjb250YWluaW5nIHRoZSBsaXN0IG9mIGFjY2Vzc2libGUgY291cnNlcy4KCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gR2V0IHVzZXIncyBhY2Nlc3NpYmxlIGNvdXJzZXMKbGV0IHVzZXJfY291cnNlcyA9IGNvbnRyYWN0Lmxpc3RfdXNlcl9jb3Vyc2VzKGVudi5jbG9uZSgpLCB1c2VyX2FkZHJlc3MpOwoKZm9yIGNvdXJzZV9pZCBpbiB1c2VyX2NvdXJzZXMuY291cnNlX2lkcyB7CnByaW50bG4hKCJVc2VyIGhhcyBhY2Nlc3MgdG8gY291cnNlOiB7fSIsIGNvdXJzZV9pZCk7Cn0KYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipObyBhY2Nlc3MqKjogUmV0dXJucyBlbXB0eSBsaXN0IGlmIHVzZXIgaGFzIG5vIGNvdXJzZSBhY2Nlc3MKKiAqKk5vbi1leGlzdGVudCB1c2VyKio6IFJldHVybnMgZW1wdHkgbGlzdCBmb3Igbm9uLWV4aXN0ZW50IHVzZXJzCiogKipQdWJsaWMgYWNjZXNzKio6IEFueW9uZSBjYW4gcXVlcnkgdXNlciBjb3Vyc2VzCiogKipSZXZva2VkIGNvdXJzZXMqKjogT25seSBpbmNsdWRlcyBjdXJyZW50bHkgYWNjZXNzaWJsZSBjb3Vyc2VzAAAAAAAAEWxpc3RfdXNlcl9jb3Vyc2VzAAAAAAAAAQAAAAAAAAAEdXNlcgAAABMAAAABAAAH0AAAAAtVc2VyQ291cnNlcwA=",
        "AAAAAAAAAxxMaXN0IGFsbCB1c2VycyB3aG8gaGF2ZSBhY2Nlc3MgdG8gYSBjb3Vyc2UuCgpSZXRyaWV2ZXMgYWxsIHVzZXJzIHdobyBoYXZlIGJlZW4gZ3JhbnRlZCBhY2Nlc3MgdG8gdGhlIHNwZWNpZmllZCBjb3Vyc2UuCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZQoKIyBSZXR1cm5zCgpSZXR1cm5zIGEgYENvdXJzZVVzZXJzYCBzdHJ1Y3QgY29udGFpbmluZyB0aGUgbGlzdCBvZiB1c2VycyB3aXRoIGFjY2Vzcy4KCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gR2V0IGFsbCB1c2VycyB3aXRoIGFjY2VzcyB0byBhIGNvdXJzZQpsZXQgY291cnNlX3VzZXJzID0gY29udHJhY3QubGlzdF9jb3Vyc2VfYWNjZXNzKGVudi5jbG9uZSgpLCAiY291cnNlXzEyMyIudHJ5X2ludG8oKS51bndyYXAoKSk7Cgpmb3IgdXNlciBpbiBjb3Vyc2VfdXNlcnMudXNlcnMgewpwcmludGxuISgiVXNlciB3aXRoIGFjY2Vzczoge30iLCB1c2VyKTsKfQpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKk5vIHVzZXJzKio6IFJldHVybnMgZW1wdHkgbGlzdCBpZiBubyB1c2VycyBoYXZlIGFjY2VzcwoqICoqTm9uLWV4aXN0ZW50IGNvdXJzZSoqOiBSZXR1cm5zIGVtcHR5IGxpc3QgZm9yIG5vbi1leGlzdGVudCBjb3Vyc2VzCiogKipQdWJsaWMgYWNjZXNzKio6IEFueW9uZSBjYW4gcXVlcnkgY291cnNlIGFjY2VzcwoqICoqUmVhbC10aW1lIGRhdGEqKjogQWx3YXlzIHJldHVybnMgY3VycmVudCBhY2Nlc3Mgc3RhdHVzAAAAEmxpc3RfY291cnNlX2FjY2VzcwAAAAAAAQAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAEAAAfQAAAAC0NvdXJzZVVzZXJzAA==",
        "AAAAAAAAA+5SZXZva2UgYWxsIHVzZXIgYWNjZXNzIGZvciBhIGNvdXJzZS4KClJlbW92ZXMgYWNjZXNzIGZvciBhbGwgdXNlcnMgZnJvbSB0aGUgc3BlY2lmaWVkIGNvdXJzZS4KT25seSBhZG1pbiBvciBjb3Vyc2UgY3JlYXRvciBpcyBhbGxvd2VkIHRvIHBlcmZvcm0gdGhpcyBvcGVyYXRpb24uCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYHVzZXJgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIHVzZXIgcmVxdWVzdGluZyB0aGUgb3BlcmF0aW9uCiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZQoKIyBSZXR1cm5zCgpSZXR1cm5zIHRoZSBudW1iZXIgb2YgdXNlcnMgYWZmZWN0ZWQgYnkgdGhlIHJldm9jYXRpb24gYW5kIGVtaXRzIGFuIGV2ZW50LgoKIyBQYW5pY3MKCiogSWYgY291cnNlIGRvZXNuJ3QgZXhpc3QKKiBJZiBjYWxsZXIgaXMgbm90IGF1dGhvcml6ZWQgKG5vdCBjb3Vyc2UgY3JlYXRvciBvciBhZG1pbikKCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gUmV2b2tlIGFsbCBhY2Nlc3MgZm9yIGEgY291cnNlCmxldCBhZmZlY3RlZF91c2VycyA9IGNvbnRyYWN0LnJldm9rZV9hbGxfYWNjZXNzKAplbnYuY2xvbmUoKSwKYWRtaW5fYWRkcmVzcywKImNvdXJzZV8xMjMiLnRyeV9pbnRvKCkudW53cmFwKCkKKTsKCnByaW50bG4hKCJSZXZva2VkIGFjY2VzcyBmb3Ige30gdXNlcnMiLCBhZmZlY3RlZF91c2Vycyk7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqTm8gdXNlcnMqKjogUmV0dXJucyAwIGlmIG5vIHVzZXJzIGhhZCBhY2Nlc3MKKiAqKk5vbi1leGlzdGVudCBjb3Vyc2UqKjogV2lsbCBwYW5pYyBpZiBjb3Vyc2UgZG9lc24ndCBleGlzdAoqICoqUGVybWlzc2lvbiBkZW5pZWQqKjogT25seSBjb3Vyc2UgY3JlYXRvcnMgYW5kIGFkbWlucyBjYW4gcGVyZm9ybSB0aGlzCiogKipCdWxrIG9wZXJhdGlvbioqOiBFZmZpY2llbnRseSByZW1vdmVzIGFsbCBhY2Nlc3MgaW4gb25lIHRyYW5zYWN0aW9uAAAAAAARcmV2b2tlX2FsbF9hY2Nlc3MAAAAAAAACAAAAAAAAAAR1c2VyAAAAEwAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAEAAAAE",
        "AAAAAAAABABDb25maWd1cmUgZXh0ZXJuYWwgY29udHJhY3QgYWRkcmVzc2VzIHVzZWQgZm9yIGF1dGggY2hlY2tzLgoKVXBkYXRlcyB0aGUgYWRkcmVzc2VzIG9mIGV4dGVybmFsIGNvbnRyYWN0cyB0aGF0IHRoaXMgY29udHJhY3QKZGVwZW5kcyBvbiBmb3IgYXV0aGVudGljYXRpb24gYW5kIGF1dGhvcml6YXRpb24gY2hlY2tzLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjYWxsZXJgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIHVzZXIgbWFraW5nIHRoZSBjb25maWd1cmF0aW9uIGNoYW5nZQoqIGB1c2VyX21nbXRfYWRkcmAgLSBBZGRyZXNzIG9mIHRoZSB1c2VyIG1hbmFnZW1lbnQgY29udHJhY3QKKiBgY291cnNlX3JlZ2lzdHJ5X2FkZHJgIC0gQWRkcmVzcyBvZiB0aGUgY291cnNlIHJlZ2lzdHJ5IGNvbnRyYWN0CgojIFBhbmljcwoKKiBJZiBjYWxsZXIgaXMgbm90IHRoZSBjb250cmFjdCBvd25lcgoqIElmIGFueSBvZiB0aGUgcHJvdmlkZWQgYWRkcmVzc2VzIGFyZSBpbnZhbGlkCgojIFN0b3JhZ2UKClN0b3JlcyB0aGUgYWRkcmVzc2VzIGluIGluc3RhbmNlIHN0b3JhZ2Uga2V5czogKCJ1c2VyX21nbXRfYWRkciIsKSBhbmQgKCJjb3Vyc2VfcmVnaXN0cnlfYWRkciIsKQoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBVcGRhdGUgY29udHJhY3QgYWRkcmVzc2VzCmNvbnRyYWN0LnNldF9jb25maWcoCmVudi5jbG9uZSgpLApjb250cmFjdF9vd25lcl9hZGRyZXNzLApuZXdfdXNlcl9tZ210X2FkZHJlc3MsCm5ld19jb3Vyc2VfcmVnaXN0cnlfYWRkcmVzcwopOwpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKk93bmVyIG9ubHkqKjogT25seSBjb250cmFjdCBvd25lciBjYW4gdXBkYXRlIGFkZHJlc3NlcwoqICoqSW52YWxpZCBhZGRyZXNzZXMqKjogV2lsbCBwYW5pYyBpZiBhZGRyZXNzZXMgYXJlIGludmFsaWQKKiAqKlJ1bnRpbWUgdXBkYXRlcyoqOiBDYW4gYmUgY2FsbGVkIGFmdGVyIGNvbnRyYWN0IGRlcGxveW1lbnQKKiAqKkltbWVkaWF0ZSBlZmZlY3QqKjogQ2hhbmdlAAAACnNldF9jb25maWcAAAAAAAMAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAAOdXNlcl9tZ210X2FkZHIAAAAAABMAAAAAAAAAFGNvdXJzZV9yZWdpc3RyeV9hZGRyAAAAEwAAAAA=",
        "AAAAAAAAARFHZXQgdGhlIGN1cnJlbnQgY29udHJhY3QgdmVyc2lvbgoKUmV0dXJucyB0aGUgc2VtYW50aWMgdmVyc2lvbiBvZiB0aGUgY3VycmVudCBjb250cmFjdCBkZXBsb3ltZW50LgpUaGlzIGlzIHVzZWZ1bCBmb3IgdHJhY2tpbmcgY29udHJhY3QgdXBncmFkZXMgYW5kIGNvbXBhdGliaWxpdHkuCgojIEFyZ3VtZW50cwoqIGBfZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50ICh1bnVzZWQpCgojIFJldHVybnMKKiBgU3RyaW5nYCAtIFRoZSBjdXJyZW50IGNvbnRyYWN0IHZlcnNpb24AAAAAAAAUZ2V0X2NvbnRyYWN0X3ZlcnNpb24AAAAAAAAAAQAAABA=",
        "AAAAAAAAAR5HZXQgY29udHJhY3QgdmVyc2lvbiBoaXN0b3J5CgpSZXR1cm5zIGEgbGlzdCBvZiBhbGwgdmVyc2lvbnMgdGhhdCBoYXZlIGJlZW4gZGVwbG95ZWQgZm9yIHRoaXMgY29udHJhY3QuClRoaXMgaGVscHMgdHJhY2sgdGhlIGV2b2x1dGlvbiBvZiB0aGUgY29udHJhY3Qgb3ZlciB0aW1lLgoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CgojIFJldHVybnMKKiBgVmVjPFN0cmluZz5gIC0gVmVjdG9yIG9mIHZlcnNpb24gc3RyaW5ncyBpbiBjaHJvbm9sb2dpY2FsIG9yZGVyAAAAAAATZ2V0X3ZlcnNpb25faGlzdG9yeQAAAAAAAAAAAQAAA+oAAAAQ",
        "AAAAAAAAAblDaGVjayBjb21wYXRpYmlsaXR5IGJldHdlZW4gY29udHJhY3QgdmVyc2lvbnMKCkRldGVybWluZXMgaWYgZGF0YSBmcm9tIG9uZSB2ZXJzaW9uIGNhbiBiZSBzYWZlbHkgdXNlZCB3aXRoIGFub3RoZXIgdmVyc2lvbi4KVGhpcyBpcyBjcnVjaWFsIGZvciBtaWdyYXRpb24gcHJvY2Vzc2VzIGFuZCBiYWNrd2FyZCBjb21wYXRpYmlsaXR5LgoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGZyb21fdmVyc2lvbmAgLSBUaGUgc291cmNlIHZlcnNpb24gdG8gY2hlY2sgY29tcGF0aWJpbGl0eSBmcm9tCiogYHRvX3ZlcnNpb25gIC0gVGhlIHRhcmdldCB2ZXJzaW9uIHRvIGNoZWNrIGNvbXBhdGliaWxpdHkgdG8KCiMgUmV0dXJucwoqIGBib29sYCAtIFRydWUgaWYgdGhlIHZlcnNpb25zIGFyZSBjb21wYXRpYmxlLCBmYWxzZSBvdGhlcndpc2UAAAAAAAAVaXNfdmVyc2lvbl9jb21wYXRpYmxlAAAAAAAAAgAAAAAAAAAMZnJvbV92ZXJzaW9uAAAAEAAAAAAAAAAKdG9fdmVyc2lvbgAAAAAAEAAAAAEAAAAB",
        "AAAAAAAAAjZNaWdyYXRlIGFjY2VzcyBkYXRhIGJldHdlZW4gY29udHJhY3QgdmVyc2lvbnMKClBlcmZvcm1zIGRhdGEgbWlncmF0aW9uIGZyb20gb25lIGNvbnRyYWN0IHZlcnNpb24gdG8gYW5vdGhlci4KVGhpcyBmdW5jdGlvbiBoYW5kbGVzIHRoZSB0cmFuc2Zvcm1hdGlvbiBvZiBjb3Vyc2UgYWNjZXNzIGRhdGEgc3RydWN0dXJlcwp3aGVuIHVwZ3JhZGluZyBjb250cmFjdCB2ZXJzaW9ucy4KCiMgQXJndW1lbnRzCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjYWxsZXJgIC0gVGhlIGFkZHJlc3MgcGVyZm9ybWluZyB0aGUgbWlncmF0aW9uIChtdXN0IGJlIGFkbWluKQoqIGBmcm9tX3ZlcnNpb25gIC0gVGhlIHNvdXJjZSB2ZXJzaW9uIHRvIG1pZ3JhdGUgZnJvbQoqIGB0b192ZXJzaW9uYCAtIFRoZSB0YXJnZXQgdmVyc2lvbiB0byBtaWdyYXRlIHRvCgojIFJldHVybnMKKiBgYm9vbGAgLSBUcnVlIGlmIG1pZ3JhdGlvbiB3YXMgc3VjY2Vzc2Z1bCwgZmFsc2Ugb3RoZXJ3aXNlCgojIEV2ZW50cwpFbWl0cyBhIG1pZ3JhdGlvbiBldmVudCB1cG9uIHN1Y2Nlc3NmdWwgY29tcGxldGlvbgAAAAAAE21pZ3JhdGVfYWNjZXNzX2RhdGEAAAAAAwAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAAxmcm9tX3ZlcnNpb24AAAAQAAAAAAAAAAp0b192ZXJzaW9uAAAAAAAQAAAAAQAAAAE=",
        "AAAAAAAAAP9HZXQgbWlncmF0aW9uIHN0YXR1cyBmb3IgdGhlIGN1cnJlbnQgY29udHJhY3QKClJldHVybnMgaW5mb3JtYXRpb24gYWJvdXQgdGhlIGN1cnJlbnQgbWlncmF0aW9uIHN0YXR1cyBhbmQgYW55CnBlbmRpbmcgbWlncmF0aW9ucyB0aGF0IG5lZWQgdG8gYmUgY29tcGxldGVkLgoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CgojIFJldHVybnMKKiBgU3RyaW5nYCAtIE1pZ3JhdGlvbiBzdGF0dXMgaW5mb3JtYXRpb24AAAAAFGdldF9taWdyYXRpb25fc3RhdHVzAAAAAAAAAAEAAAAQ",
        "AAAAAAAAAAAAAAAPdHJhbnNmZXJfY291cnNlAAAAAAMAAAAAAAAACWNvdXJzZV9pZAAAAAAAABAAAAAAAAAABGZyb20AAAATAAAAAAAAAAJ0bwAAAAAAEwAAAAA=" ]),
      options
    )
  }
  public readonly fromJSON = {
    initialize: this.txFromJSON<null>,
        grant_access: this.txFromJSON<null>,
        revoke_access: this.txFromJSON<boolean>,
        save_user_profile: this.txFromJSON<null>,
        list_user_courses: this.txFromJSON<UserCourses>,
        list_course_access: this.txFromJSON<CourseUsers>,
        revoke_all_access: this.txFromJSON<u32>,
        set_config: this.txFromJSON<null>,
        get_contract_version: this.txFromJSON<string>,
        get_version_history: this.txFromJSON<Array<string>>,
        is_version_compatible: this.txFromJSON<boolean>,
        migrate_access_data: this.txFromJSON<boolean>,
        get_migration_status: this.txFromJSON<string>,
        transfer_course: this.txFromJSON<null>
  }
}
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
  1: {message:"OnlyCreatorCanAddGoals"},
  2: {message:"EmptyGoalContent"},
  3: {message:"CourseIdNotExist"},
  4: {message:"OnlyCreatorCanArchive"},
  5: {message:"CourseAlreadyArchived"},
  6: {message:"Unauthorized"},
  7: {message:"NameRequired"},
  8: {message:"EmptyCourseTitle"},
  9: {message:"InvalidPrice"},
  10: {message:"DuplicateCourseTitle"},
  11: {message:"DuplicateCourseId"},
  12: {message:"OnlyCreatorCanEditPrereqs"},
  13: {message:"PrereqCourseNotFound"},
  14: {message:"SelfPrerequisite"},
  15: {message:"CircularDependency"},
  16: {message:"EmptyCourseId"},
  17: {message:"CourseNotFound"},
  18: {message:"EmptyNewGoalContent"},
  19: {message:"EmptyGoalId"},
  20: {message:"GoalCourseMismatch"},
  21: {message:"ModuleNotFound"},
  401: {message:"UnauthorizedCaller"},
  402: {message:"UnauthorizedCourseAccess"},
  403: {message:"InvalidAdminOperation"},
  404: {message:"EmptyModuleTitle"},
  405: {message:"DuplicateModulePosition"},
  22: {message:"EmptyModuleId"},
  23: {message:"PrereqNotInList"},
  24: {message:"InvalidModulePosition"},
  25: {message:"InvalidModuleTitle"},
  26: {message:"InvalidCourseDescription"},
  27: {message:"InvalidCategoryName"},
  28: {message:"EmptyCategory"},
  29: {message:"InvalidTitleLength"},
  43: {message:"InvalidLanguageLength"},
  44: {message:"InvalidThumbnailUrlLength"},
  45: {message:"InvalidDurationValue"},
  46: {message:"InvalidLimitValue"},
  47: {message:"InvalidOffsetValue"},
  48: {message:"InvalidGoalContent"},
  49: {message:"InvalidPrerequisiteId"},
  50: {message:"EmptyPrerequisiteList"},
  51: {message:"TooManyPrerequisites"},
  52: {message:"EmptyPrerequisiteId"},
  53: {message:"InvalidCourseId"},
  54: {message:"InvalidPrice100"},
  55: {message:"AlreadyInitialized"},
  56: {message:"DuplicatePrerequisite"},
  57: {message:"CourseRateLimitExceeded"},
  58: {message:"CourseRateLimitNotConfigured"}
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


export interface CourseModule {
  course_id: string;
  created_at: u64;
  id: string;
  position: u32;
  title: string;
}


export interface CourseGoal {
  content: string;
  course_id: string;
  created_at: u64;
  created_by: string;
  goal_id: string;
}


/**
 * Rate limiting configuration for course operations.
 * 
 * Tracks rate limiting settings for spam protection in course creation.
 */
export interface CourseRateLimitConfig {
  /**
 * Maximum course creations allowed per window
 */
max_courses_per_window: u32;
  /**
 * Time window for rate limiting in seconds
 */
window_seconds: u64;
}


/**
 * Rate limiting tracking data for course operations per address.
 * 
 * Stores the current usage count and window start time for course rate limiting.
 */
export interface CourseRateLimitData {
  /**
 * Current count of course creations in this window
 */
count: u32;
  /**
 * Timestamp when the current window started
 */
window_start: u64;
}


export interface CourseCategory {
  description: Option<string>;
  id: u128;
  name: string;
}

export type DataKey = {tag: "Module", values: readonly [string]} | {tag: "Courses", values: void} | {tag: "CourseGoalList", values: readonly [string]} | {tag: "CourseGoal", values: readonly [string, string]} | {tag: "CoursePrerequisites", values: readonly [string]} | {tag: "CategorySeq", values: void} | {tag: "CourseCategory", values: readonly [u128]} | {tag: "Admins", values: void} | {tag: "CourseRateLimitConfig", values: void} | {tag: "CourseRateLimit", values: readonly [string]};


export interface Course {
  category: Option<string>;
  creator: string;
  description: string;
  duration_hours: Option<u32>;
  id: string;
  is_archived: boolean;
  language: Option<string>;
  level: Option<CourseLevel>;
  prerequisites: Array<CourseId>;
  price: u128;
  published: boolean;
  thumbnail_url: Option<string>;
  title: string;
}


export interface CourseId {
  count: u128;
  id: string;
}


export interface Category {
  count: u128;
  name: string;
}


export interface CourseFilters {
  category: Option<string>;
  level: Option<CourseLevel>;
  max_duration: Option<u32>;
  max_price: Option<u128>;
  min_duration: Option<u32>;
  min_price: Option<u128>;
  /**
 * Text search in course title and description
 */
search_text: Option<string>;
}


export interface EditCourseParams {
  new_category: Option<Option<string>>;
  new_description: Option<string>;
  new_duration_hours: Option<Option<u32>>;
  new_language: Option<Option<string>>;
  new_level: Option<Option<CourseLevel>>;
  new_price: Option<u128>;
  new_published: Option<boolean>;
  new_thumbnail_url: Option<Option<string>>;
  new_title: Option<string>;
}


/**
 * Backup data structure for course registry system.
 * 
 * Contains all course data, categories, modules, goals, and prerequisites
 * for backup and recovery operations.
 */
export interface CourseBackupData {
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
 * All course categories
 */
categories: Map<u128, CourseCategory>;
  /**
 * Category sequence counter
 */
category_seq: u128;
  /**
 * All courses in the system
 */
courses: Map<string, Course>;
  /**
 * All course goals mapped by (course_id, goal_id)
 */
goals: Map<string, Array<CourseGoal>>;
  /**
 * All course modules
 */
modules: Map<string, CourseModule>;
  /**
 * Course prerequisites mapping
 */
prerequisites: Map<string, Array<CourseId>>;
}

export interface Client {
  /**
   * Construct and simulate a create_course transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Create a new course in the registry.
   * 
   * This function creates a new course with the specified metadata and
   * returns the created course object with a unique identifier.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `creator` - The address of the course creator
   * * `title` - The course title
   * * `description` - The course description
   * * `price` - The course price in the platform's currency
   * * `category` - Optional course category
   * * `language` - Optional course language
   * * `thumbnail_url` - Optional URL for the course thumbnail image
   * * `level` - Optional course difficulty level
   * * `duration_hours` - Optional estimated duration in hours
   * 
   * # Returns
   * 
   * Returns the created `Course` object with all metadata and a unique ID.
   * 
   * # Panics
   * 
   * * If title or description are empty
   * * If creator address is invalid
   * * If price exceeds maximum allowed value
   * 
   * # Examples
   * 
   * ```rust
   * let course = contract.create_course(
   * env.clone(),
   * instructor_address,
   * "Rust Programming Basics".try_into().unwrap(),
   * "Learn Rust from scratch".try_into().unwrap(),
   * 50
   */
  create_course: ({creator, title, description, price, category, language, thumbnail_url, level, duration_hours}: {creator: string, title: string, description: string, price: u128, category: Option<string>, language: Option<string>, thumbnail_url: Option<string>, level: Option<CourseLevel>, duration_hours: Option<u32>}, options?: {
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
  }) => Promise<AssembledTransaction<Course>>

  /**
   * Construct and simulate a create_course_category transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Create a new course category.
   * 
   * This function creates a new category that can be used to classify courses.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `caller` - The address of the user creating the category
   * * `name` - The name of the category
   * * `description` - Optional description of the category
   * 
   * # Returns
   * 
   * Returns the unique ID of the created category.
   * 
   * # Panics
   * 
   * * If category name is empty
   * * If category with same name already exists
   * 
   * # Examples
   * 
   * ```rust
   * // Create a programming category
   * let category_id = contract.create_course_category(
   * env.clone(),
   * admin_address,
   * "Programming".try_into().unwrap(),
   * Some("Computer programming courses".try_into().unwrap())
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Duplicate names**: Cannot create categories with existing names
   * * **Empty names**: Category name cannot be empty
   * * **Unique IDs**: Each category gets a unique auto-generated ID
   */
  create_course_category: ({caller, name, description}: {caller: string, name: string, description: Option<string>}, options?: {
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
  }) => Promise<AssembledTransaction<u128>>

  /**
   * Construct and simulate a get_course transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Retrieve a course by its ID.
   * 
   * This function fetches a course's complete information using its unique identifier.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `course_id` - The unique identifier of the course to retrieve
   * 
   * # Returns
   * 
   * Returns the `Course` object containing all course metadata.
   * 
   * # Panics
   * 
   * * If course with given ID doesn't exist
   * * If course_id is invalid or empty
   * 
   * # Examples
   * 
   * ```rust
   * // Get course by ID
   * let course = contract.get_course(env.clone(), "course_123".try_into().unwrap());
   * println!("Course title: {}", course.title);
   * ```
   * 
   * # Edge Cases
   * 
   * * **Non-existent course**: Will panic if course ID doesn't exist
   * * **Archived courses**: Still retrievable but marked as archived
   * * **Public access**: Anyone can retrieve course information
   */
  get_course: ({course_id}: {course_id: string}, options?: {
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
  }) => Promise<AssembledTransaction<Course>>

  /**
   * Construct and simulate a get_course_category transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Retrieve a course category by its ID.
   * 
   * This function fetches a category's information using its unique identifier.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `category_id` - The unique identifier of the category to retrieve
   * 
   * # Returns
   * 
   * Returns `Some(CourseCategory)` if found, `None` if the category doesn't exist.
   * 
   * # Examples
   * 
   * ```rust
   * // Get category by ID
   * if let Some(category) = contract.get_course_category(env.clone(), 1) {
   * println!("Category: {}", category.name);
   * } else {
   * println!("Category not found");
   * }
   * ```
   * 
   * # Edge Cases
   * 
   * * **Non-existent category**: Returns `None` instead of panicking
   * * **Invalid ID**: Returns `None` for invalid category IDs
   * * **Public access**: Anyone can retrieve category information
   */
  get_course_category: ({category_id}: {category_id: u128}, options?: {
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
  }) => Promise<AssembledTransaction<Option<CourseCategory>>>

  /**
   * Construct and simulate a get_courses_by_instructor transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get all courses created by a specific instructor.
   * 
   * This function retrieves all courses that were created by the specified instructor.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `instructor` - The address of the instructor to query courses for
   * 
   * # Returns
   * 
   * Returns a vector of `Course` objects created by the instructor.
   * 
   * # Examples
   * 
   * ```rust
   * // Get all courses by an instructor
   * let instructor_courses = contract.get_courses_by_instructor(env.clone(), instructor_address);
   * for course in instructor_courses {
   * println!("Course: {}", course.title);
   * }
   * ```
   * 
   * # Edge Cases
   * 
   * * **No courses**: Returns empty vector if instructor has no courses
   * * **Archived courses**: Includes archived courses in results
   * * **Public access**: Anyone can query instructor courses
   * * **Invalid instructor**: Returns empty vector for non-existent instructors
   */
  get_courses_by_instructor: ({instructor}: {instructor: string}, options?: {
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
  }) => Promise<AssembledTransaction<Array<Course>>>

  /**
   * Construct and simulate a remove_module transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Remove a module from a course.
   * 
   * This function removes a specific module from its associated course.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `module_id` - The unique identifier of the module to remove
   * 
   * # Panics
   * 
   * Remove a module from a course.
   * 
   * This function removes a specific module from its associated course.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `module_id` - The unique identifier of the module to remove
   * 
   * # Panics
   * 
   * * If the module doesn't exist
   * * If the module_id is invalid or empty
   * * If module removal operation fails
   * 
   * # Examples
   * 
   * ```rust
   * // Remove a module from a course
   * contract.remove_module(env.clone(), "module_123".try_into().unwrap());
   * ```
   * 
   * # Edge Cases
   * 
   * * **Non-existent module**: Will panic if module ID doesn't exist
   * * **Invalid ID**: Will panic for invalid or empty module IDs
   * * **Course updates**: Automatically updates course module count
   * 
   * Panics if the module removal fails or if the module doesn't exist.
   */
  remove_module: ({module_id}: {module_id: string}, options?: {
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
   * Construct and simulate a add_module transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Add a new module to a course.
   * 
   * This function creates and adds a new module to the specified course
   * at the given position.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `course_id` - The unique identifier of the course to add the module to
   * * `position` - The position where the module should be inserted
   * * `title` - The title of the new module
   * 
   * # Returns
   * 
   * Returns the created `CourseModule` object.
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If caller is not the course creator
   * * If module title is empty
   * * If position is invalid
   * 
   * # Examples
   * 
   * ```rust
   * // Add a module at position 1
   * let module = contract.add_module(
   * env.clone(),
   * course_creator_address,
   * "course_123".try_into().unwrap(),
   * 1,
   * "Introduction to Variables".try_into().unwrap()
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Invalid position**: Position must be valid for the course
   * * **Empty title**: Module title cannot be empty
   * * **Creator only**: Only course creator can add modules
   * * **Auto-generated ID**: Module gets unique auto-generated ID
   */
  add_module: ({caller, course_id, position, title}: {caller: string, course_id: string, position: u32, title: string}, options?: {
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
  }) => Promise<AssembledTransaction<CourseModule>>

  /**
   * Construct and simulate a delete_course transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Delete a course from the registry.
   * 
   * This function permanently removes a course from the registry.
   * Only the course creator can delete their own courses.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `creator` - The address of the course creator
   * * `course_id` - The unique identifier of the course to delete
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If creator is not the actual course creator
   * * If course_id is invalid or empty
   * 
   * # Examples
   * 
   * ```rust
   * // Course creator deleting their course
   * contract.delete_course(env.clone(), course_creator_address, "course_123".try_into().unwrap());
   * ```
   * 
   * # Edge Cases
   * 
   * * **Permission denied**: Only course creator can delete their courses
   * * **Non-existent course**: Will panic if course doesn't exist
   * * **Permanent deletion**: Course and all associated data are permanently removed
   * * **Enrolled students**: Consider impact on enrolled students before deletion
   * 
   * Panics if the deletion fails or if the creator is not authorized.
   */
  delete_course: ({creator, course_id}: {creator: string, course_id: string}, options?: {
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
   * Construct and simulate a hello_world transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Simple hello world function for testing.
   * 
   * This is a basic function that returns a greeting message,
   * primarily used for testing contract deployment and basic functionality.
   * 
   * # Arguments
   * 
   * * `_env` - The Soroban environment (unused)
   * 
   * # Returns
   * 
   * Returns a greeting string.
   * 
   * # Examples
   * 
   * ```rust
   * // Test contract deployment
   * let greeting = contract.hello_world(env.clone());
   * assert_eq!(greeting, "Hello from Web3 👋");
   * ```
   * 
   * # Edge Cases
   * 
   * * **Always succeeds**: This function never fails
   * * **No dependencies**: Requires no external data or state
   * * **Testing only**: Primarily intended for contract testing
   */
  hello_world: (options?: {
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
   * Construct and simulate a edit_goal transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Edit an existing course goal.
   * 
   * This function allows the course creator to modify the content of an existing goal.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `creator` - The address of the course creator
   * * `course_id` - The unique identifier of the course
   * * `goal_id` - The unique identifier of the goal to edit
   * * `new_content` - The new content for the goal
   * 
   * # Returns
   * 
   * Returns the updated `CourseGoal` object.
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If goal doesn't exist
   * * If creator is not the course creator
   * * If new_content is empty
   * 
   * # Examples
   * 
   * ```rust
   * // Edit a course goal
   * let updated_goal = contract.edit_goal(
   * env.clone(),
   * course_creator_address,
   * "course_123".try_into().unwrap(),
   * "goal_456".try_into().unwrap(),
   * "Updated learning objective".try_into().unwrap()
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Empty content**: New content cannot be empty
   * * **Creator only**: Only course creator can edit goals
   * * **Non-existent goal**: Will panic if goal ID doesn't exist
   * * **Content validation**: New content must meet validation 
   */
  edit_goal: ({creator, course_id, goal_id, new_content}: {creator: string, course_id: string, goal_id: string, new_content: string}, options?: {
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
  }) => Promise<AssembledTransaction<CourseGoal>>

  /**
   * Construct and simulate a add_goal transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Add a new goal to a course.
   * 
   * This function creates and adds a new learning goal to the specified course.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `creator` - The address of the course creator
   * * `course_id` - The unique identifier of the course
   * * `content` - The content/description of the goal
   * 
   * # Returns
   * 
   * Returns the created `CourseGoal` object.
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If creator is not the course creator
   * * If content is empty
   * 
   * # Examples
   * 
   * ```rust
   * // Add a learning goal to a course
   * let goal = contract.add_goal(
   * env.clone(),
   * course_creator_address,
   * "course_123".try_into().unwrap(),
   * "Students will learn basic programming concepts".try_into().unwrap()
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Empty content**: Goal content cannot be empty
   * * **Creator only**: Only course creator can add goals
   * * **Auto-generated ID**: Goal gets unique auto-generated ID
   * * **Content validation**: Goal content must meet validation requirements
   */
  add_goal: ({creator, course_id, content}: {creator: string, course_id: string, content: string}, options?: {
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
  }) => Promise<AssembledTransaction<CourseGoal>>

  /**
   * Construct and simulate a remove_goal transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Remove a goal from a course.
   * 
   * This function removes a specific learning goal from the course.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `caller` - The address of the user requesting the removal
   * * `course_id` - The unique identifier of the course
   * * `goal_id` - The unique identifier of the goal to remove
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If goal doesn't exist
   * * If caller is not the course creator
   * 
   * # Examples
   * 
   * ```rust
   * // Remove a goal from a course
   * contract.remove_goal(
   * env.clone(),
   * course_creator_address,
   * "course_123".try_into().unwrap(),
   * "goal_456".try_into().unwrap()
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Creator only**: Only course creator can remove goals
   * * **Non-existent goal**: Will panic if goal ID doesn't exist
   * * **Permanent removal**: Goal is permanently deleted from course
   * * **Goal count**: Automatically updates course goal count
   */
  remove_goal: ({caller, course_id, goal_id}: {caller: string, course_id: string, goal_id: string}, options?: {
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
   * Construct and simulate a add_prerequisite transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Add prerequisites to a course.
   * 
   * This function adds prerequisite courses that must be completed
   * before a student can enroll in the target course.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `creator` - The address of the course creator
   * * `course_id` - The unique identifier of the course
   * * `prerequisite_course_ids` - Vector of course IDs that are prerequisites
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If creator is not the course creator
   * * If any prerequisite course doesn't exist
   * * If trying to add self as prerequisite
   * 
   * # Examples
   * 
   * ```rust
   * let mut prerequisites = Vec::new(&env);
   * prerequisites.push_back("basic_rust".try_into().unwrap());
   * prerequisites.push_back("programming_fundamentals".try_into().unwrap());
   * 
   * contract.add_prerequisite(
   * env.clone(),
   * course_creator_address,
   * "advanced_rust".try_into().unwrap(),
   * prerequisites
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Circular dependencies**: Cannot add self as prerequisite
   * * **Non-existent courses**: All prerequisite courses must exist
   * * **Creator only**: Only course creator ca
   */
  add_prerequisite: ({creator, course_id, prerequisite_course_ids}: {creator: string, course_id: string, prerequisite_course_ids: Array<string>}, options?: {
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
   * Construct and simulate a remove_prerequisite transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Remove a prerequisite from a course.
   * 
   * This function removes a specific prerequisite course requirement
   * from the target course.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `creator` - The address of the course creator
   * * `course_id` - The unique identifier of the course
   * * `prerequisite_course_id` - The ID of the prerequisite course to remove
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If creator is not the course creator
   * * If prerequisite doesn't exist for the course
   * 
   * # Examples
   * 
   * ```rust
   * // Remove a prerequisite from a course
   * contract.remove_prerequisite(
   * env.clone(),
   * course_creator_address,
   * "advanced_rust".try_into().unwrap(),
   * "basic_rust".try_into().unwrap()
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Non-existent prerequisite**: Will panic if prerequisite doesn't exist
   * * **Creator only**: Only course creator can remove prerequisites
   * * **No effect**: Removing non-existent prerequisite has no effect
   * * **Student impact**: Consider impact on enrolled students
   */
  remove_prerequisite: ({creator, course_id, prerequisite_course_id}: {creator: string, course_id: string, prerequisite_course_id: string}, options?: {
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
   * Construct and simulate a edit_prerequisite transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Edit the prerequisites for a course.
   * 
   * This function replaces all existing prerequisites with a new set
   * of prerequisite courses.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `creator` - The address of the course creator
   * * `course_id` - The unique identifier of the course
   * * `new_prerequisites` - Vector of new prerequisite course IDs
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If creator is not the course creator
   * * If any prerequisite course doesn't exist
   * * If trying to add self as prerequisite
   * 
   * # Examples
   * 
   * ```rust
   * let mut new_prerequisites = Vec::new(&env);
   * new_prerequisites.push_back("updated_course_1".try_into().unwrap());
   * new_prerequisites.push_back("updated_course_2".try_into().unwrap());
   * 
   * contract.edit_prerequisite(
   * env.clone(),
   * course_creator_address,
   * "target_course".try_into().unwrap(),
   * new_prerequisites
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Complete replacement**: All old prerequisites are removed
   * * **Empty vector**: Can clear all prerequisites with empty vector
   * * **Circular dependencies**: Cannot add self as prere
   */
  edit_prerequisite: ({creator, course_id, new_prerequisites}: {creator: string, course_id: string, new_prerequisites: Array<string>}, options?: {
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
   * Construct and simulate a edit_course transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Edit course information.
   * 
   * This function allows the course creator to update various aspects
   * of the course using the provided parameters.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `creator` - The address of the course creator
   * * `course_id` - The unique identifier of the course to edit
   * * `params` - Parameters containing the fields to update
   * 
   * # Returns
   * 
   * Returns the updated `Course` object.
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If creator is not the course creator
   * * If any field validation fails
   * 
   * # Examples
   * 
   * ```rust
   * let params = EditCourseParams {
   * title: Some("Updated Course Title".try_into().unwrap()),
   * description: Some("Updated description".try_into().unwrap()),
   * price: Some(7500),
   * level: Some(CourseLevel::Intermediate),
   * ..Default::default()
   * };
   * 
   * let updated_course = contract.edit_course(
   * env.clone(),
   * course_creator_address,
   * "course_123".try_into().unwrap(),
   * params
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Partial updates**: Only provided fields are updated
   * * **Validation**: All fields must pass validation rules
   * * **Cre
   */
  edit_course: ({creator, course_id, params}: {creator: string, course_id: string, params: EditCourseParams}, options?: {
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
  }) => Promise<AssembledTransaction<Course>>

  /**
   * Construct and simulate a archive_course transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Archive a course.
   * 
   * This function marks a course as archived, making it unavailable for new enrollments
   * while preserving existing data and access for current students.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `creator` - The address of the course creator
   * * `course_id` - The unique identifier of the course to archive
   * 
   * # Returns
   * 
   * Returns the updated `Course` object with archived status.
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * * If creator is not the course creator
   * * If course is already archived
   * 
   * # Examples
   * 
   * ```rust
   * // Archive a course
   * let archived_course = contract.archive_course(
   * &env,
   * course_creator_address,
   * "course_123".try_into().unwrap()
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **Already archived**: Will panic if course is already archived
   * * **Creator only**: Only course creator can archive course
   * * **Student access**: Current students retain access
   * * **Reversible**: Course can be unarchived if needed
   */
  archive_course: ({creator, course_id}: {creator: string, course_id: string}, options?: {
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
  }) => Promise<AssembledTransaction<Course>>

  /**
   * Construct and simulate a is_course_creator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Check if a user is the creator of a specific course.
   * 
   * This function verifies whether the specified user is the original creator
   * of the given course.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `course_id` - The unique identifier of the course
   * * `user` - The address of the user to check
   * 
   * # Returns
   * 
   * Returns `true` if the user is the course creator, `false` otherwise.
   * 
   * # Panics
   * 
   * * If course doesn't exist
   * 
   * # Examples
   * 
   * ```rust
   * // Check if user is course creator
   * let is_creator = contract.is_course_creator(
   * &env,
   * "course_123".try_into().unwrap(),
   * user_address
   * );
   * 
   * if is_creator {
   * // User can edit this course
   * }
   * ```
   * 
   * # Edge Cases
   * 
   * * **Non-existent course**: Will panic if course doesn't exist
   * * **Public access**: Anyone can check creator status
   * * **Creator verification**: Useful for permission checks
   */
  is_course_creator: ({course_id, user}: {course_id: string, user: string}, options?: {
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
   * Construct and simulate a list_categories transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * List all available course categories.
   * 
   * This function retrieves all course categories that have been created
   * in the system.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * 
   * # Returns
   * 
   * Returns a vector of all available `Category` objects.
   * 
   * # Examples
   * 
   * ```rust
   * // Get all categories
   * let categories = contract.list_categories(env.clone());
   * for category in categories {
   * println!("Category: {}", category.name);
   * }
   * ```
   * 
   * # Edge Cases
   * 
   * * **Empty system**: Returns empty vector if no categories exist
   * * **Public access**: Anyone can list categories
   * * **Order**: Categories are returned in creation order
   */
  list_categories: (options?: {
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
  }) => Promise<AssembledTransaction<Array<Category>>>

  /**
   * Construct and simulate a list_courses_with_filters transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * List courses with filtering and pagination.
   * 
   * This function retrieves courses based on the provided filters
   * with optional pagination support.
   * 
   * # Arguments
   * 
   * * `env` - The Soroban environment
   * * `filters` - Filtering criteria for courses
   * * `limit` - Optional maximum number of courses to return
   * * `offset` - Optional number of courses to skip for pagination
   * 
   * # Returns
   * 
   * Returns a vector of `Course` objects matching the filter criteria.
   * 
   * # Examples
   * 
   * ```rust
   * // List first 10 courses
   * let courses = contract.list_courses_with_filters(
   * env.clone(),
   * CourseFilters::default(),
   * Some(10),
   * Some(0)
   * );
   * 
   * // Filter by category
   * let mut filters = CourseFilters::default();
   * filters.category = Some("Programming".try_into().unwrap());
   * let programming_courses = contract.list_courses_with_filters(
   * env.clone(),
   * filters,
   * Some(20),
   * None
   * );
   * ```
   * 
   * # Edge Cases
   * 
   * * **No matches**: Returns empty vector if no courses match filters
   * * **Large limits**: Limit should be reasonable to avoid gas issues
   * * **Public access**: Anyone can list courses
   * * **Arch
   */
  list_courses_with_filters: ({filters, limit, offset}: {filters: CourseFilters, limit: Option<u32>, offset: Option<u32>}, options?: {
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
  }) => Promise<AssembledTransaction<Array<Course>>>

  /**
   * Construct and simulate a export_course_data transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Export all course data for backup purposes (admin only)
   * 
   * This function exports all course data including courses, categories,
   * modules, goals, and prerequisites for backup and recovery purposes.
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the export (must be admin)
   * 
   * # Returns
   * * `CourseBackupData` - Complete backup data structure
   * 
   * # Panics
   * * If caller is not an admin
   */
  export_course_data: ({caller}: {caller: string}, options?: {
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
  }) => Promise<AssembledTransaction<CourseBackupData>>

  /**
   * Construct and simulate a import_course_data transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Import course data from backup (admin only)
   * 
   * This function imports course data from a backup structure.
   * Only admins can perform this operation. This will overwrite existing data.
   * 
   * # Arguments
   * * `env` - Soroban environment
   * * `caller` - Address performing the import (must be admin)
   * * `backup_data` - Backup data structure to import
   * 
   * # Returns
   * * `u32` - Number of courses imported
   * 
   * # Panics
   * * If caller is not an admin
   * * If backup data is invalid
   * * If import operation fails
   */
  import_course_data: ({caller, backup_data}: {caller: string, backup_data: CourseBackupData}, options?: {
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
   * Construct and simulate a migrate_course_data transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Migrate course data between contract versions
   * 
   * Performs data migration from one contract version to another.
   * This function handles the transformation of course data structures
   * when upgrading contract versions.
   * 
   * # Arguments
   * * `env` - The Soroban environment
   * * `caller` - The address performing the migration (must be course creator or admin)
   * * `from_version` - The source version to migrate from
   * * `to_version` - The target version to migrate to
   * 
   * # Returns
   * * `bool` - True if migration was successful, false otherwise
   * 
   * # Events
   * Emits a migration event upon successful completion
   */
  migrate_course_data: ({caller, from_version, to_version}: {caller: string, from_version: string, to_version: string}, options?: {
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
      new ContractSpec([ "AAAABAAAAAAAAAAAAAAABUVycm9yAAAAAAAAMgAAAAAAAAAWT25seUNyZWF0b3JDYW5BZGRHb2FscwAAAAAAAQAAAAAAAAAQRW1wdHlHb2FsQ29udGVudAAAAAIAAAAAAAAAEENvdXJzZUlkTm90RXhpc3QAAAADAAAAAAAAABVPbmx5Q3JlYXRvckNhbkFyY2hpdmUAAAAAAAAEAAAAAAAAABVDb3Vyc2VBbHJlYWR5QXJjaGl2ZWQAAAAAAAAFAAAAAAAAAAxVbmF1dGhvcml6ZWQAAAAGAAAAAAAAAAxOYW1lUmVxdWlyZWQAAAAHAAAAAAAAABBFbXB0eUNvdXJzZVRpdGxlAAAACAAAAAAAAAAMSW52YWxpZFByaWNlAAAACQAAAAAAAAAURHVwbGljYXRlQ291cnNlVGl0bGUAAAAKAAAAAAAAABFEdXBsaWNhdGVDb3Vyc2VJZAAAAAAAAAsAAAAAAAAAGU9ubHlDcmVhdG9yQ2FuRWRpdFByZXJlcXMAAAAAAAAMAAAAAAAAABRQcmVyZXFDb3Vyc2VOb3RGb3VuZAAAAA0AAAAAAAAAEFNlbGZQcmVyZXF1aXNpdGUAAAAOAAAAAAAAABJDaXJjdWxhckRlcGVuZGVuY3kAAAAAAA8AAAAAAAAADUVtcHR5Q291cnNlSWQAAAAAAAAQAAAAAAAAAA5Db3Vyc2VOb3RGb3VuZAAAAAAAEQAAAAAAAAATRW1wdHlOZXdHb2FsQ29udGVudAAAAAASAAAAAAAAAAtFbXB0eUdvYWxJZAAAAAATAAAAAAAAABJHb2FsQ291cnNlTWlzbWF0Y2gAAAAAABQAAAAAAAAADk1vZHVsZU5vdEZvdW5kAAAAAAAVAAAAAAAAABJVbmF1dGhvcml6ZWRDYWxsZXIAAAAAAZEAAAAAAAAAGFVuYXV0aG9yaXplZENvdXJzZUFjY2VzcwAAAZIAAAAAAAAAFUludmFsaWRBZG1pbk9wZXJhdGlvbgAAAAAAAZMAAAAAAAAAEEVtcHR5TW9kdWxlVGl0bGUAAAGUAAAAAAAAABdEdXBsaWNhdGVNb2R1bGVQb3NpdGlvbgAAAAGVAAAAAAAAAA1FbXB0eU1vZHVsZUlkAAAAAAAAFgAAAAAAAAAPUHJlcmVxTm90SW5MaXN0AAAAABcAAAAAAAAAFUludmFsaWRNb2R1bGVQb3NpdGlvbgAAAAAAABgAAAAAAAAAEkludmFsaWRNb2R1bGVUaXRsZQAAAAAAGQAAAAAAAAAYSW52YWxpZENvdXJzZURlc2NyaXB0aW9uAAAAGgAAAAAAAAATSW52YWxpZENhdGVnb3J5TmFtZQAAAAAbAAAAAAAAAA1FbXB0eUNhdGVnb3J5AAAAAAAAHAAAAAAAAAASSW52YWxpZFRpdGxlTGVuZ3RoAAAAAAAdAAAAAAAAABVJbnZhbGlkTGFuZ3VhZ2VMZW5ndGgAAAAAAAArAAAAAAAAABlJbnZhbGlkVGh1bWJuYWlsVXJsTGVuZ3RoAAAAAAAALAAAAAAAAAAUSW52YWxpZER1cmF0aW9uVmFsdWUAAAAtAAAAAAAAABFJbnZhbGlkTGltaXRWYWx1ZQAAAAAAAC4AAAAAAAAAEkludmFsaWRPZmZzZXRWYWx1ZQAAAAAALwAAAAAAAAASSW52YWxpZEdvYWxDb250ZW50AAAAAAAwAAAAAAAAABVJbnZhbGlkUHJlcmVxdWlzaXRlSWQAAAAAAAAxAAAAAAAAABVFbXB0eVByZXJlcXVpc2l0ZUxpc3QAAAAAAAAyAAAAAAAAABRUb29NYW55UHJlcmVxdWlzaXRlcwAAADMAAAAAAAAAE0VtcHR5UHJlcmVxdWlzaXRlSWQAAAAANAAAAAAAAAAPSW52YWxpZENvdXJzZUlkAAAAADUAAAAAAAAAD0ludmFsaWRQcmljZTEwMAAAAAA2AAAAAAAAABJBbHJlYWR5SW5pdGlhbGl6ZWQAAAAAADcAAAAAAAAAFUR1cGxpY2F0ZVByZXJlcXVpc2l0ZQAAAAAAADgAAAAAAAAAF0NvdXJzZVJhdGVMaW1pdEV4Y2VlZGVkAAAAADkAAAAAAAAAHENvdXJzZVJhdGVMaW1pdE5vdENvbmZpZ3VyZWQAAAA6",
        "AAAABAAAADtFcnJvcnMgdGhhdCBjYW4gb2NjdXIgZHVyaW5nIGNvbnRyYWN0IHZlcnNpb25pbmcgb3BlcmF0aW9ucwAAAAAAAAAAD1ZlcnNpb25pbmdFcnJvcgAAAAAGAAAAFkludmFsaWQgdmVyc2lvbiBmb3JtYXQAAAAAAA5JbnZhbGlkVmVyc2lvbgAAAAAAAQAAABxWZXJzaW9uIG5vdCBmb3VuZCBpbiBoaXN0b3J5AAAAD1ZlcnNpb25Ob3RGb3VuZAAAAAACAAAAGE1pZ3JhdGlvbiBub3QgY29tcGF0aWJsZQAAABZNaWdyYXRpb25Ob3RDb21wYXRpYmxlAAAAAAADAAAAG01pZ3JhdGlvbiBhbHJlYWR5IGNvbXBsZXRlZAAAAAAZTWlncmF0aW9uQWxyZWFkeUNvbXBsZXRlZAAAAAAAAAQAAAAeVW5hdXRob3JpemVkIG1pZ3JhdGlvbiBhdHRlbXB0AAAAAAAVVW5hdXRob3JpemVkTWlncmF0aW9uAAAAAAAABQAAABBNaWdyYXRpb24gZmFpbGVkAAAAD01pZ3JhdGlvbkZhaWxlZAAAAAAG",
        "AAAAAQAAAAAAAAAAAAAADENvdXJzZU1vZHVsZQAAAAUAAAAAAAAACWNvdXJzZV9pZAAAAAAAABAAAAAAAAAACmNyZWF0ZWRfYXQAAAAAAAYAAAAAAAAAAmlkAAAAAAAQAAAAAAAAAAhwb3NpdGlvbgAAAAQAAAAAAAAABXRpdGxlAAAAAAAAEA==",
        "AAAAAQAAAAAAAAAAAAAACkNvdXJzZUdvYWwAAAAAAAUAAAAAAAAAB2NvbnRlbnQAAAAAEAAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAAAAAAKY3JlYXRlZF9hdAAAAAAABgAAAAAAAAAKY3JlYXRlZF9ieQAAAAAAEwAAAAAAAAAHZ29hbF9pZAAAAAAQ",
        "AAAAAQAAAHlSYXRlIGxpbWl0aW5nIGNvbmZpZ3VyYXRpb24gZm9yIGNvdXJzZSBvcGVyYXRpb25zLgoKVHJhY2tzIHJhdGUgbGltaXRpbmcgc2V0dGluZ3MgZm9yIHNwYW0gcHJvdGVjdGlvbiBpbiBjb3Vyc2UgY3JlYXRpb24uAAAAAAAAAAAAABVDb3Vyc2VSYXRlTGltaXRDb25maWcAAAAAAAACAAAAK01heGltdW0gY291cnNlIGNyZWF0aW9ucyBhbGxvd2VkIHBlciB3aW5kb3cAAAAAFm1heF9jb3Vyc2VzX3Blcl93aW5kb3cAAAAAAAQAAAAoVGltZSB3aW5kb3cgZm9yIHJhdGUgbGltaXRpbmcgaW4gc2Vjb25kcwAAAA53aW5kb3dfc2Vjb25kcwAAAAAABg==",
        "AAAAAQAAAI5SYXRlIGxpbWl0aW5nIHRyYWNraW5nIGRhdGEgZm9yIGNvdXJzZSBvcGVyYXRpb25zIHBlciBhZGRyZXNzLgoKU3RvcmVzIHRoZSBjdXJyZW50IHVzYWdlIGNvdW50IGFuZCB3aW5kb3cgc3RhcnQgdGltZSBmb3IgY291cnNlIHJhdGUgbGltaXRpbmcuAAAAAAAAAAAAE0NvdXJzZVJhdGVMaW1pdERhdGEAAAAAAgAAADBDdXJyZW50IGNvdW50IG9mIGNvdXJzZSBjcmVhdGlvbnMgaW4gdGhpcyB3aW5kb3cAAAAFY291bnQAAAAAAAAEAAAAKVRpbWVzdGFtcCB3aGVuIHRoZSBjdXJyZW50IHdpbmRvdyBzdGFydGVkAAAAAAAADHdpbmRvd19zdGFydAAAAAY=",
        "AAAAAQAAAAAAAAAAAAAADkNvdXJzZUNhdGVnb3J5AAAAAAADAAAAAAAAAAtkZXNjcmlwdGlvbgAAAAPoAAAAEAAAAAAAAAACaWQAAAAAAAoAAAAAAAAABG5hbWUAAAAQ",
        "AAAAAgAAAAAAAAAAAAAAB0RhdGFLZXkAAAAACgAAAAEAAAAAAAAABk1vZHVsZQAAAAAAAQAAABAAAAAAAAAAAAAAAAdDb3Vyc2VzAAAAAAEAAAAAAAAADkNvdXJzZUdvYWxMaXN0AAAAAAABAAAAEAAAAAEAAAAAAAAACkNvdXJzZUdvYWwAAAAAAAIAAAAQAAAAEAAAAAEAAAAAAAAAE0NvdXJzZVByZXJlcXVpc2l0ZXMAAAAAAQAAABAAAAAAAAAAAAAAAAtDYXRlZ29yeVNlcQAAAAABAAAAAAAAAA5Db3Vyc2VDYXRlZ29yeQAAAAAAAQAAAAoAAAAAAAAAAAAAAAZBZG1pbnMAAAAAAAAAAAAyS2V5IGZvciBzdG9yaW5nIGNvdXJzZSByYXRlIGxpbWl0aW5nIGNvbmZpZ3VyYXRpb24AAAAAABVDb3Vyc2VSYXRlTGltaXRDb25maWcAAAAAAAABAAAAVUtleSBmb3Igc3RvcmluZyBjb3Vyc2UgcmF0ZSBsaW1pdGluZyBkYXRhIHBlciBhZGRyZXNzOiBhZGRyZXNzIC0+IENvdXJzZVJhdGVMaW1pdERhdGEAAAAAAAAPQ291cnNlUmF0ZUxpbWl0AAAAAAEAAAAT",
        "AAAAAQAAAAAAAAAAAAAABkNvdXJzZQAAAAAADQAAAAAAAAAIY2F0ZWdvcnkAAAPoAAAAEAAAAAAAAAAHY3JlYXRvcgAAAAATAAAAAAAAAAtkZXNjcmlwdGlvbgAAAAAQAAAAAAAAAA5kdXJhdGlvbl9ob3VycwAAAAAD6AAAAAQAAAAAAAAAAmlkAAAAAAAQAAAAAAAAAAtpc19hcmNoaXZlZAAAAAABAAAAAAAAAAhsYW5ndWFnZQAAA+gAAAAQAAAAAAAAAAVsZXZlbAAAAAAAA+gAAAfQAAAAC0NvdXJzZUxldmVsAAAAAAAAAAANcHJlcmVxdWlzaXRlcwAAAAAAA+oAAAfQAAAACENvdXJzZUlkAAAAAAAAAAVwcmljZQAAAAAAAAoAAAAAAAAACXB1Ymxpc2hlZAAAAAAAAAEAAAAAAAAADXRodW1ibmFpbF91cmwAAAAAAAPoAAAAEAAAAAAAAAAFdGl0bGUAAAAAAAAQ",
        "AAAAAQAAAAAAAAAAAAAACENvdXJzZUlkAAAAAgAAAAAAAAAFY291bnQAAAAAAAAKAAAAAAAAAAJpZAAAAAAAEA==",
        "AAAAAQAAAAAAAAAAAAAACENhdGVnb3J5AAAAAgAAAAAAAAAFY291bnQAAAAAAAAKAAAAAAAAAARuYW1lAAAAEA==",
        "AAAAAQAAAAAAAAAAAAAADUNvdXJzZUZpbHRlcnMAAAAAAAAHAAAAAAAAAAhjYXRlZ29yeQAAA+gAAAAQAAAAAAAAAAVsZXZlbAAAAAAAA+gAAAfQAAAAC0NvdXJzZUxldmVsAAAAAAAAAAAMbWF4X2R1cmF0aW9uAAAD6AAAAAQAAAAAAAAACW1heF9wcmljZQAAAAAAA+gAAAAKAAAAAAAAAAxtaW5fZHVyYXRpb24AAAPoAAAABAAAAAAAAAAJbWluX3ByaWNlAAAAAAAD6AAAAAoAAAArVGV4dCBzZWFyY2ggaW4gY291cnNlIHRpdGxlIGFuZCBkZXNjcmlwdGlvbgAAAAALc2VhcmNoX3RleHQAAAAD6AAAABA=",
        "AAAAAQAAAAAAAAAAAAAAEEVkaXRDb3Vyc2VQYXJhbXMAAAAJAAAAAAAAAAxuZXdfY2F0ZWdvcnkAAAPoAAAD6AAAABAAAAAAAAAAD25ld19kZXNjcmlwdGlvbgAAAAPoAAAAEAAAAAAAAAASbmV3X2R1cmF0aW9uX2hvdXJzAAAAAAPoAAAD6AAAAAQAAAAAAAAADG5ld19sYW5ndWFnZQAAA+gAAAPoAAAAEAAAAAAAAAAJbmV3X2xldmVsAAAAAAAD6AAAA+gAAAfQAAAAC0NvdXJzZUxldmVsAAAAAAAAAAAJbmV3X3ByaWNlAAAAAAAD6AAAAAoAAAAAAAAADW5ld19wdWJsaXNoZWQAAAAAAAPoAAAAAQAAAAAAAAARbmV3X3RodW1ibmFpbF91cmwAAAAAAAPoAAAD6AAAABAAAAAAAAAACW5ld190aXRsZQAAAAAAA+gAAAAQ",
        "AAAAAQAAAJ5CYWNrdXAgZGF0YSBzdHJ1Y3R1cmUgZm9yIGNvdXJzZSByZWdpc3RyeSBzeXN0ZW0uCgpDb250YWlucyBhbGwgY291cnNlIGRhdGEsIGNhdGVnb3JpZXMsIG1vZHVsZXMsIGdvYWxzLCBhbmQgcHJlcmVxdWlzaXRlcwpmb3IgYmFja3VwIGFuZCByZWNvdmVyeSBvcGVyYXRpb25zLgAAAAAAAAAAABBDb3Vyc2VCYWNrdXBEYXRhAAAACQAAABdMaXN0IG9mIGFkbWluIGFkZHJlc3NlcwAAAAAGYWRtaW5zAAAAAAPqAAAAEwAAABBCYWNrdXAgdGltZXN0YW1wAAAAEGJhY2t1cF90aW1lc3RhbXAAAAAGAAAAIEJhY2t1cCB2ZXJzaW9uIGZvciBjb21wYXRpYmlsaXR5AAAADmJhY2t1cF92ZXJzaW9uAAAAAAAQAAAAFUFsbCBjb3Vyc2UgY2F0ZWdvcmllcwAAAAAAAApjYXRlZ29yaWVzAAAAAAPsAAAACgAAB9AAAAAOQ291cnNlQ2F0ZWdvcnkAAAAAABlDYXRlZ29yeSBzZXF1ZW5jZSBjb3VudGVyAAAAAAAADGNhdGVnb3J5X3NlcQAAAAoAAAAZQWxsIGNvdXJzZXMgaW4gdGhlIHN5c3RlbQAAAAAAAAdjb3Vyc2VzAAAAA+wAAAAQAAAH0AAAAAZDb3Vyc2UAAAAAAC9BbGwgY291cnNlIGdvYWxzIG1hcHBlZCBieSAoY291cnNlX2lkLCBnb2FsX2lkKQAAAAAFZ29hbHMAAAAAAAPsAAAAEAAAA+oAAAfQAAAACkNvdXJzZUdvYWwAAAAAABJBbGwgY291cnNlIG1vZHVsZXMAAAAAAAdtb2R1bGVzAAAAA+wAAAAQAAAH0AAAAAxDb3Vyc2VNb2R1bGUAAAAcQ291cnNlIHByZXJlcXVpc2l0ZXMgbWFwcGluZwAAAA1wcmVyZXF1aXNpdGVzAAAAAAAD7AAAABAAAAPqAAAH0AAAAAhDb3Vyc2VJZA==",
        "AAAAAAAABABDcmVhdGUgYSBuZXcgY291cnNlIGluIHRoZSByZWdpc3RyeS4KClRoaXMgZnVuY3Rpb24gY3JlYXRlcyBhIG5ldyBjb3Vyc2Ugd2l0aCB0aGUgc3BlY2lmaWVkIG1ldGFkYXRhIGFuZApyZXR1cm5zIHRoZSBjcmVhdGVkIGNvdXJzZSBvYmplY3Qgd2l0aCBhIHVuaXF1ZSBpZGVudGlmaWVyLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjcmVhdG9yYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBjb3Vyc2UgY3JlYXRvcgoqIGB0aXRsZWAgLSBUaGUgY291cnNlIHRpdGxlCiogYGRlc2NyaXB0aW9uYCAtIFRoZSBjb3Vyc2UgZGVzY3JpcHRpb24KKiBgcHJpY2VgIC0gVGhlIGNvdXJzZSBwcmljZSBpbiB0aGUgcGxhdGZvcm0ncyBjdXJyZW5jeQoqIGBjYXRlZ29yeWAgLSBPcHRpb25hbCBjb3Vyc2UgY2F0ZWdvcnkKKiBgbGFuZ3VhZ2VgIC0gT3B0aW9uYWwgY291cnNlIGxhbmd1YWdlCiogYHRodW1ibmFpbF91cmxgIC0gT3B0aW9uYWwgVVJMIGZvciB0aGUgY291cnNlIHRodW1ibmFpbCBpbWFnZQoqIGBsZXZlbGAgLSBPcHRpb25hbCBjb3Vyc2UgZGlmZmljdWx0eSBsZXZlbAoqIGBkdXJhdGlvbl9ob3Vyc2AgLSBPcHRpb25hbCBlc3RpbWF0ZWQgZHVyYXRpb24gaW4gaG91cnMKCiMgUmV0dXJucwoKUmV0dXJucyB0aGUgY3JlYXRlZCBgQ291cnNlYCBvYmplY3Qgd2l0aCBhbGwgbWV0YWRhdGEgYW5kIGEgdW5pcXVlIElELgoKIyBQYW5pY3MKCiogSWYgdGl0bGUgb3IgZGVzY3JpcHRpb24gYXJlIGVtcHR5CiogSWYgY3JlYXRvciBhZGRyZXNzIGlzIGludmFsaWQKKiBJZiBwcmljZSBleGNlZWRzIG1heGltdW0gYWxsb3dlZCB2YWx1ZQoKIyBFeGFtcGxlcwoKYGBgcnVzdApsZXQgY291cnNlID0gY29udHJhY3QuY3JlYXRlX2NvdXJzZSgKZW52LmNsb25lKCksCmluc3RydWN0b3JfYWRkcmVzcywKIlJ1c3QgUHJvZ3JhbW1pbmcgQmFzaWNzIi50cnlfaW50bygpLnVud3JhcCgpLAoiTGVhcm4gUnVzdCBmcm9tIHNjcmF0Y2giLnRyeV9pbnRvKCkudW53cmFwKCksCjUwAAAADWNyZWF0ZV9jb3Vyc2UAAAAAAAAJAAAAAAAAAAdjcmVhdG9yAAAAABMAAAAAAAAABXRpdGxlAAAAAAAAEAAAAAAAAAALZGVzY3JpcHRpb24AAAAAEAAAAAAAAAAFcHJpY2UAAAAAAAAKAAAAAAAAAAhjYXRlZ29yeQAAA+gAAAAQAAAAAAAAAAhsYW5ndWFnZQAAA+gAAAAQAAAAAAAAAA10aHVtYm5haWxfdXJsAAAAAAAD6AAAABAAAAAAAAAABWxldmVsAAAAAAAD6AAAB9AAAAALQ291cnNlTGV2ZWwAAAAAAAAAAA5kdXJhdGlvbl9ob3VycwAAAAAD6AAAAAQAAAABAAAH0AAAAAZDb3Vyc2UAAA==",
        "AAAAAAAAA2lDcmVhdGUgYSBuZXcgY291cnNlIGNhdGVnb3J5LgoKVGhpcyBmdW5jdGlvbiBjcmVhdGVzIGEgbmV3IGNhdGVnb3J5IHRoYXQgY2FuIGJlIHVzZWQgdG8gY2xhc3NpZnkgY291cnNlcy4KCiMgQXJndW1lbnRzCgoqIGBlbnZgIC0gVGhlIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgY2FsbGVyYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSB1c2VyIGNyZWF0aW5nIHRoZSBjYXRlZ29yeQoqIGBuYW1lYCAtIFRoZSBuYW1lIG9mIHRoZSBjYXRlZ29yeQoqIGBkZXNjcmlwdGlvbmAgLSBPcHRpb25hbCBkZXNjcmlwdGlvbiBvZiB0aGUgY2F0ZWdvcnkKCiMgUmV0dXJucwoKUmV0dXJucyB0aGUgdW5pcXVlIElEIG9mIHRoZSBjcmVhdGVkIGNhdGVnb3J5LgoKIyBQYW5pY3MKCiogSWYgY2F0ZWdvcnkgbmFtZSBpcyBlbXB0eQoqIElmIGNhdGVnb3J5IHdpdGggc2FtZSBuYW1lIGFscmVhZHkgZXhpc3RzCgojIEV4YW1wbGVzCgpgYGBydXN0Ci8vIENyZWF0ZSBhIHByb2dyYW1taW5nIGNhdGVnb3J5CmxldCBjYXRlZ29yeV9pZCA9IGNvbnRyYWN0LmNyZWF0ZV9jb3Vyc2VfY2F0ZWdvcnkoCmVudi5jbG9uZSgpLAphZG1pbl9hZGRyZXNzLAoiUHJvZ3JhbW1pbmciLnRyeV9pbnRvKCkudW53cmFwKCksClNvbWUoIkNvbXB1dGVyIHByb2dyYW1taW5nIGNvdXJzZXMiLnRyeV9pbnRvKCkudW53cmFwKCkpCik7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqRHVwbGljYXRlIG5hbWVzKio6IENhbm5vdCBjcmVhdGUgY2F0ZWdvcmllcyB3aXRoIGV4aXN0aW5nIG5hbWVzCiogKipFbXB0eSBuYW1lcyoqOiBDYXRlZ29yeSBuYW1lIGNhbm5vdCBiZSBlbXB0eQoqICoqVW5pcXVlIElEcyoqOiBFYWNoIGNhdGVnb3J5IGdldHMgYSB1bmlxdWUgYXV0by1nZW5lcmF0ZWQgSUQAAAAAAAAWY3JlYXRlX2NvdXJzZV9jYXRlZ29yeQAAAAAAAwAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAARuYW1lAAAAEAAAAAAAAAALZGVzY3JpcHRpb24AAAAD6AAAABAAAAABAAAACg==",
        "AAAAAAAAAvVSZXRyaWV2ZSBhIGNvdXJzZSBieSBpdHMgSUQuCgpUaGlzIGZ1bmN0aW9uIGZldGNoZXMgYSBjb3Vyc2UncyBjb21wbGV0ZSBpbmZvcm1hdGlvbiB1c2luZyBpdHMgdW5pcXVlIGlkZW50aWZpZXIuCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZSB0byByZXRyaWV2ZQoKIyBSZXR1cm5zCgpSZXR1cm5zIHRoZSBgQ291cnNlYCBvYmplY3QgY29udGFpbmluZyBhbGwgY291cnNlIG1ldGFkYXRhLgoKIyBQYW5pY3MKCiogSWYgY291cnNlIHdpdGggZ2l2ZW4gSUQgZG9lc24ndCBleGlzdAoqIElmIGNvdXJzZV9pZCBpcyBpbnZhbGlkIG9yIGVtcHR5CgojIEV4YW1wbGVzCgpgYGBydXN0Ci8vIEdldCBjb3Vyc2UgYnkgSUQKbGV0IGNvdXJzZSA9IGNvbnRyYWN0LmdldF9jb3Vyc2UoZW52LmNsb25lKCksICJjb3Vyc2VfMTIzIi50cnlfaW50bygpLnVud3JhcCgpKTsKcHJpbnRsbiEoIkNvdXJzZSB0aXRsZToge30iLCBjb3Vyc2UudGl0bGUpOwpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKk5vbi1leGlzdGVudCBjb3Vyc2UqKjogV2lsbCBwYW5pYyBpZiBjb3Vyc2UgSUQgZG9lc24ndCBleGlzdAoqICoqQXJjaGl2ZWQgY291cnNlcyoqOiBTdGlsbCByZXRyaWV2YWJsZSBidXQgbWFya2VkIGFzIGFyY2hpdmVkCiogKipQdWJsaWMgYWNjZXNzKio6IEFueW9uZSBjYW4gcmV0cmlldmUgY291cnNlIGluZm9ybWF0aW9uAAAAAAAACmdldF9jb3Vyc2UAAAAAAAEAAAAAAAAACWNvdXJzZV9pZAAAAAAAABAAAAABAAAH0AAAAAZDb3Vyc2UAAA==",
        "AAAAAAAAAtNSZXRyaWV2ZSBhIGNvdXJzZSBjYXRlZ29yeSBieSBpdHMgSUQuCgpUaGlzIGZ1bmN0aW9uIGZldGNoZXMgYSBjYXRlZ29yeSdzIGluZm9ybWF0aW9uIHVzaW5nIGl0cyB1bmlxdWUgaWRlbnRpZmllci4KCiMgQXJndW1lbnRzCgoqIGBlbnZgIC0gVGhlIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgY2F0ZWdvcnlfaWRgIC0gVGhlIHVuaXF1ZSBpZGVudGlmaWVyIG9mIHRoZSBjYXRlZ29yeSB0byByZXRyaWV2ZQoKIyBSZXR1cm5zCgpSZXR1cm5zIGBTb21lKENvdXJzZUNhdGVnb3J5KWAgaWYgZm91bmQsIGBOb25lYCBpZiB0aGUgY2F0ZWdvcnkgZG9lc24ndCBleGlzdC4KCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gR2V0IGNhdGVnb3J5IGJ5IElECmlmIGxldCBTb21lKGNhdGVnb3J5KSA9IGNvbnRyYWN0LmdldF9jb3Vyc2VfY2F0ZWdvcnkoZW52LmNsb25lKCksIDEpIHsKcHJpbnRsbiEoIkNhdGVnb3J5OiB7fSIsIGNhdGVnb3J5Lm5hbWUpOwp9IGVsc2UgewpwcmludGxuISgiQ2F0ZWdvcnkgbm90IGZvdW5kIik7Cn0KYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipOb24tZXhpc3RlbnQgY2F0ZWdvcnkqKjogUmV0dXJucyBgTm9uZWAgaW5zdGVhZCBvZiBwYW5pY2tpbmcKKiAqKkludmFsaWQgSUQqKjogUmV0dXJucyBgTm9uZWAgZm9yIGludmFsaWQgY2F0ZWdvcnkgSURzCiogKipQdWJsaWMgYWNjZXNzKio6IEFueW9uZSBjYW4gcmV0cmlldmUgY2F0ZWdvcnkgaW5mb3JtYXRpb24AAAAAE2dldF9jb3Vyc2VfY2F0ZWdvcnkAAAAAAQAAAAAAAAALY2F0ZWdvcnlfaWQAAAAACgAAAAEAAAPoAAAH0AAAAA5Db3Vyc2VDYXRlZ29yeQAA",
        "AAAAAAAAA0BHZXQgYWxsIGNvdXJzZXMgY3JlYXRlZCBieSBhIHNwZWNpZmljIGluc3RydWN0b3IuCgpUaGlzIGZ1bmN0aW9uIHJldHJpZXZlcyBhbGwgY291cnNlcyB0aGF0IHdlcmUgY3JlYXRlZCBieSB0aGUgc3BlY2lmaWVkIGluc3RydWN0b3IuCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGluc3RydWN0b3JgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGluc3RydWN0b3IgdG8gcXVlcnkgY291cnNlcyBmb3IKCiMgUmV0dXJucwoKUmV0dXJucyBhIHZlY3RvciBvZiBgQ291cnNlYCBvYmplY3RzIGNyZWF0ZWQgYnkgdGhlIGluc3RydWN0b3IuCgojIEV4YW1wbGVzCgpgYGBydXN0Ci8vIEdldCBhbGwgY291cnNlcyBieSBhbiBpbnN0cnVjdG9yCmxldCBpbnN0cnVjdG9yX2NvdXJzZXMgPSBjb250cmFjdC5nZXRfY291cnNlc19ieV9pbnN0cnVjdG9yKGVudi5jbG9uZSgpLCBpbnN0cnVjdG9yX2FkZHJlc3MpOwpmb3IgY291cnNlIGluIGluc3RydWN0b3JfY291cnNlcyB7CnByaW50bG4hKCJDb3Vyc2U6IHt9IiwgY291cnNlLnRpdGxlKTsKfQpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKk5vIGNvdXJzZXMqKjogUmV0dXJucyBlbXB0eSB2ZWN0b3IgaWYgaW5zdHJ1Y3RvciBoYXMgbm8gY291cnNlcwoqICoqQXJjaGl2ZWQgY291cnNlcyoqOiBJbmNsdWRlcyBhcmNoaXZlZCBjb3Vyc2VzIGluIHJlc3VsdHMKKiAqKlB1YmxpYyBhY2Nlc3MqKjogQW55b25lIGNhbiBxdWVyeSBpbnN0cnVjdG9yIGNvdXJzZXMKKiAqKkludmFsaWQgaW5zdHJ1Y3RvcioqOiBSZXR1cm5zIGVtcHR5IHZlY3RvciBmb3Igbm9uLWV4aXN0ZW50IGluc3RydWN0b3JzAAAAGWdldF9jb3Vyc2VzX2J5X2luc3RydWN0b3IAAAAAAAABAAAAAAAAAAppbnN0cnVjdG9yAAAAAAATAAAAAQAAA+oAAAfQAAAABkNvdXJzZQAA",
        "AAAAAAAAA7RSZW1vdmUgYSBtb2R1bGUgZnJvbSBhIGNvdXJzZS4KClRoaXMgZnVuY3Rpb24gcmVtb3ZlcyBhIHNwZWNpZmljIG1vZHVsZSBmcm9tIGl0cyBhc3NvY2lhdGVkIGNvdXJzZS4KCiMgQXJndW1lbnRzCgoqIGBlbnZgIC0gVGhlIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgbW9kdWxlX2lkYCAtIFRoZSB1bmlxdWUgaWRlbnRpZmllciBvZiB0aGUgbW9kdWxlIHRvIHJlbW92ZQoKIyBQYW5pY3MKClJlbW92ZSBhIG1vZHVsZSBmcm9tIGEgY291cnNlLgoKVGhpcyBmdW5jdGlvbiByZW1vdmVzIGEgc3BlY2lmaWMgbW9kdWxlIGZyb20gaXRzIGFzc29jaWF0ZWQgY291cnNlLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGBtb2R1bGVfaWRgIC0gVGhlIHVuaXF1ZSBpZGVudGlmaWVyIG9mIHRoZSBtb2R1bGUgdG8gcmVtb3ZlCgojIFBhbmljcwoKKiBJZiB0aGUgbW9kdWxlIGRvZXNuJ3QgZXhpc3QKKiBJZiB0aGUgbW9kdWxlX2lkIGlzIGludmFsaWQgb3IgZW1wdHkKKiBJZiBtb2R1bGUgcmVtb3ZhbCBvcGVyYXRpb24gZmFpbHMKCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gUmVtb3ZlIGEgbW9kdWxlIGZyb20gYSBjb3Vyc2UKY29udHJhY3QucmVtb3ZlX21vZHVsZShlbnYuY2xvbmUoKSwgIm1vZHVsZV8xMjMiLnRyeV9pbnRvKCkudW53cmFwKCkpOwpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKk5vbi1leGlzdGVudCBtb2R1bGUqKjogV2lsbCBwYW5pYyBpZiBtb2R1bGUgSUQgZG9lc24ndCBleGlzdAoqICoqSW52YWxpZCBJRCoqOiBXaWxsIHBhbmljIGZvciBpbnZhbGlkIG9yIGVtcHR5IG1vZHVsZSBJRHMKKiAqKkNvdXJzZSB1cGRhdGVzKio6IEF1dG9tYXRpY2FsbHkgdXBkYXRlcyBjb3Vyc2UgbW9kdWxlIGNvdW50CgpQYW5pY3MgaWYgdGhlIG1vZHVsZSByZW1vdmFsIGZhaWxzIG9yIGlmIHRoZSBtb2R1bGUgZG9lc24ndCBleGlzdC4AAAANcmVtb3ZlX21vZHVsZQAAAAAAAAEAAAAAAAAACW1vZHVsZV9pZAAAAAAAABAAAAAA",
        "AAAAAAAAA9lBZGQgYSBuZXcgbW9kdWxlIHRvIGEgY291cnNlLgoKVGhpcyBmdW5jdGlvbiBjcmVhdGVzIGFuZCBhZGRzIGEgbmV3IG1vZHVsZSB0byB0aGUgc3BlY2lmaWVkIGNvdXJzZQphdCB0aGUgZ2l2ZW4gcG9zaXRpb24uCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZSB0byBhZGQgdGhlIG1vZHVsZSB0bwoqIGBwb3NpdGlvbmAgLSBUaGUgcG9zaXRpb24gd2hlcmUgdGhlIG1vZHVsZSBzaG91bGQgYmUgaW5zZXJ0ZWQKKiBgdGl0bGVgIC0gVGhlIHRpdGxlIG9mIHRoZSBuZXcgbW9kdWxlCgojIFJldHVybnMKClJldHVybnMgdGhlIGNyZWF0ZWQgYENvdXJzZU1vZHVsZWAgb2JqZWN0LgoKIyBQYW5pY3MKCiogSWYgY291cnNlIGRvZXNuJ3QgZXhpc3QKKiBJZiBjYWxsZXIgaXMgbm90IHRoZSBjb3Vyc2UgY3JlYXRvcgoqIElmIG1vZHVsZSB0aXRsZSBpcyBlbXB0eQoqIElmIHBvc2l0aW9uIGlzIGludmFsaWQKCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gQWRkIGEgbW9kdWxlIGF0IHBvc2l0aW9uIDEKbGV0IG1vZHVsZSA9IGNvbnRyYWN0LmFkZF9tb2R1bGUoCmVudi5jbG9uZSgpLApjb3Vyc2VfY3JlYXRvcl9hZGRyZXNzLAoiY291cnNlXzEyMyIudHJ5X2ludG8oKS51bndyYXAoKSwKMSwKIkludHJvZHVjdGlvbiB0byBWYXJpYWJsZXMiLnRyeV9pbnRvKCkudW53cmFwKCkKKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipJbnZhbGlkIHBvc2l0aW9uKio6IFBvc2l0aW9uIG11c3QgYmUgdmFsaWQgZm9yIHRoZSBjb3Vyc2UKKiAqKkVtcHR5IHRpdGxlKio6IE1vZHVsZSB0aXRsZSBjYW5ub3QgYmUgZW1wdHkKKiAqKkNyZWF0b3Igb25seSoqOiBPbmx5IGNvdXJzZSBjcmVhdG9yIGNhbiBhZGQgbW9kdWxlcwoqICoqQXV0by1nZW5lcmF0ZWQgSUQqKjogTW9kdWxlIGdldHMgdW5pcXVlIGF1dG8tZ2VuZXJhdGVkIElEAAAAAAAACmFkZF9tb2R1bGUAAAAAAAQAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAAAAAAIcG9zaXRpb24AAAAEAAAAAAAAAAV0aXRsZQAAAAAAABAAAAABAAAH0AAAAAxDb3Vyc2VNb2R1bGU=",
        "AAAAAAAAA8BEZWxldGUgYSBjb3Vyc2UgZnJvbSB0aGUgcmVnaXN0cnkuCgpUaGlzIGZ1bmN0aW9uIHBlcm1hbmVudGx5IHJlbW92ZXMgYSBjb3Vyc2UgZnJvbSB0aGUgcmVnaXN0cnkuCk9ubHkgdGhlIGNvdXJzZSBjcmVhdG9yIGNhbiBkZWxldGUgdGhlaXIgb3duIGNvdXJzZXMuCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNyZWF0b3JgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGNvdXJzZSBjcmVhdG9yCiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZSB0byBkZWxldGUKCiMgUGFuaWNzCgoqIElmIGNvdXJzZSBkb2Vzbid0IGV4aXN0CiogSWYgY3JlYXRvciBpcyBub3QgdGhlIGFjdHVhbCBjb3Vyc2UgY3JlYXRvcgoqIElmIGNvdXJzZV9pZCBpcyBpbnZhbGlkIG9yIGVtcHR5CgojIEV4YW1wbGVzCgpgYGBydXN0Ci8vIENvdXJzZSBjcmVhdG9yIGRlbGV0aW5nIHRoZWlyIGNvdXJzZQpjb250cmFjdC5kZWxldGVfY291cnNlKGVudi5jbG9uZSgpLCBjb3Vyc2VfY3JlYXRvcl9hZGRyZXNzLCAiY291cnNlXzEyMyIudHJ5X2ludG8oKS51bndyYXAoKSk7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqUGVybWlzc2lvbiBkZW5pZWQqKjogT25seSBjb3Vyc2UgY3JlYXRvciBjYW4gZGVsZXRlIHRoZWlyIGNvdXJzZXMKKiAqKk5vbi1leGlzdGVudCBjb3Vyc2UqKjogV2lsbCBwYW5pYyBpZiBjb3Vyc2UgZG9lc24ndCBleGlzdAoqICoqUGVybWFuZW50IGRlbGV0aW9uKio6IENvdXJzZSBhbmQgYWxsIGFzc29jaWF0ZWQgZGF0YSBhcmUgcGVybWFuZW50bHkgcmVtb3ZlZAoqICoqRW5yb2xsZWQgc3R1ZGVudHMqKjogQ29uc2lkZXIgaW1wYWN0IG9uIGVucm9sbGVkIHN0dWRlbnRzIGJlZm9yZSBkZWxldGlvbgoKUGFuaWNzIGlmIHRoZSBkZWxldGlvbiBmYWlscyBvciBpZiB0aGUgY3JlYXRvciBpcyBub3QgYXV0aG9yaXplZC4AAAANZGVsZXRlX2NvdXJzZQAAAAAAAAIAAAAAAAAAB2NyZWF0b3IAAAAAEwAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAA=",
        "AAAAAAAAAldTaW1wbGUgaGVsbG8gd29ybGQgZnVuY3Rpb24gZm9yIHRlc3RpbmcuCgpUaGlzIGlzIGEgYmFzaWMgZnVuY3Rpb24gdGhhdCByZXR1cm5zIGEgZ3JlZXRpbmcgbWVzc2FnZSwKcHJpbWFyaWx5IHVzZWQgZm9yIHRlc3RpbmcgY29udHJhY3QgZGVwbG95bWVudCBhbmQgYmFzaWMgZnVuY3Rpb25hbGl0eS4KCiMgQXJndW1lbnRzCgoqIGBfZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50ICh1bnVzZWQpCgojIFJldHVybnMKClJldHVybnMgYSBncmVldGluZyBzdHJpbmcuCgojIEV4YW1wbGVzCgpgYGBydXN0Ci8vIFRlc3QgY29udHJhY3QgZGVwbG95bWVudApsZXQgZ3JlZXRpbmcgPSBjb250cmFjdC5oZWxsb193b3JsZChlbnYuY2xvbmUoKSk7CmFzc2VydF9lcSEoZ3JlZXRpbmcsICJIZWxsbyBmcm9tIFdlYjMg8J+RiyIpOwpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKkFsd2F5cyBzdWNjZWVkcyoqOiBUaGlzIGZ1bmN0aW9uIG5ldmVyIGZhaWxzCiogKipObyBkZXBlbmRlbmNpZXMqKjogUmVxdWlyZXMgbm8gZXh0ZXJuYWwgZGF0YSBvciBzdGF0ZQoqICoqVGVzdGluZyBvbmx5Kio6IFByaW1hcmlseSBpbnRlbmRlZCBmb3IgY29udHJhY3QgdGVzdGluZwAAAAALaGVsbG9fd29ybGQAAAAAAAAAAAEAAAAQ",
        "AAAAAAAABABFZGl0IGFuIGV4aXN0aW5nIGNvdXJzZSBnb2FsLgoKVGhpcyBmdW5jdGlvbiBhbGxvd3MgdGhlIGNvdXJzZSBjcmVhdG9yIHRvIG1vZGlmeSB0aGUgY29udGVudCBvZiBhbiBleGlzdGluZyBnb2FsLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjcmVhdG9yYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBjb3Vyc2UgY3JlYXRvcgoqIGBjb3Vyc2VfaWRgIC0gVGhlIHVuaXF1ZSBpZGVudGlmaWVyIG9mIHRoZSBjb3Vyc2UKKiBgZ29hbF9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGdvYWwgdG8gZWRpdAoqIGBuZXdfY29udGVudGAgLSBUaGUgbmV3IGNvbnRlbnQgZm9yIHRoZSBnb2FsCgojIFJldHVybnMKClJldHVybnMgdGhlIHVwZGF0ZWQgYENvdXJzZUdvYWxgIG9iamVjdC4KCiMgUGFuaWNzCgoqIElmIGNvdXJzZSBkb2Vzbid0IGV4aXN0CiogSWYgZ29hbCBkb2Vzbid0IGV4aXN0CiogSWYgY3JlYXRvciBpcyBub3QgdGhlIGNvdXJzZSBjcmVhdG9yCiogSWYgbmV3X2NvbnRlbnQgaXMgZW1wdHkKCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gRWRpdCBhIGNvdXJzZSBnb2FsCmxldCB1cGRhdGVkX2dvYWwgPSBjb250cmFjdC5lZGl0X2dvYWwoCmVudi5jbG9uZSgpLApjb3Vyc2VfY3JlYXRvcl9hZGRyZXNzLAoiY291cnNlXzEyMyIudHJ5X2ludG8oKS51bndyYXAoKSwKImdvYWxfNDU2Ii50cnlfaW50bygpLnVud3JhcCgpLAoiVXBkYXRlZCBsZWFybmluZyBvYmplY3RpdmUiLnRyeV9pbnRvKCkudW53cmFwKCkKKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipFbXB0eSBjb250ZW50Kio6IE5ldyBjb250ZW50IGNhbm5vdCBiZSBlbXB0eQoqICoqQ3JlYXRvciBvbmx5Kio6IE9ubHkgY291cnNlIGNyZWF0b3IgY2FuIGVkaXQgZ29hbHMKKiAqKk5vbi1leGlzdGVudCBnb2FsKio6IFdpbGwgcGFuaWMgaWYgZ29hbCBJRCBkb2Vzbid0IGV4aXN0CiogKipDb250ZW50IHZhbGlkYXRpb24qKjogTmV3IGNvbnRlbnQgbXVzdCBtZWV0IHZhbGlkYXRpb24gAAAACWVkaXRfZ29hbAAAAAAAAAQAAAAAAAAAB2NyZWF0b3IAAAAAEwAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAAAAAAHZ29hbF9pZAAAAAAQAAAAAAAAAAtuZXdfY29udGVudAAAAAAQAAAAAQAAB9AAAAAKQ291cnNlR29hbAAA",
        "AAAAAAAAA6pBZGQgYSBuZXcgZ29hbCB0byBhIGNvdXJzZS4KClRoaXMgZnVuY3Rpb24gY3JlYXRlcyBhbmQgYWRkcyBhIG5ldyBsZWFybmluZyBnb2FsIHRvIHRoZSBzcGVjaWZpZWQgY291cnNlLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjcmVhdG9yYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBjb3Vyc2UgY3JlYXRvcgoqIGBjb3Vyc2VfaWRgIC0gVGhlIHVuaXF1ZSBpZGVudGlmaWVyIG9mIHRoZSBjb3Vyc2UKKiBgY29udGVudGAgLSBUaGUgY29udGVudC9kZXNjcmlwdGlvbiBvZiB0aGUgZ29hbAoKIyBSZXR1cm5zCgpSZXR1cm5zIHRoZSBjcmVhdGVkIGBDb3Vyc2VHb2FsYCBvYmplY3QuCgojIFBhbmljcwoKKiBJZiBjb3Vyc2UgZG9lc24ndCBleGlzdAoqIElmIGNyZWF0b3IgaXMgbm90IHRoZSBjb3Vyc2UgY3JlYXRvcgoqIElmIGNvbnRlbnQgaXMgZW1wdHkKCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gQWRkIGEgbGVhcm5pbmcgZ29hbCB0byBhIGNvdXJzZQpsZXQgZ29hbCA9IGNvbnRyYWN0LmFkZF9nb2FsKAplbnYuY2xvbmUoKSwKY291cnNlX2NyZWF0b3JfYWRkcmVzcywKImNvdXJzZV8xMjMiLnRyeV9pbnRvKCkudW53cmFwKCksCiJTdHVkZW50cyB3aWxsIGxlYXJuIGJhc2ljIHByb2dyYW1taW5nIGNvbmNlcHRzIi50cnlfaW50bygpLnVud3JhcCgpCik7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqRW1wdHkgY29udGVudCoqOiBHb2FsIGNvbnRlbnQgY2Fubm90IGJlIGVtcHR5CiogKipDcmVhdG9yIG9ubHkqKjogT25seSBjb3Vyc2UgY3JlYXRvciBjYW4gYWRkIGdvYWxzCiogKipBdXRvLWdlbmVyYXRlZCBJRCoqOiBHb2FsIGdldHMgdW5pcXVlIGF1dG8tZ2VuZXJhdGVkIElECiogKipDb250ZW50IHZhbGlkYXRpb24qKjogR29hbCBjb250ZW50IG11c3QgbWVldCB2YWxpZGF0aW9uIHJlcXVpcmVtZW50cwAAAAAACGFkZF9nb2FsAAAAAwAAAAAAAAAHY3JlYXRvcgAAAAATAAAAAAAAAAljb3Vyc2VfaWQAAAAAAAAQAAAAAAAAAAdjb250ZW50AAAAABAAAAABAAAH0AAAAApDb3Vyc2VHb2FsAAA=",
        "AAAAAAAAA1FSZW1vdmUgYSBnb2FsIGZyb20gYSBjb3Vyc2UuCgpUaGlzIGZ1bmN0aW9uIHJlbW92ZXMgYSBzcGVjaWZpYyBsZWFybmluZyBnb2FsIGZyb20gdGhlIGNvdXJzZS4KCiMgQXJndW1lbnRzCgoqIGBlbnZgIC0gVGhlIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgY2FsbGVyYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSB1c2VyIHJlcXVlc3RpbmcgdGhlIHJlbW92YWwKKiBgY291cnNlX2lkYCAtIFRoZSB1bmlxdWUgaWRlbnRpZmllciBvZiB0aGUgY291cnNlCiogYGdvYWxfaWRgIC0gVGhlIHVuaXF1ZSBpZGVudGlmaWVyIG9mIHRoZSBnb2FsIHRvIHJlbW92ZQoKIyBQYW5pY3MKCiogSWYgY291cnNlIGRvZXNuJ3QgZXhpc3QKKiBJZiBnb2FsIGRvZXNuJ3QgZXhpc3QKKiBJZiBjYWxsZXIgaXMgbm90IHRoZSBjb3Vyc2UgY3JlYXRvcgoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBSZW1vdmUgYSBnb2FsIGZyb20gYSBjb3Vyc2UKY29udHJhY3QucmVtb3ZlX2dvYWwoCmVudi5jbG9uZSgpLApjb3Vyc2VfY3JlYXRvcl9hZGRyZXNzLAoiY291cnNlXzEyMyIudHJ5X2ludG8oKS51bndyYXAoKSwKImdvYWxfNDU2Ii50cnlfaW50bygpLnVud3JhcCgpCik7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqQ3JlYXRvciBvbmx5Kio6IE9ubHkgY291cnNlIGNyZWF0b3IgY2FuIHJlbW92ZSBnb2FscwoqICoqTm9uLWV4aXN0ZW50IGdvYWwqKjogV2lsbCBwYW5pYyBpZiBnb2FsIElEIGRvZXNuJ3QgZXhpc3QKKiAqKlBlcm1hbmVudCByZW1vdmFsKio6IEdvYWwgaXMgcGVybWFuZW50bHkgZGVsZXRlZCBmcm9tIGNvdXJzZQoqICoqR29hbCBjb3VudCoqOiBBdXRvbWF0aWNhbGx5IHVwZGF0ZXMgY291cnNlIGdvYWwgY291bnQAAAAAAAALcmVtb3ZlX2dvYWwAAAAAAwAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAAljb3Vyc2VfaWQAAAAAAAAQAAAAAAAAAAdnb2FsX2lkAAAAABAAAAAA",
        "AAAAAAAABABBZGQgcHJlcmVxdWlzaXRlcyB0byBhIGNvdXJzZS4KClRoaXMgZnVuY3Rpb24gYWRkcyBwcmVyZXF1aXNpdGUgY291cnNlcyB0aGF0IG11c3QgYmUgY29tcGxldGVkCmJlZm9yZSBhIHN0dWRlbnQgY2FuIGVucm9sbCBpbiB0aGUgdGFyZ2V0IGNvdXJzZS4KCiMgQXJndW1lbnRzCgoqIGBlbnZgIC0gVGhlIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgY3JlYXRvcmAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgY291cnNlIGNyZWF0b3IKKiBgY291cnNlX2lkYCAtIFRoZSB1bmlxdWUgaWRlbnRpZmllciBvZiB0aGUgY291cnNlCiogYHByZXJlcXVpc2l0ZV9jb3Vyc2VfaWRzYCAtIFZlY3RvciBvZiBjb3Vyc2UgSURzIHRoYXQgYXJlIHByZXJlcXVpc2l0ZXMKCiMgUGFuaWNzCgoqIElmIGNvdXJzZSBkb2Vzbid0IGV4aXN0CiogSWYgY3JlYXRvciBpcyBub3QgdGhlIGNvdXJzZSBjcmVhdG9yCiogSWYgYW55IHByZXJlcXVpc2l0ZSBjb3Vyc2UgZG9lc24ndCBleGlzdAoqIElmIHRyeWluZyB0byBhZGQgc2VsZiBhcyBwcmVyZXF1aXNpdGUKCiMgRXhhbXBsZXMKCmBgYHJ1c3QKbGV0IG11dCBwcmVyZXF1aXNpdGVzID0gVmVjOjpuZXcoJmVudik7CnByZXJlcXVpc2l0ZXMucHVzaF9iYWNrKCJiYXNpY19ydXN0Ii50cnlfaW50bygpLnVud3JhcCgpKTsKcHJlcmVxdWlzaXRlcy5wdXNoX2JhY2soInByb2dyYW1taW5nX2Z1bmRhbWVudGFscyIudHJ5X2ludG8oKS51bndyYXAoKSk7Cgpjb250cmFjdC5hZGRfcHJlcmVxdWlzaXRlKAplbnYuY2xvbmUoKSwKY291cnNlX2NyZWF0b3JfYWRkcmVzcywKImFkdmFuY2VkX3J1c3QiLnRyeV9pbnRvKCkudW53cmFwKCksCnByZXJlcXVpc2l0ZXMKKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipDaXJjdWxhciBkZXBlbmRlbmNpZXMqKjogQ2Fubm90IGFkZCBzZWxmIGFzIHByZXJlcXVpc2l0ZQoqICoqTm9uLWV4aXN0ZW50IGNvdXJzZXMqKjogQWxsIHByZXJlcXVpc2l0ZSBjb3Vyc2VzIG11c3QgZXhpc3QKKiAqKkNyZWF0b3Igb25seSoqOiBPbmx5IGNvdXJzZSBjcmVhdG9yIGNhAAAAEGFkZF9wcmVyZXF1aXNpdGUAAAADAAAAAAAAAAdjcmVhdG9yAAAAABMAAAAAAAAACWNvdXJzZV9pZAAAAAAAABAAAAAAAAAAF3ByZXJlcXVpc2l0ZV9jb3Vyc2VfaWRzAAAAA+oAAAAQAAAAAA==",
        "AAAAAAAAA7lSZW1vdmUgYSBwcmVyZXF1aXNpdGUgZnJvbSBhIGNvdXJzZS4KClRoaXMgZnVuY3Rpb24gcmVtb3ZlcyBhIHNwZWNpZmljIHByZXJlcXVpc2l0ZSBjb3Vyc2UgcmVxdWlyZW1lbnQKZnJvbSB0aGUgdGFyZ2V0IGNvdXJzZS4KCiMgQXJndW1lbnRzCgoqIGBlbnZgIC0gVGhlIFNvcm9iYW4gZW52aXJvbm1lbnQKKiBgY3JlYXRvcmAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgY291cnNlIGNyZWF0b3IKKiBgY291cnNlX2lkYCAtIFRoZSB1bmlxdWUgaWRlbnRpZmllciBvZiB0aGUgY291cnNlCiogYHByZXJlcXVpc2l0ZV9jb3Vyc2VfaWRgIC0gVGhlIElEIG9mIHRoZSBwcmVyZXF1aXNpdGUgY291cnNlIHRvIHJlbW92ZQoKIyBQYW5pY3MKCiogSWYgY291cnNlIGRvZXNuJ3QgZXhpc3QKKiBJZiBjcmVhdG9yIGlzIG5vdCB0aGUgY291cnNlIGNyZWF0b3IKKiBJZiBwcmVyZXF1aXNpdGUgZG9lc24ndCBleGlzdCBmb3IgdGhlIGNvdXJzZQoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBSZW1vdmUgYSBwcmVyZXF1aXNpdGUgZnJvbSBhIGNvdXJzZQpjb250cmFjdC5yZW1vdmVfcHJlcmVxdWlzaXRlKAplbnYuY2xvbmUoKSwKY291cnNlX2NyZWF0b3JfYWRkcmVzcywKImFkdmFuY2VkX3J1c3QiLnRyeV9pbnRvKCkudW53cmFwKCksCiJiYXNpY19ydXN0Ii50cnlfaW50bygpLnVud3JhcCgpCik7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqTm9uLWV4aXN0ZW50IHByZXJlcXVpc2l0ZSoqOiBXaWxsIHBhbmljIGlmIHByZXJlcXVpc2l0ZSBkb2Vzbid0IGV4aXN0CiogKipDcmVhdG9yIG9ubHkqKjogT25seSBjb3Vyc2UgY3JlYXRvciBjYW4gcmVtb3ZlIHByZXJlcXVpc2l0ZXMKKiAqKk5vIGVmZmVjdCoqOiBSZW1vdmluZyBub24tZXhpc3RlbnQgcHJlcmVxdWlzaXRlIGhhcyBubyBlZmZlY3QKKiAqKlN0dWRlbnQgaW1wYWN0Kio6IENvbnNpZGVyIGltcGFjdCBvbiBlbnJvbGxlZCBzdHVkZW50cwAAAAAAABNyZW1vdmVfcHJlcmVxdWlzaXRlAAAAAAMAAAAAAAAAB2NyZWF0b3IAAAAAEwAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAAAAAAWcHJlcmVxdWlzaXRlX2NvdXJzZV9pZAAAAAAAEAAAAAA=",
        "AAAAAAAABABFZGl0IHRoZSBwcmVyZXF1aXNpdGVzIGZvciBhIGNvdXJzZS4KClRoaXMgZnVuY3Rpb24gcmVwbGFjZXMgYWxsIGV4aXN0aW5nIHByZXJlcXVpc2l0ZXMgd2l0aCBhIG5ldyBzZXQKb2YgcHJlcmVxdWlzaXRlIGNvdXJzZXMuCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNyZWF0b3JgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGNvdXJzZSBjcmVhdG9yCiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZQoqIGBuZXdfcHJlcmVxdWlzaXRlc2AgLSBWZWN0b3Igb2YgbmV3IHByZXJlcXVpc2l0ZSBjb3Vyc2UgSURzCgojIFBhbmljcwoKKiBJZiBjb3Vyc2UgZG9lc24ndCBleGlzdAoqIElmIGNyZWF0b3IgaXMgbm90IHRoZSBjb3Vyc2UgY3JlYXRvcgoqIElmIGFueSBwcmVyZXF1aXNpdGUgY291cnNlIGRvZXNuJ3QgZXhpc3QKKiBJZiB0cnlpbmcgdG8gYWRkIHNlbGYgYXMgcHJlcmVxdWlzaXRlCgojIEV4YW1wbGVzCgpgYGBydXN0CmxldCBtdXQgbmV3X3ByZXJlcXVpc2l0ZXMgPSBWZWM6Om5ldygmZW52KTsKbmV3X3ByZXJlcXVpc2l0ZXMucHVzaF9iYWNrKCJ1cGRhdGVkX2NvdXJzZV8xIi50cnlfaW50bygpLnVud3JhcCgpKTsKbmV3X3ByZXJlcXVpc2l0ZXMucHVzaF9iYWNrKCJ1cGRhdGVkX2NvdXJzZV8yIi50cnlfaW50bygpLnVud3JhcCgpKTsKCmNvbnRyYWN0LmVkaXRfcHJlcmVxdWlzaXRlKAplbnYuY2xvbmUoKSwKY291cnNlX2NyZWF0b3JfYWRkcmVzcywKInRhcmdldF9jb3Vyc2UiLnRyeV9pbnRvKCkudW53cmFwKCksCm5ld19wcmVyZXF1aXNpdGVzCik7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqQ29tcGxldGUgcmVwbGFjZW1lbnQqKjogQWxsIG9sZCBwcmVyZXF1aXNpdGVzIGFyZSByZW1vdmVkCiogKipFbXB0eSB2ZWN0b3IqKjogQ2FuIGNsZWFyIGFsbCBwcmVyZXF1aXNpdGVzIHdpdGggZW1wdHkgdmVjdG9yCiogKipDaXJjdWxhciBkZXBlbmRlbmNpZXMqKjogQ2Fubm90IGFkZCBzZWxmIGFzIHByZXJlAAAAEWVkaXRfcHJlcmVxdWlzaXRlAAAAAAAAAwAAAAAAAAAHY3JlYXRvcgAAAAATAAAAAAAAAAljb3Vyc2VfaWQAAAAAAAAQAAAAAAAAABFuZXdfcHJlcmVxdWlzaXRlcwAAAAAAA+oAAAAQAAAAAA==",
        "AAAAAAAABABFZGl0IGNvdXJzZSBpbmZvcm1hdGlvbi4KClRoaXMgZnVuY3Rpb24gYWxsb3dzIHRoZSBjb3Vyc2UgY3JlYXRvciB0byB1cGRhdGUgdmFyaW91cyBhc3BlY3RzCm9mIHRoZSBjb3Vyc2UgdXNpbmcgdGhlIHByb3ZpZGVkIHBhcmFtZXRlcnMuCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNyZWF0b3JgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGNvdXJzZSBjcmVhdG9yCiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZSB0byBlZGl0CiogYHBhcmFtc2AgLSBQYXJhbWV0ZXJzIGNvbnRhaW5pbmcgdGhlIGZpZWxkcyB0byB1cGRhdGUKCiMgUmV0dXJucwoKUmV0dXJucyB0aGUgdXBkYXRlZCBgQ291cnNlYCBvYmplY3QuCgojIFBhbmljcwoKKiBJZiBjb3Vyc2UgZG9lc24ndCBleGlzdAoqIElmIGNyZWF0b3IgaXMgbm90IHRoZSBjb3Vyc2UgY3JlYXRvcgoqIElmIGFueSBmaWVsZCB2YWxpZGF0aW9uIGZhaWxzCgojIEV4YW1wbGVzCgpgYGBydXN0CmxldCBwYXJhbXMgPSBFZGl0Q291cnNlUGFyYW1zIHsKdGl0bGU6IFNvbWUoIlVwZGF0ZWQgQ291cnNlIFRpdGxlIi50cnlfaW50bygpLnVud3JhcCgpKSwKZGVzY3JpcHRpb246IFNvbWUoIlVwZGF0ZWQgZGVzY3JpcHRpb24iLnRyeV9pbnRvKCkudW53cmFwKCkpLApwcmljZTogU29tZSg3NTAwKSwKbGV2ZWw6IFNvbWUoQ291cnNlTGV2ZWw6OkludGVybWVkaWF0ZSksCi4uRGVmYXVsdDo6ZGVmYXVsdCgpCn07CgpsZXQgdXBkYXRlZF9jb3Vyc2UgPSBjb250cmFjdC5lZGl0X2NvdXJzZSgKZW52LmNsb25lKCksCmNvdXJzZV9jcmVhdG9yX2FkZHJlc3MsCiJjb3Vyc2VfMTIzIi50cnlfaW50bygpLnVud3JhcCgpLApwYXJhbXMKKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipQYXJ0aWFsIHVwZGF0ZXMqKjogT25seSBwcm92aWRlZCBmaWVsZHMgYXJlIHVwZGF0ZWQKKiAqKlZhbGlkYXRpb24qKjogQWxsIGZpZWxkcyBtdXN0IHBhc3MgdmFsaWRhdGlvbiBydWxlcwoqICoqQ3JlAAAAC2VkaXRfY291cnNlAAAAAAMAAAAAAAAAB2NyZWF0b3IAAAAAEwAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAAAAAAGcGFyYW1zAAAAAAfQAAAAEEVkaXRDb3Vyc2VQYXJhbXMAAAABAAAH0AAAAAZDb3Vyc2UAAA==",
        "AAAAAAAAA4pBcmNoaXZlIGEgY291cnNlLgoKVGhpcyBmdW5jdGlvbiBtYXJrcyBhIGNvdXJzZSBhcyBhcmNoaXZlZCwgbWFraW5nIGl0IHVuYXZhaWxhYmxlIGZvciBuZXcgZW5yb2xsbWVudHMKd2hpbGUgcHJlc2VydmluZyBleGlzdGluZyBkYXRhIGFuZCBhY2Nlc3MgZm9yIGN1cnJlbnQgc3R1ZGVudHMuCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNyZWF0b3JgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGNvdXJzZSBjcmVhdG9yCiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZSB0byBhcmNoaXZlCgojIFJldHVybnMKClJldHVybnMgdGhlIHVwZGF0ZWQgYENvdXJzZWAgb2JqZWN0IHdpdGggYXJjaGl2ZWQgc3RhdHVzLgoKIyBQYW5pY3MKCiogSWYgY291cnNlIGRvZXNuJ3QgZXhpc3QKKiBJZiBjcmVhdG9yIGlzIG5vdCB0aGUgY291cnNlIGNyZWF0b3IKKiBJZiBjb3Vyc2UgaXMgYWxyZWFkeSBhcmNoaXZlZAoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBBcmNoaXZlIGEgY291cnNlCmxldCBhcmNoaXZlZF9jb3Vyc2UgPSBjb250cmFjdC5hcmNoaXZlX2NvdXJzZSgKJmVudiwKY291cnNlX2NyZWF0b3JfYWRkcmVzcywKImNvdXJzZV8xMjMiLnRyeV9pbnRvKCkudW53cmFwKCkKKTsKYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipBbHJlYWR5IGFyY2hpdmVkKio6IFdpbGwgcGFuaWMgaWYgY291cnNlIGlzIGFscmVhZHkgYXJjaGl2ZWQKKiAqKkNyZWF0b3Igb25seSoqOiBPbmx5IGNvdXJzZSBjcmVhdG9yIGNhbiBhcmNoaXZlIGNvdXJzZQoqICoqU3R1ZGVudCBhY2Nlc3MqKjogQ3VycmVudCBzdHVkZW50cyByZXRhaW4gYWNjZXNzCiogKipSZXZlcnNpYmxlKio6IENvdXJzZSBjYW4gYmUgdW5hcmNoaXZlZCBpZiBuZWVkZWQAAAAAAA5hcmNoaXZlX2NvdXJzZQAAAAAAAgAAAAAAAAAHY3JlYXRvcgAAAAATAAAAAAAAAAljb3Vyc2VfaWQAAAAAAAAQAAAAAQAAB9AAAAAGQ291cnNlAAA=",
        "AAAAAAAAAyZDaGVjayBpZiBhIHVzZXIgaXMgdGhlIGNyZWF0b3Igb2YgYSBzcGVjaWZpYyBjb3Vyc2UuCgpUaGlzIGZ1bmN0aW9uIHZlcmlmaWVzIHdoZXRoZXIgdGhlIHNwZWNpZmllZCB1c2VyIGlzIHRoZSBvcmlnaW5hbCBjcmVhdG9yCm9mIHRoZSBnaXZlbiBjb3Vyc2UuCgojIEFyZ3VtZW50cwoKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNvdXJzZV9pZGAgLSBUaGUgdW5pcXVlIGlkZW50aWZpZXIgb2YgdGhlIGNvdXJzZQoqIGB1c2VyYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSB1c2VyIHRvIGNoZWNrCgojIFJldHVybnMKClJldHVybnMgYHRydWVgIGlmIHRoZSB1c2VyIGlzIHRoZSBjb3Vyc2UgY3JlYXRvciwgYGZhbHNlYCBvdGhlcndpc2UuCgojIFBhbmljcwoKKiBJZiBjb3Vyc2UgZG9lc24ndCBleGlzdAoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBDaGVjayBpZiB1c2VyIGlzIGNvdXJzZSBjcmVhdG9yCmxldCBpc19jcmVhdG9yID0gY29udHJhY3QuaXNfY291cnNlX2NyZWF0b3IoCiZlbnYsCiJjb3Vyc2VfMTIzIi50cnlfaW50bygpLnVud3JhcCgpLAp1c2VyX2FkZHJlc3MKKTsKCmlmIGlzX2NyZWF0b3IgewovLyBVc2VyIGNhbiBlZGl0IHRoaXMgY291cnNlCn0KYGBgCgojIEVkZ2UgQ2FzZXMKCiogKipOb24tZXhpc3RlbnQgY291cnNlKio6IFdpbGwgcGFuaWMgaWYgY291cnNlIGRvZXNuJ3QgZXhpc3QKKiAqKlB1YmxpYyBhY2Nlc3MqKjogQW55b25lIGNhbiBjaGVjayBjcmVhdG9yIHN0YXR1cwoqICoqQ3JlYXRvciB2ZXJpZmljYXRpb24qKjogVXNlZnVsIGZvciBwZXJtaXNzaW9uIGNoZWNrcwAAAAAAEWlzX2NvdXJzZV9jcmVhdG9yAAAAAAAAAgAAAAAAAAAJY291cnNlX2lkAAAAAAAAEAAAAAAAAAAEdXNlcgAAABMAAAABAAAAAQ==",
        "AAAAAAAAAlFMaXN0IGFsbCBhdmFpbGFibGUgY291cnNlIGNhdGVnb3JpZXMuCgpUaGlzIGZ1bmN0aW9uIHJldHJpZXZlcyBhbGwgY291cnNlIGNhdGVnb3JpZXMgdGhhdCBoYXZlIGJlZW4gY3JlYXRlZAppbiB0aGUgc3lzdGVtLgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoKIyBSZXR1cm5zCgpSZXR1cm5zIGEgdmVjdG9yIG9mIGFsbCBhdmFpbGFibGUgYENhdGVnb3J5YCBvYmplY3RzLgoKIyBFeGFtcGxlcwoKYGBgcnVzdAovLyBHZXQgYWxsIGNhdGVnb3JpZXMKbGV0IGNhdGVnb3JpZXMgPSBjb250cmFjdC5saXN0X2NhdGVnb3JpZXMoZW52LmNsb25lKCkpOwpmb3IgY2F0ZWdvcnkgaW4gY2F0ZWdvcmllcyB7CnByaW50bG4hKCJDYXRlZ29yeToge30iLCBjYXRlZ29yeS5uYW1lKTsKfQpgYGAKCiMgRWRnZSBDYXNlcwoKKiAqKkVtcHR5IHN5c3RlbSoqOiBSZXR1cm5zIGVtcHR5IHZlY3RvciBpZiBubyBjYXRlZ29yaWVzIGV4aXN0CiogKipQdWJsaWMgYWNjZXNzKio6IEFueW9uZSBjYW4gbGlzdCBjYXRlZ29yaWVzCiogKipPcmRlcioqOiBDYXRlZ29yaWVzIGFyZSByZXR1cm5lZCBpbiBjcmVhdGlvbiBvcmRlcgAAAAAAAA9saXN0X2NhdGVnb3JpZXMAAAAAAAAAAAEAAAPqAAAH0AAAAAhDYXRlZ29yeQ==",
        "AAAAAAAABABMaXN0IGNvdXJzZXMgd2l0aCBmaWx0ZXJpbmcgYW5kIHBhZ2luYXRpb24uCgpUaGlzIGZ1bmN0aW9uIHJldHJpZXZlcyBjb3Vyc2VzIGJhc2VkIG9uIHRoZSBwcm92aWRlZCBmaWx0ZXJzCndpdGggb3B0aW9uYWwgcGFnaW5hdGlvbiBzdXBwb3J0LgoKIyBBcmd1bWVudHMKCiogYGVudmAgLSBUaGUgU29yb2JhbiBlbnZpcm9ubWVudAoqIGBmaWx0ZXJzYCAtIEZpbHRlcmluZyBjcml0ZXJpYSBmb3IgY291cnNlcwoqIGBsaW1pdGAgLSBPcHRpb25hbCBtYXhpbXVtIG51bWJlciBvZiBjb3Vyc2VzIHRvIHJldHVybgoqIGBvZmZzZXRgIC0gT3B0aW9uYWwgbnVtYmVyIG9mIGNvdXJzZXMgdG8gc2tpcCBmb3IgcGFnaW5hdGlvbgoKIyBSZXR1cm5zCgpSZXR1cm5zIGEgdmVjdG9yIG9mIGBDb3Vyc2VgIG9iamVjdHMgbWF0Y2hpbmcgdGhlIGZpbHRlciBjcml0ZXJpYS4KCiMgRXhhbXBsZXMKCmBgYHJ1c3QKLy8gTGlzdCBmaXJzdCAxMCBjb3Vyc2VzCmxldCBjb3Vyc2VzID0gY29udHJhY3QubGlzdF9jb3Vyc2VzX3dpdGhfZmlsdGVycygKZW52LmNsb25lKCksCkNvdXJzZUZpbHRlcnM6OmRlZmF1bHQoKSwKU29tZSgxMCksClNvbWUoMCkKKTsKCi8vIEZpbHRlciBieSBjYXRlZ29yeQpsZXQgbXV0IGZpbHRlcnMgPSBDb3Vyc2VGaWx0ZXJzOjpkZWZhdWx0KCk7CmZpbHRlcnMuY2F0ZWdvcnkgPSBTb21lKCJQcm9ncmFtbWluZyIudHJ5X2ludG8oKS51bndyYXAoKSk7CmxldCBwcm9ncmFtbWluZ19jb3Vyc2VzID0gY29udHJhY3QubGlzdF9jb3Vyc2VzX3dpdGhfZmlsdGVycygKZW52LmNsb25lKCksCmZpbHRlcnMsClNvbWUoMjApLApOb25lCik7CmBgYAoKIyBFZGdlIENhc2VzCgoqICoqTm8gbWF0Y2hlcyoqOiBSZXR1cm5zIGVtcHR5IHZlY3RvciBpZiBubyBjb3Vyc2VzIG1hdGNoIGZpbHRlcnMKKiAqKkxhcmdlIGxpbWl0cyoqOiBMaW1pdCBzaG91bGQgYmUgcmVhc29uYWJsZSB0byBhdm9pZCBnYXMgaXNzdWVzCiogKipQdWJsaWMgYWNjZXNzKio6IEFueW9uZSBjYW4gbGlzdCBjb3Vyc2VzCiogKipBcmNoAAAAGWxpc3RfY291cnNlc193aXRoX2ZpbHRlcnMAAAAAAAADAAAAAAAAAAdmaWx0ZXJzAAAAB9AAAAANQ291cnNlRmlsdGVycwAAAAAAAAAAAAAFbGltaXQAAAAAAAPoAAAABAAAAAAAAAAGb2Zmc2V0AAAAAAPoAAAABAAAAAEAAAPqAAAH0AAAAAZDb3Vyc2UAAA==",
        "AAAAAAAAAY5FeHBvcnQgYWxsIGNvdXJzZSBkYXRhIGZvciBiYWNrdXAgcHVycG9zZXMgKGFkbWluIG9ubHkpCgpUaGlzIGZ1bmN0aW9uIGV4cG9ydHMgYWxsIGNvdXJzZSBkYXRhIGluY2x1ZGluZyBjb3Vyc2VzLCBjYXRlZ29yaWVzLAptb2R1bGVzLCBnb2FscywgYW5kIHByZXJlcXVpc2l0ZXMgZm9yIGJhY2t1cCBhbmQgcmVjb3ZlcnkgcHVycG9zZXMuCgojIEFyZ3VtZW50cwoqIGBlbnZgIC0gU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjYWxsZXJgIC0gQWRkcmVzcyBwZXJmb3JtaW5nIHRoZSBleHBvcnQgKG11c3QgYmUgYWRtaW4pCgojIFJldHVybnMKKiBgQ291cnNlQmFja3VwRGF0YWAgLSBDb21wbGV0ZSBiYWNrdXAgZGF0YSBzdHJ1Y3R1cmUKCiMgUGFuaWNzCiogSWYgY2FsbGVyIGlzIG5vdCBhbiBhZG1pbgAAAAAAEmV4cG9ydF9jb3Vyc2VfZGF0YQAAAAAAAQAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAQAAB9AAAAAQQ291cnNlQmFja3VwRGF0YQ==",
        "AAAAAAAAAdhJbXBvcnQgY291cnNlIGRhdGEgZnJvbSBiYWNrdXAgKGFkbWluIG9ubHkpCgpUaGlzIGZ1bmN0aW9uIGltcG9ydHMgY291cnNlIGRhdGEgZnJvbSBhIGJhY2t1cCBzdHJ1Y3R1cmUuCk9ubHkgYWRtaW5zIGNhbiBwZXJmb3JtIHRoaXMgb3BlcmF0aW9uLiBUaGlzIHdpbGwgb3ZlcndyaXRlIGV4aXN0aW5nIGRhdGEuCgojIEFyZ3VtZW50cwoqIGBlbnZgIC0gU29yb2JhbiBlbnZpcm9ubWVudAoqIGBjYWxsZXJgIC0gQWRkcmVzcyBwZXJmb3JtaW5nIHRoZSBpbXBvcnQgKG11c3QgYmUgYWRtaW4pCiogYGJhY2t1cF9kYXRhYCAtIEJhY2t1cCBkYXRhIHN0cnVjdHVyZSB0byBpbXBvcnQKCiMgUmV0dXJucwoqIGB1MzJgIC0gTnVtYmVyIG9mIGNvdXJzZXMgaW1wb3J0ZWQKCiMgUGFuaWNzCiogSWYgY2FsbGVyIGlzIG5vdCBhbiBhZG1pbgoqIElmIGJhY2t1cCBkYXRhIGlzIGludmFsaWQKKiBJZiBpbXBvcnQgb3BlcmF0aW9uIGZhaWxzAAAAEmltcG9ydF9jb3Vyc2VfZGF0YQAAAAAAAgAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAAtiYWNrdXBfZGF0YQAAAAfQAAAAEENvdXJzZUJhY2t1cERhdGEAAAABAAAABA==",
        "AAAAAAAAARFHZXQgdGhlIGN1cnJlbnQgY29udHJhY3QgdmVyc2lvbgoKUmV0dXJucyB0aGUgc2VtYW50aWMgdmVyc2lvbiBvZiB0aGUgY3VycmVudCBjb250cmFjdCBkZXBsb3ltZW50LgpUaGlzIGlzIHVzZWZ1bCBmb3IgdHJhY2tpbmcgY29udHJhY3QgdXBncmFkZXMgYW5kIGNvbXBhdGliaWxpdHkuCgojIEFyZ3VtZW50cwoqIGBfZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50ICh1bnVzZWQpCgojIFJldHVybnMKKiBgU3RyaW5nYCAtIFRoZSBjdXJyZW50IGNvbnRyYWN0IHZlcnNpb24AAAAAAAAUZ2V0X2NvbnRyYWN0X3ZlcnNpb24AAAAAAAAAAQAAABA=",
        "AAAAAAAAAR5HZXQgY29udHJhY3QgdmVyc2lvbiBoaXN0b3J5CgpSZXR1cm5zIGEgbGlzdCBvZiBhbGwgdmVyc2lvbnMgdGhhdCBoYXZlIGJlZW4gZGVwbG95ZWQgZm9yIHRoaXMgY29udHJhY3QuClRoaXMgaGVscHMgdHJhY2sgdGhlIGV2b2x1dGlvbiBvZiB0aGUgY29udHJhY3Qgb3ZlciB0aW1lLgoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CgojIFJldHVybnMKKiBgVmVjPFN0cmluZz5gIC0gVmVjdG9yIG9mIHZlcnNpb24gc3RyaW5ncyBpbiBjaHJvbm9sb2dpY2FsIG9yZGVyAAAAAAATZ2V0X3ZlcnNpb25faGlzdG9yeQAAAAAAAAAAAQAAA+oAAAAQ",
        "AAAAAAAAAblDaGVjayBjb21wYXRpYmlsaXR5IGJldHdlZW4gY29udHJhY3QgdmVyc2lvbnMKCkRldGVybWluZXMgaWYgZGF0YSBmcm9tIG9uZSB2ZXJzaW9uIGNhbiBiZSBzYWZlbHkgdXNlZCB3aXRoIGFub3RoZXIgdmVyc2lvbi4KVGhpcyBpcyBjcnVjaWFsIGZvciBtaWdyYXRpb24gcHJvY2Vzc2VzIGFuZCBiYWNrd2FyZCBjb21wYXRpYmlsaXR5LgoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGZyb21fdmVyc2lvbmAgLSBUaGUgc291cmNlIHZlcnNpb24gdG8gY2hlY2sgY29tcGF0aWJpbGl0eSBmcm9tCiogYHRvX3ZlcnNpb25gIC0gVGhlIHRhcmdldCB2ZXJzaW9uIHRvIGNoZWNrIGNvbXBhdGliaWxpdHkgdG8KCiMgUmV0dXJucwoqIGBib29sYCAtIFRydWUgaWYgdGhlIHZlcnNpb25zIGFyZSBjb21wYXRpYmxlLCBmYWxzZSBvdGhlcndpc2UAAAAAAAAVaXNfdmVyc2lvbl9jb21wYXRpYmxlAAAAAAAAAgAAAAAAAAAMZnJvbV92ZXJzaW9uAAAAEAAAAAAAAAAKdG9fdmVyc2lvbgAAAAAAEAAAAAEAAAAB",
        "AAAAAAAAAkFNaWdyYXRlIGNvdXJzZSBkYXRhIGJldHdlZW4gY29udHJhY3QgdmVyc2lvbnMKClBlcmZvcm1zIGRhdGEgbWlncmF0aW9uIGZyb20gb25lIGNvbnRyYWN0IHZlcnNpb24gdG8gYW5vdGhlci4KVGhpcyBmdW5jdGlvbiBoYW5kbGVzIHRoZSB0cmFuc2Zvcm1hdGlvbiBvZiBjb3Vyc2UgZGF0YSBzdHJ1Y3R1cmVzCndoZW4gdXBncmFkaW5nIGNvbnRyYWN0IHZlcnNpb25zLgoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CiogYGNhbGxlcmAgLSBUaGUgYWRkcmVzcyBwZXJmb3JtaW5nIHRoZSBtaWdyYXRpb24gKG11c3QgYmUgY291cnNlIGNyZWF0b3Igb3IgYWRtaW4pCiogYGZyb21fdmVyc2lvbmAgLSBUaGUgc291cmNlIHZlcnNpb24gdG8gbWlncmF0ZSBmcm9tCiogYHRvX3ZlcnNpb25gIC0gVGhlIHRhcmdldCB2ZXJzaW9uIHRvIG1pZ3JhdGUgdG8KCiMgUmV0dXJucwoqIGBib29sYCAtIFRydWUgaWYgbWlncmF0aW9uIHdhcyBzdWNjZXNzZnVsLCBmYWxzZSBvdGhlcndpc2UKCiMgRXZlbnRzCkVtaXRzIGEgbWlncmF0aW9uIGV2ZW50IHVwb24gc3VjY2Vzc2Z1bCBjb21wbGV0aW9uAAAAAAAAE21pZ3JhdGVfY291cnNlX2RhdGEAAAAAAwAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAAxmcm9tX3ZlcnNpb24AAAAQAAAAAAAAAAp0b192ZXJzaW9uAAAAAAAQAAAAAQAAAAE=",
        "AAAAAAAAAP9HZXQgbWlncmF0aW9uIHN0YXR1cyBmb3IgdGhlIGN1cnJlbnQgY29udHJhY3QKClJldHVybnMgaW5mb3JtYXRpb24gYWJvdXQgdGhlIGN1cnJlbnQgbWlncmF0aW9uIHN0YXR1cyBhbmQgYW55CnBlbmRpbmcgbWlncmF0aW9ucyB0aGF0IG5lZWQgdG8gYmUgY29tcGxldGVkLgoKIyBBcmd1bWVudHMKKiBgZW52YCAtIFRoZSBTb3JvYmFuIGVudmlyb25tZW50CgojIFJldHVybnMKKiBgU3RyaW5nYCAtIE1pZ3JhdGlvbiBzdGF0dXMgaW5mb3JtYXRpb24AAAAAFGdldF9taWdyYXRpb25fc3RhdHVzAAAAAAAAAAEAAAAQ" ]),
      options
    )
  }
  public readonly fromJSON = {
    create_course: this.txFromJSON<Course>,
        create_course_category: this.txFromJSON<u128>,
        get_course: this.txFromJSON<Course>,
        get_course_category: this.txFromJSON<Option<CourseCategory>>,
        get_courses_by_instructor: this.txFromJSON<Array<Course>>,
        remove_module: this.txFromJSON<null>,
        add_module: this.txFromJSON<CourseModule>,
        delete_course: this.txFromJSON<null>,
        hello_world: this.txFromJSON<string>,
        edit_goal: this.txFromJSON<CourseGoal>,
        add_goal: this.txFromJSON<CourseGoal>,
        remove_goal: this.txFromJSON<null>,
        add_prerequisite: this.txFromJSON<null>,
        remove_prerequisite: this.txFromJSON<null>,
        edit_prerequisite: this.txFromJSON<null>,
        edit_course: this.txFromJSON<Course>,
        archive_course: this.txFromJSON<Course>,
        is_course_creator: this.txFromJSON<boolean>,
        list_categories: this.txFromJSON<Array<Category>>,
        list_courses_with_filters: this.txFromJSON<Array<Course>>,
        export_course_data: this.txFromJSON<CourseBackupData>,
        import_course_data: this.txFromJSON<u32>,
        get_contract_version: this.txFromJSON<string>,
        get_version_history: this.txFromJSON<Array<string>>,
        is_version_compatible: this.txFromJSON<boolean>,
        migrate_course_data: this.txFromJSON<boolean>,
        get_migration_status: this.txFromJSON<string>
  }
}
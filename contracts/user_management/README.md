# User Management Contract

Manages user accounts, authentication, and permissions within the SkillCert platform.

## Features

### User Management
- User profile creation and management
- User authentication and authorization
- Role-based access control (Student, Instructor, Admin)
- User status management (Active, Inactive, Suspended)

### Pagination
The contract provides two pagination approaches for efficient data retrieval:

#### Traditional Pagination (`list_all_users`)
- Page-based pagination with zero-based page index
- Suitable for small to medium datasets
- Returns `Vec<LightProfile>` with filtered results

#### Cursor-based Pagination (`list_all_users_cursor`)
- Efficient cursor-based pagination to avoid gas limit issues
- Recommended for large datasets
- Returns `PaginatedLightProfiles` with navigation metadata
- Includes `next_cursor`, `has_more`, and `total_count` fields

### Admin Functions
- System initialization
- Admin management (add/remove admins)
- User listing with filtering and pagination
- Access control and authorization

## Usage

### Cursor-based Pagination Example
```rust
// First page
let pagination = PaginationParams {
    cursor: None,
    limit: 50,
};
let result = contract.list_all_users_cursor(env, admin, pagination, None, None);

// Next page using cursor from previous result
if result.has_more {
    let next_pagination = PaginationParams {
        cursor: result.next_cursor,
        limit: 50,
    };
    let next_result = contract.list_all_users_cursor(env, admin, next_pagination, None, None);
}
```
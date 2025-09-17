# Course Access Contract

This directory contains the implementation of the Course Access Contract, which manages user access permissions to courses in the SkillCert platform.

## 📁 Directory Structure

```txt
src/
├── functions/                  # Modular contract functions
│   ├── config.rs              # Contract initialization and configuration
│   ├── grant_access.rs        # Grant course access to users
│   ├── revoke_access.rs       # Revoke course access from users
│   ├── revoke_all_access.rs   # Revoke access for all users from a course
│   ├── list_user_courses.rs   # List courses accessible to a user
│   ├── list_course_access.rs  # List users with access to a course
│   ├── save_profile.rs        # Save/update user profile information
│   ├── transfer_course_access.rs # Transfer access between users
│   ├── has_access.rs          # Check if user has course access
│   └── mod.rs                 # Function module exports
├── error.rs               # Contract error definitions
├── schema.rs              # Data structure definitions (CourseAccess, UserCourses, etc.)
├── lib.rs                # Contract entry point and implementation
└── test.rs               # Unit tests with mock contracts
```

## Quick Overview

- lib.rs: Main contract interface with public functions
- functions/: One function per file for modularity and maintainability
- schema.rs: Data structures and storage keys
- error.rs: Centralized error handling
- test.rs: Comprehensive unit tests

## Getting Started
1. New functions go in functions/ directory
2. Update functions/mod.rs and lib.rs to export new functions
3. Add data structures to schema.rs if needed
4. Include tests in test.rs
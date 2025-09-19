# User Management Contract

This directory contains the implementation of the User Management Contract, which handles user registration, profile management, authentication, and administrative functions for the SkillCert platform.

## 📁 Directory Structure

```txt
src/
├── functions/                    # Modular contract functions
│   ├── admin_management.rs      # System initialization and admin management
│   ├── create_user_profile.rs   # User registration and profile creation
│   ├── get_user_by_id.rs        # User profile retrieval with access control
│   ├── delete_user.rs           # User account deactivation (soft delete)
│   ├── is_admin.rs              # Admin privilege verification
│   ├── list_all_registered_users.rs # User listing with pagination and filtering
│   ├── list_users_with_access.rs    # Course access user listing (legacy)
│   └── mod.rs                   # Function module exports
├── error.rs               # Contract error definitions (25+ error types)
├── schema.rs              # Data structures (UserProfile, AdminConfig, roles, etc.)
├── lib.rs                # Contract entry point and public interface
└── test.rs               # Comprehensive integration tests
```

## Quick Overview

- lib.rs: Main contract interface with 10+ public functions for user and admin management
- functions/: Modular functions organized by feature (user management, admin operations)
- schema.rs: Complex data structures including UserProfile, LightProfile, roles, and admin configuration
- error.rs: 25+ specific error types for comprehensive validation and security
- test.rs: Integration tests covering user lifecycle and admin operations

## Getting Started
1. System Setup: Use initialize_system to set up super admin and configuration
2. User Registration: Use create_user_profile for new user registration
3. Profile Access: Use get_user_by_id for profile retrieval (self or admin)
4. Admin Operations: Use add_admin, remove_admin for admin management
5. User Management: Use delete_user for account deactivation
6. User Discovery: Use list_all_users for admin user listing with filters


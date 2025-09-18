# User Profile Contract

This directory contains the implementation of the User Profile Contract, which provides read-only access to user profile information with privacy controls and permission checks in the SkillCert platform.

## 📁 Directory Structure

```txt
src/
├── functions/                    # Modular contract functions
│   ├── get_user_profile.rs      # User profile retrieval with privacy controls
│   └── mod.rs                   # Function module exports
├── schema.rs              # Data structures (UserProfile, DataKey)
├── lib.rs                # Contract entry point and public interface
└── test.rs               # Basic function existence tests
```

## Quick Overview

- lib.rs: Main contract interface with 2 public functions for profile retrieval
- functions/: Single module focused on profile access functionality
- schema.rs: Simple data structures for user profiles and storage keys
- test.rs: Basic tests to verify function availability

## Getting Started
1. Profile Retrieval: Use get_user_profile for internal access
2. Privacy-Aware Access: Use get_user_profile_with_privacy for user-facing features
3. Integration: Connect with user_management contract for complete user lifecycle

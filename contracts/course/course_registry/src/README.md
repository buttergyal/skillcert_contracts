# Course Registry Contract

This directory contains the implementation of the Course Registry Contract, which manages course creation, modification, categorization, and comprehensive course lifecycle management in the SkillCert platform.

## 📁 Directory Structure

```txt
src/
├── functions/                    # Modular contract functions
│   ├── access_control.rs        # Authorization and admin management
│   ├── create_course.rs         # Create new courses with metadata
│   ├── get_course.rs           # Retrieve course information by ID
│   ├── edit_course.rs          # Update course metadata and settings
│   ├── delete_course.rs        # Remove courses and cleanup
│   ├── archive_course.rs       # Archive courses (soft delete)
│   ├── is_course_creator.rs    # Verify course ownership
│   ├── get_courses_by_instructor.rs # List courses by instructor
│   ├── add_module.rs           # Add modules to courses
│   ├── remove_module.rs        # Remove modules from courses
│   ├── list_modules.rs         # List course modules
│   ├── add_goal.rs             # Add learning goals to courses
│   ├── edit_goal.rs            # Edit course goals
│   ├── remove_goal.rs          # Remove course goals
│   ├── create_prerequisite.rs   # Add course prerequisites
│   ├── edit_prerequisite.rs    # Update prerequisite requirements
│   ├── remove_prerequisite.rs  # Remove prerequisite requirements
│   ├── get_prerequisites_by_course.rs # Get course prerequisites
│   ├── create_course_category.rs # Create course categories (admin)
│   ├── get_course_category.rs  # Retrieve category information
│   ├── list_categories.rs      # List all available categories
│   ├── list_courses_with_filters.rs # Advanced course filtering and search
│   ├── utils.rs                # Utility functions (ID generation, string manipulation)
│   └── mod.rs                  # Function module exports
├── error.rs               # Contract error definitions and handling
├── schema.rs              # Data structures (Course, Module, Goal, Category, etc.)
├── lib.rs                # Contract entry point and public interface
└── test.rs               # Comprehensive unit tests
```

## Quick Overview

- lib.rs: Main contract interface with 20+ public functions for course management
- functions/: Modular functions organized by feature (courses, modules, goals, prerequisites, categories)
- schema.rs: Complex data structures including Course, CourseModule, CourseGoal, CourseCategory, CourseFilters
- error.rs: 25+ specific error types for comprehensive error handling
- test.rs: Extensive test coverage for all contract functionality

## Getting Started
1. Core Operations: Use create_course, get_course, edit_course for basic course management
2. Modules: Use add_module, remove_module to structure course content
3. Goals: Use add_goal, edit_goal, remove_goal for learning objectives
4. Prerequisites: Use create_prerequisite, edit_prerequisite for course dependencies
5. Categories: Use create_course_category (admin), list_categories for organization
6. Search: Use list_courses_with_filters for advanced course discovery

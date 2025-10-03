// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{
    AdminConfig, DataKey, Permission, RolePermissions, UserPermissions, UserRole,
};
use soroban_sdk::{Address, Env, Vec};

/// Initializes default role-based permissions for the RBAC system.
///
/// Sets default permissions for the following roles:
/// - `Student`
/// - `Instructor`
/// - `Moderator`
/// - `Support`
/// - `Admin`
/// - `SuperAdmin` (all permissions)
///
/// Marks the default permissions as initialized in persistent storage.
///
/// # Arguments
///
/// * `env` - The Soroban environment.
///
/// # Returns
///
/// * () - Updates persistent storage with role-permission mappings.
pub fn initialize_default_permissions(env: &Env) {
    // Student permissions
    let student_permissions = Vec::from_array(
        env,
        [Permission::ViewCourses, Permission::ViewUsers],
    );
    let student_role_perms = RolePermissions {
        role: UserRole::Student,
        permissions: student_permissions,
    };
    env.storage().persistent().set(
        &DataKey::RolePermissions(UserRole::Student),
        &student_role_perms,
    );

    // Instructor permissions
    let instructor_permissions = Vec::from_array(
        env,
        [
            Permission::ViewCourses,
            Permission::CreateCourses,
            Permission::EditCourses,
            Permission::ManageCourseAccess,
            Permission::ViewUsers,
        ],
    );
    let instructor_role_perms = RolePermissions {
        role: UserRole::Instructor,
        permissions: instructor_permissions,
    };
    env.storage().persistent().set(
        &DataKey::RolePermissions(UserRole::Instructor),
        &instructor_role_perms,
    );

    // Moderator permissions
    let moderator_permissions = Vec::from_array(
        env,
        [
            Permission::ViewCourses,
            Permission::ViewUsers,
            Permission::EditCourses,
            Permission::ModerateContent,
            Permission::ManageCourseAccess,
        ],
    );
    let moderator_role_perms = RolePermissions {
        role: UserRole::Moderator,
        permissions: moderator_permissions,
    };
    env.storage().persistent().set(
        &DataKey::RolePermissions(UserRole::Moderator),
        &moderator_role_perms,
    );

    // Support permissions
    let support_permissions = Vec::from_array(
        env,
        [
            Permission::ViewUsers,
            Permission::ProvideSupport,
            Permission::ViewSupport,
            Permission::ViewCourses,
        ],
    );
    let support_role_perms = RolePermissions {
        role: UserRole::Support,
        permissions: support_permissions,
    };
    env.storage().persistent().set(
        &DataKey::RolePermissions(UserRole::Support),
        &support_role_perms,
    );
    // Admin permissions
    let admin_permissions = Vec::from_array(
        env,
        [
            Permission::ViewUsers,
            Permission::EditUsers,
            Permission::DeleteUsers,
            Permission::CreateUsers,
            Permission::ViewCourses,
            Permission::CreateCourses,
            Permission::EditCourses,
            Permission::DeleteCourses,
            Permission::ManageCourseAccess,
            Permission::ViewAnalytics,
            Permission::ModerateContent,
            Permission::ProvideSupport,
            Permission::ViewSupport,
        ],
    );
    let admin_role_perms = RolePermissions {
        role: UserRole::Admin,
        permissions: admin_permissions,
    };
    env.storage().persistent().set(
        &DataKey::RolePermissions(UserRole::Admin),
        &admin_role_perms,
    );

    // SuperAdmin permissions (all permissions)
    let super_admin_permissions = Vec::from_array(
        env,
        [
            Permission::ViewUsers,
            Permission::EditUsers,
            Permission::DeleteUsers,
            Permission::CreateUsers,
            Permission::ViewCourses,
            Permission::CreateCourses,
            Permission::EditCourses,
            Permission::DeleteCourses,
            Permission::ManageCourseAccess,
            Permission::ManageSystem,
            Permission::ManageAdmins,
            Permission::ViewAnalytics,
            Permission::ModerateContent,
            Permission::ProvideSupport,
            Permission::ViewSupport,
        ],
    );
    let super_admin_role_perms = RolePermissions {
        role: UserRole::SuperAdmin,
        permissions: super_admin_permissions,
    };
    env.storage().persistent().set(
        &DataKey::RolePermissions(UserRole::SuperAdmin),
        &super_admin_role_perms,
    );

    // Mark default permissions as initialized
    env.storage()
        .persistent()
        .set(&DataKey::DefaultRolePermissions, &true);
}

/// Retrieves a user's role from storage.
///
/// If no role is set, the user defaults to `Student`.
///
/// # Arguments
///
/// * `env` - The Soroban environment.
/// * `user` - The user's address.
///
/// # Returns
///
/// * `UserRole` - The stored role, or `Student` if unset.
pub fn get_user_role(env: &Env, user: &Address) -> UserRole {
    env.storage()
        .persistent()
        .get::<DataKey, UserRole>(&DataKey::UserRole(user.clone()))
        .unwrap_or(UserRole::Student)
}
/// Sets a user's role in storage (admin only).
///
/// Requires that the caller has the `ManageAdmins` permission.
///
/// # Arguments
///
/// * `env` - The Soroban environment.
/// * `caller` - The address of the caller (must be authorized).
/// * `user` - The user whose role is being updated.
/// * `role` - The role to assign.
///
/// # Returns
///
/// * () - Updates storage or raises `AccessDenied`.
pub fn set_user_role(env: Env, caller: Address, user: Address, role: UserRole) {
    caller.require_auth();

    if !has_permission(&env, &caller, &Permission::ManageAdmins) {
        handle_error(&env, Error::AccessDenied);
    }

    env.storage()
        .persistent()
        .set(&DataKey::UserRole(user), &role);
}

/// Checks if a user has a specific permission.
///
/// Verifies both role-based permissions and user-specific overrides.
///
/// # Arguments
///
/// * `env` - The Soroban environment.
/// * `user` - The user's address.
/// * `permission` - The permission to check.
///
/// # Returns
///
/// * `bool` - `true` if the user has the permission, `false` otherwise.
pub fn has_permission(env: &Env, user: &Address, permission: &Permission) -> bool {
    let initialized: bool = env
        .storage()
        .persistent()
        .get(&DataKey::DefaultRolePermissions)
        .unwrap_or(false);
    if !initialized {
        initialize_default_permissions(env);
    }

    let user_role = get_user_role(env, user);

    if is_super_admin(env, user) {
        return true;
    }

    let role_permissions: Option<RolePermissions> = env
        .storage()
        .persistent()
        .get(&DataKey::RolePermissions(user_role));

    let mut has_role_permission = false;
    if let Some(role_perms) = role_permissions {
        has_role_permission = role_perms.permissions.iter().any(|p| p == *permission);
    }

    let user_permissions: Option<UserPermissions> = env
        .storage()
        .persistent()
        .get(&DataKey::UserPermissions(user.clone()));

    if let Some(user_perms) = user_permissions {
        if user_perms.revoked_permissions.iter().any(|p| p == *permission) {
            return false;
        }
        if user_perms.granted_permissions.iter().any(|p| p == *permission) {
            return true;
        }
    }

    has_role_permission
}

/// Determines if a user is the super admin.
///
/// # Arguments
///
/// * `env` - The Soroban environment.
/// * `user` - The user's address.
///
/// # Returns
///
/// * `bool` - `true` if the user is the configured super admin.
pub fn is_super_admin(env: &Env, user: &Address) -> bool {
    let config: Option<AdminConfig> = env.storage().persistent().get(&DataKey::AdminConfig);
    match config {
        Some(cfg) if cfg.initialized => cfg.super_admin == *user,
        _ => false,
    }
}

/// Determines if a user has admin-level privileges.
///
/// Checks if the user is either a super admin or has the `Admin` role.
///
/// # Arguments
///
/// * `env` - The Soroban environment.
/// * `user` - The user's address.
///
/// # Returns
///
/// * `bool` - `true` if the user is an admin.
pub fn is_admin(env: &Env, user: &Address) -> bool {
    if is_super_admin(env, user) {
        return true;
    }

    let admins: Option<Vec<Address>> = env
        .storage()
        .persistent()
        .get::<DataKey, Vec<Address>>(&DataKey::Admins);
    if let Some(list) = admins {
        if list.iter().any(|a| a == *user) {
            return true;
        }
    }

    let user_role = get_user_role(env, user);
    matches!(user_role, UserRole::Admin | UserRole::SuperAdmin)
}

/// Grants a specific permission to a user (admin only).
///
/// Overrides role-based permissions if needed.
///
/// # Arguments
///
/// * `env` - The Soroban environment.
/// * `caller` - The address of the caller (must be authorized and have `ManageAdmins`).
/// * `user` - The user to grant permission to.
/// * `permission` - The permission to grant.
///
/// # Returns
///
/// * () - Updates user-specific permissions in storage.
pub fn grant_user_permission(env: Env, caller: Address, user: Address, permission: Permission) {
    caller.require_auth();

    if !has_permission(&env, &caller, &Permission::ManageAdmins) {
        handle_error(&env, Error::AccessDenied);
    }

    let mut user_permissions: UserPermissions = env
        .storage()
        .persistent()
        .get(&DataKey::UserPermissions(user.clone()))
        .unwrap_or_else(|| UserPermissions {
            user: user.clone(),
            granted_permissions: Vec::new(&env),
            revoked_permissions: Vec::new(&env),
        });

    let mut new_revoked = Vec::new(&env);
    for p in user_permissions.revoked_permissions.iter() {
        if p != permission {
            new_revoked.push_back(p);
        }
    }
    user_permissions.revoked_permissions = new_revoked;

    if !user_permissions.granted_permissions.iter().any(|p| p == permission) {
        user_permissions.granted_permissions.push_back(permission);
    }

    env.storage()
        .persistent()
        .set(&DataKey::UserPermissions(user), &user_permissions);
}

/// Revokes a specific permission from a user (admin only).
///
/// Overrides role-based permissions if needed.
///
/// # Arguments
///
/// * `env` - The Soroban environment.
/// * `caller` - The address of the caller (must be authorized and have `ManageAdmins`).
/// * `user` - The user to revoke permission from.
/// * `permission` - The permission to revoke.
///
/// # Returns
///
/// * () - Updates user-specific permissions in storage.
pub fn revoke_user_permission(env: Env, caller: Address, user: Address, permission: Permission) {
    caller.require_auth();

    if !has_permission(&env, &caller, &Permission::ManageAdmins) {
        handle_error(&env, Error::AccessDenied);
    }

    let mut user_permissions: UserPermissions = env
        .storage()
        .persistent()
        .get(&DataKey::UserPermissions(user.clone()))
        .unwrap_or_else(|| UserPermissions {
            user: user.clone(),
            granted_permissions: Vec::new(&env),
            revoked_permissions: Vec::new(&env),
        });

    let mut new_granted = Vec::new(&env);
    for p in user_permissions.granted_permissions.iter() {
        if p != permission {
            new_granted.push_back(p);
        }
    }
    user_permissions.granted_permissions = new_granted;

    if !user_permissions.revoked_permissions.iter().any(|p| p == permission) {
        user_permissions.revoked_permissions.push_back(permission);
    }

    env.storage()
        .persistent()
        .set(&DataKey::UserPermissions(user), &user_permissions);
}

/// Retrieves all effective permissions for a user.
///
/// Combines role-based permissions with user-specific overrides.
/// Grants take precedence over revokes, and super admins receive all permissions.
///
/// # Arguments
///
/// * `env` - The Soroban environment.
/// * `caller` - The caller (must be authorized).
/// * `user` - The user whose permissions are being checked.
///
/// # Returns
///
/// * `Vec<Permission>` - A vector containing all effective permissions.
pub fn get_user_permissions(env: Env, caller: Address, user: Address) -> Vec<Permission> {
    caller.require_auth();

    if caller != user && !has_permission(&env, &caller, &Permission::ViewUsers) {
        handle_error(&env, Error::AccessDenied);
    }

    let initialized: bool = env
        .storage()
        .persistent()
        .get(&DataKey::DefaultRolePermissions)
        .unwrap_or(false);
    if !initialized {
        initialize_default_permissions(&env);
    }

    let mut final_permissions = Vec::new(&env);

    if is_super_admin(&env, &user) {
        return Vec::from_array(
            &env,
            [
                Permission::ViewUsers,
                Permission::EditUsers,
                Permission::DeleteUsers,
                Permission::CreateUsers,
                Permission::ViewCourses,
                Permission::CreateCourses,
                Permission::EditCourses,
                Permission::DeleteCourses,
                Permission::ManageCourseAccess,
                Permission::ManageSystem,
                Permission::ManageAdmins,
                Permission::ViewAnalytics,
                Permission::ModerateContent,
                Permission::ProvideSupport,
                Permission::ViewSupport,
            ],
        );
    }

    let user_role = get_user_role(&env, &user);
    let role_permissions: Option<RolePermissions> = env
        .storage()
        .persistent()
        .get(&DataKey::RolePermissions(user_role));

    if let Some(role_perms) = role_permissions {
        for permission in role_perms.permissions.iter() {
            final_permissions.push_back(permission);
        }
    }

    let user_permissions: Option<UserPermissions> = env
        .storage()
        .persistent()
        .get(&DataKey::UserPermissions(user));

    if let Some(user_perms) = user_permissions {
        for permission in user_perms.granted_permissions.iter() {
            if !final_permissions.iter().any(|p| p == permission) {
                final_permissions.push_back(permission);
            }
        }

        let mut filtered_permissions = Vec::new(&env);
        for permission in final_permissions.iter() {
            if !user_perms.revoked_permissions.iter().any(|p| p == permission) {
                filtered_permissions.push_back(permission);
            }
        }
        final_permissions = filtered_permissions;
    }

    final_permissions
}
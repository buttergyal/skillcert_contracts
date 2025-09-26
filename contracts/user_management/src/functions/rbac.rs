// SPDX-License-Identifier: MIT
// Copyright (c) 2025 SkillCert

use crate::error::{handle_error, Error};
use crate::schema::{
    AdminConfig, DataKey, Permission, RolePermissions, UserPermissions, UserRole,
};
use soroban_sdk::{Address, Env, Vec};

/// Initialize default role permissions for the RBAC system
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

/// Get user's role from storage
pub fn get_user_role(env: &Env, user: &Address) -> UserRole {
    env.storage()
        .persistent()
        .get::<DataKey, UserRole>(&DataKey::UserRole(user.clone()))
        .unwrap_or(UserRole::Student) // Default to Student if no role is set
}

/// Set user's role in storage (admin only)
pub fn set_user_role(env: Env, caller: Address, user: Address, role: UserRole) {
    caller.require_auth();

    // Check if caller has ManageAdmins permission
    if !has_permission(&env, &caller, &Permission::ManageAdmins) {
        handle_error(&env, Error::AccessDenied);
    }

    // Store the user's role
    env.storage()
        .persistent()
        .set(&DataKey::UserRole(user), &role);
}

/// Check if a user has a specific permission
pub fn has_permission(env: &Env, user: &Address, permission: &Permission) -> bool {
    // Initialize default permissions if not already done
    let initialized: bool = env
        .storage()
        .persistent()
        .get(&DataKey::DefaultRolePermissions)
        .unwrap_or(false);
    if !initialized {
        initialize_default_permissions(env);
    }

    // Get user's role
    let user_role = get_user_role(env, user);

    // Check if user is super admin (has all permissions)
    if is_super_admin(env, user) {
        return true;
    }

    // Get role-based permissions
    let role_permissions: Option<RolePermissions> = env
        .storage()
        .persistent()
        .get(&DataKey::RolePermissions(user_role));

    let mut has_role_permission = false;
    if let Some(role_perms) = role_permissions {
        has_role_permission = role_perms.permissions.iter().any(|p| p == *permission);
    }

    // Get user-specific permission overrides
    let user_permissions: Option<UserPermissions> = env
        .storage()
        .persistent()
        .get(&DataKey::UserPermissions(user.clone()));

    if let Some(user_perms) = user_permissions {
        // Check if permission is explicitly revoked
        if user_perms.revoked_permissions.iter().any(|p| p == *permission) {
            return false;
        }
        
        // Check if permission is explicitly granted
        if user_perms.granted_permissions.iter().any(|p| p == *permission) {
            return true;
        }
    }

    has_role_permission
}

/// Check if user is super admin
pub fn is_super_admin(env: &Env, user: &Address) -> bool {
    let config: Option<AdminConfig> = env.storage().persistent().get(&DataKey::AdminConfig);
    match config {
        Some(cfg) if cfg.initialized => cfg.super_admin == *user,
        _ => false,
    }
}

/// Check if user has admin privileges (super admin or admin role)
pub fn is_admin(env: &Env, user: &Address) -> bool {
    // Check if user is super admin
    if is_super_admin(env, user) {
        return true;
    }

    // Check if user is in admin list (legacy support)
    let admins: Option<Vec<Address>> = env
        .storage()
        .persistent()
        .get::<DataKey, Vec<Address>>(&DataKey::Admins);
    if let Some(list) = admins {
        if list.iter().any(|a| a == *user) {
            return true;
        }
    }

    // Check if user has Admin role
    let user_role = get_user_role(env, user);
    matches!(user_role, UserRole::Admin | UserRole::SuperAdmin)
}

/// Grant additional permission to a user (admin only)
pub fn grant_user_permission(env: Env, caller: Address, user: Address, permission: Permission) {
    caller.require_auth();

    // Check if caller has ManageAdmins permission
    if !has_permission(&env, &caller, &Permission::ManageAdmins) {
        handle_error(&env, Error::AccessDenied);
    }

    // Get existing user permissions or create new
    let mut user_permissions: UserPermissions = env
        .storage()
        .persistent()
        .get(&DataKey::UserPermissions(user.clone()))
        .unwrap_or_else(|| UserPermissions {
            user: user.clone(),
            granted_permissions: Vec::new(&env),
            revoked_permissions: Vec::new(&env),
        });

    // Remove from revoked if present
    let mut new_revoked = Vec::new(&env);
    for p in user_permissions.revoked_permissions.iter() {
        if p != permission {
            new_revoked.push_back(p);
        }
    }
    user_permissions.revoked_permissions = new_revoked;

    // Add to granted if not already present
    if !user_permissions.granted_permissions.iter().any(|p| p == permission) {
        user_permissions.granted_permissions.push_back(permission);
    }

    // Store updated permissions
    env.storage()
        .persistent()
        .set(&DataKey::UserPermissions(user), &user_permissions);
}

/// Revoke permission from a user (admin only)
pub fn revoke_user_permission(env: Env, caller: Address, user: Address, permission: Permission) {
    caller.require_auth();

    // Check if caller has ManageAdmins permission
    if !has_permission(&env, &caller, &Permission::ManageAdmins) {
        handle_error(&env, Error::AccessDenied);
    }

    // Get existing user permissions or create new
    let mut user_permissions: UserPermissions = env
        .storage()
        .persistent()
        .get(&DataKey::UserPermissions(user.clone()))
        .unwrap_or_else(|| UserPermissions {
            user: user.clone(),
            granted_permissions: Vec::new(&env),
            revoked_permissions: Vec::new(&env),
        });

    // Remove from granted if present
    let mut new_granted = Vec::new(&env);
    for p in user_permissions.granted_permissions.iter() {
        if p != permission {
            new_granted.push_back(p);
        }
    }
    user_permissions.granted_permissions = new_granted;

    // Add to revoked if not already present
    if !user_permissions.revoked_permissions.iter().any(|p| p == permission) {
        user_permissions.revoked_permissions.push_back(permission);
    }

    // Store updated permissions
    env.storage()
        .persistent()
        .set(&DataKey::UserPermissions(user), &user_permissions);
}

/// Get all permissions for a user (combining role and user-specific permissions)
pub fn get_user_permissions(env: Env, caller: Address, user: Address) -> Vec<Permission> {
    caller.require_auth();

    // Check if caller has permission to view user permissions
    if caller != user && !has_permission(&env, &caller, &Permission::ViewUsers) {
        handle_error(&env, Error::AccessDenied);
    }

    // Initialize default permissions if not already done
    let initialized: bool = env
        .storage()
        .persistent()
        .get(&DataKey::DefaultRolePermissions)
        .unwrap_or(false);
    if !initialized {
        initialize_default_permissions(&env);
    }

    let mut final_permissions = Vec::new(&env);

    // If user is super admin, return all permissions
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

    // Get role-based permissions
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

    // Apply user-specific permission overrides
    let user_permissions: Option<UserPermissions> = env
        .storage()
        .persistent()
        .get(&DataKey::UserPermissions(user));

    if let Some(user_perms) = user_permissions {
        // Add granted permissions
        for permission in user_perms.granted_permissions.iter() {
            if !final_permissions.iter().any(|p| p == permission) {
                final_permissions.push_back(permission);
            }
        }

        // Remove revoked permissions
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

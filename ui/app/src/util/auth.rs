//! Authentication utilities

use components::prelude::*;

/// User role enumeration
#[derive(Clone, PartialEq, Debug)]
pub enum UserRole {
    ProcurementOfficer,
    ProcurementManager,
    BudgetHolder,
    Approver,
    ContractManager,
    SupplierAdmin,
    AuditViewer,
    SystemAdmin,
}

/// User information
#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub department: String,
    pub avatar_initials: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: "U001".to_string(),
            name: "Thabo Mokoena".to_string(),
            email: "thabo.mokoena@sars.gov.za".to_string(),
            role: UserRole::ProcurementManager,
            department: "SCM".to_string(),
            avatar_initials: "TM".to_string(),
        }
    }
}

/// Authentication state
#[derive(Clone, Debug)]
pub struct AuthState {
    pub authenticated: bool,
    pub user: Option<User>,
    pub token: Option<String>,
}

impl Default for AuthState {
    fn default() -> Self {
        // Default to authenticated for demo purposes
        Self {
            authenticated: true,
            user: Some(User::default()),
            token: Some("demo-token".to_string()),
        }
    }
}

/// Check if user has a specific role
pub fn has_role(auth: &AuthState, role: UserRole) -> bool {
    auth.user.as_ref().map(|u| u.role == role).unwrap_or(false)
}

/// Check if user has any of the specified roles
pub fn has_any_role(auth: &AuthState, roles: &[UserRole]) -> bool {
    auth.user
        .as_ref()
        .map(|u| roles.contains(&u.role))
        .unwrap_or(false)
}

/// Authentication provider component
#[component]
pub fn auth_provider(auth_state: Signal<AuthState>, children: Vec<View>) -> View {
    if !auth_state.get().authenticated {
        return view! {
            <div class="auth-required">
                <h2>"Authentication Required"</h2>
                <p>"Please log in to access the eProcurement system."</p>
            </div>
        };
    }

    view! {
        for child in children {
            {child}
        }
    }
}

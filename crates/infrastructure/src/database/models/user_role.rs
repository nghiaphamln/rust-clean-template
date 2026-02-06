use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Type, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    #[sqlx(rename = "Admin")]
    Admin,
    #[sqlx(rename = "User")]
    User,
}

// Type aliases for cleaner conversions
type DomainUserRole = rust_clean_domain::UserRole;

impl From<DomainUserRole> for UserRole {
    fn from(role: DomainUserRole) -> Self {
        match role {
            DomainUserRole::Admin => Self::Admin,
            DomainUserRole::User => Self::User,
        }
    }
}

impl From<UserRole> for DomainUserRole {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Admin => Self::Admin,
            UserRole::User => Self::User,
        }
    }
}

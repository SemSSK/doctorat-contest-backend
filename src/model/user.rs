use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Type)]
pub enum Role {
    Admin,
    CFD,
    ViceDoyen,
    Professor,
    Applicant,
}

pub const ALL_ROLES: [Role; 5] = [
    Role::Admin,
    Role::CFD,
    Role::ViceDoyen,
    Role::Professor,
    Role::Applicant,
];

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub role: Option<Role>,
    pub domaine: Option<String>,
    pub specialty: Option<String>,
}

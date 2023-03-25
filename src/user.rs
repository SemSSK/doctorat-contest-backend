use serde::{Deserialize, Serialize};
use sqlx::Decode;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Decode)]
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
    pub email: String,
    pub password: Option<String>,
    pub role: Option<Role>,
}

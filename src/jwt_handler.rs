use crate::model::user::User;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

/// Describes the content of the Jwt
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    user_data: User,
    exp: usize,
}

impl Claims {
    pub fn new(user: User) -> Self {
        let exp = (Utc::now() + Duration::minutes(15)).timestamp_millis() as usize;
        let user = User {
            id: user.id,
            email: user.email,
            password: None,
            role: user.role,
            domaine: user.domaine,
            specialty: user.specialty,
        };
        Self {
            sub: "me".to_string(),
            user_data: user,
            exp,
        }
    }
}

fn get_key() -> EncodingKey {
    EncodingKey::from_secret(env::var("KEY").expect("no KEY in .env file").as_bytes())
}
fn get_key_string() -> String {
    env::var("KEY").expect("no KEY in .env file")
}
fn check_expiry(c: Claims) -> Option<User> {
    let now = Utc::now().timestamp_millis() as usize;
    if now > c.exp {
        None
    } else {
        Some(c.user_data)
    }
}

/// Generates jwt for a given user
/// does not include the password in the jwt for security reasons
pub fn encode_to_jwt(user: User) -> String {
    let claims = Claims::new(user);
    match encode(&Header::default(), &claims, &get_key()) {
        Ok(t) => t,
        Err(e) => {
            eprint!("{}", e);
            panic!()
        }
    }
}

/// Verifies the JWT token
/// returns None if the token is invalid
/// returns Some(User) otherwise
pub fn validate_jwt(jwt: &str) -> Option<User> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some("me".to_string());
    match decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(get_key_string().as_bytes()),
        &validation,
    ) {
        Ok(c) => check_expiry(c.claims),
        _ => {
            println!("Invalid token");
            None
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    session_id: Option<i32>,
    code: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub session_id: Option<i32>,
    pub code: String,
}

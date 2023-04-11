use serde::{Serialize, Deserialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct Reclamation {
    pub applicant_id: i32,
    pub module_id: String,
    pub session_id: i32,
    pub content: String
}
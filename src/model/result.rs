use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    pub applicant_id: i32,
    pub module_id: String,
    pub session_id: i32,

    pub corrector_1_id: i32,
    pub corrector_2_id: i32,
    pub corrector_3_id: i32,

    pub note_1: Option<i32>,
    pub note_2: Option<i32>,
    pub note_3: Option<i32>,

    pub display_to_applicant: Option<bool>,
    pub display_to_cfd: Option<bool>,
}


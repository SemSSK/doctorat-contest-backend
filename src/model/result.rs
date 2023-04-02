use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    applicant_id: i32,
    module_id: i32,
    session_id: i32,

    corrector_1_id: Option<i32>,
    corrector_2_id: Option<i32>,
    corrector_3_id: Option<i32>,

    note_1: Option<i32>,
    note_2: Option<i32>,
    note_3: Option<i32>,

    display: bool,
}

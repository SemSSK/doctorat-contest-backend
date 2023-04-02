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

impl Result {
    pub fn calc_note_final(&self) -> Option<i32> {
        let (Some(n1),Some(n2)) = (self.note_1,self.note_2) else {
        return None;
      };
        match self.note_3 {
            None => Some((n1 + n2) / 2),
            Some(n) => {
                let n1_n3_diff = (n - n1).abs();
                let n2_n3_diff = (n - n2).abs();
                if n1_n3_diff > n2_n3_diff {
                    Some(((n + n2) / 2))
                } else {
                    Some(((n + n1) / 2))
                }
            }
        }
    }

    pub fn is_valid_note(&self) -> bool {
        let (Some(n1),Some(n2)) = (self.note_1,self.note_2) else {
        return false;
      };
        (n1 - n2) > 2
    }
}

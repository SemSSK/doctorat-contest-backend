use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualPlatform {
    pub vd_id: i32,
    pub name: String,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Option<i32>,
    pub virtual_platform_id: i32,
    pub cfd_id: i32,
    pub starting_time: i64,
    pub ending_time: i64,
    pub room_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Announcement {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub session_id: i32,
}

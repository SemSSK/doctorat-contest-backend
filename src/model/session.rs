use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    id: Option<i32>,
    virtual_platform_id: i32,
    cfd_id: i32,
    starting_time: u64,
    ending_time: u64,
    room_number: u32,
    specialty: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Announcement {
    id: Option<i32>,
    title: String,
    content: String,
    session_id: i32,
}

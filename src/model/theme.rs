use serde::{Serialize, Deserialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct Theme {
    pub session_id: i32,
    pub professor_id: i32,
    pub title: String,
    pub content: String,
}
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{
    model::{module, session},
    ServerState,
};

mod db {}

// Session management
#[post("/")]
pub async fn create_session(
    session: web::Json<session::Session>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

#[get("/{id}")]
pub async fn get_session(
    id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

#[get("/")]
pub async fn get_sessions(data: web::Data<ServerState>, request: HttpRequest) -> HttpResponse {
    todo!()
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateSessionInput {
    cfd_id: i32,
    starting_time: u64,
    ending_time: u64,
    room_number: u64,
}

#[put("/")]
pub async fn update_session(
    session: web::Json<UpdateSessionInput>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

#[delete("/{id}")]
pub async fn delete_session(
    id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

// Module management

#[post("/module")]
pub async fn create_module(
    module: web::Json<module::Module>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

#[delete("/module")]
pub async fn delete_module(
    module: web::Json<module::Module>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

// Applicant management
#[derive(Debug, Serialize, Deserialize)]
struct ApplicantAffectation {
    session_id: i32,
    applicant_id: i32,
    encoding: String,
}

#[post("/applicant")]
pub async fn affect_applicant(
    af: web::Json<ApplicantAffectation>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

#[delete("/applicant")]
pub async fn delete_applicant(
    af: web::Json<ApplicantAffectation>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

// Announcement Management
#[post("/announcement")]
pub async fn create_announcement(
    announcement: web::Json<session::Announcement>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

#[delete("/announcement/{id}")]
pub async fn delete_announcement(
    id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    todo!()
}

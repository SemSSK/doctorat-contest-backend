use actix_web::{get,post,put,HttpRequest, web, Either, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::{ServerState, model::{theme, user}, apis::authentication::secure_function};


const API_ROLES : [user::Role; 1] = [user::Role::Professor];

mod db {
    use crate::model::{user, session, result, theme};

    use super::AddMarkInput;

    pub async fn get_session(
        cfd: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<session::Session>> {
        todo!()
    }

    pub async fn get_corrections(
        session_id: i32,
        cfd: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<result::Result>> {
        todo!()
    }

    pub async fn add_mark(
        note: AddMarkInput,
        cfd: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<()> {
        todo!()
    } 

    pub async fn get_themes(
        session_id: i32,
        cfd: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<theme::Theme>> {
        todo!()
    }

    pub async fn add_theme(
        t: theme::Theme,
        cfd: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<()> {
        todo!()
    }

    pub async fn check_accepted_applicants(
        session_id: i32,
        cfd: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<user::User>> {
        todo!()
    } 

}



#[get("/session")]
pub async fn get_sessions(
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse, impl Responder> {
    
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_session(u, &data.pool),
        &API_ROLES, 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(s) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    Either::Right(web::Json(s))
}

#[get("/corrections/session={id}")]
pub async fn get_corrections(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_corrections(session_id.0,u, &data.pool),
        &API_ROLES, 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(r) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    Either::Right(web::Json(r))
}

#[derive(Debug,Serialize,Deserialize)]
pub struct AddMarkInput {
    pub applicant_id: i32,
    pub module_id: i32,
    pub session_id: i32,
    pub note: i32
}

#[put("/corrections")]
pub async fn add_mark(
    note: web::Json<AddMarkInput>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::add_mark(note.0,u, &data.pool),
        &API_ROLES, 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(_) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    Either::Right(HttpResponse::Ok().finish())
}

#[get("/theme/session={id}")]
pub async fn get_themes(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_themes(session_id.0,u, &data.pool),
        &API_ROLES, 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(themes) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    Either::Right(web::Json(themes))
}

#[post("/theme")]
pub async fn add_theme(
    t: web::Json<theme::Theme>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::add_theme(t.0,u, &data.pool),
        &API_ROLES, 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(_) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    Either::Right(HttpResponse::Ok().finish())
}

#[get("/theme-applicants/session={id}")]
pub async fn check_accepted_applicants(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::check_accepted_applicants(session_id.0,u, &data.pool),
        &API_ROLES, 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(apps) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    Either::Right(web::Json(apps))
}
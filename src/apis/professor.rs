use actix_web::{get,post,put,HttpRequest, web, Either, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::{ServerState, model::{theme, user}, apis::authentication::secure_function};


const API_ROLES : [user::Role; 1] = [user::Role::Professor];

mod db {
    use crate::model::{user, session, result, theme};

    use super::AddMarkInput;

    pub async fn get_session(
        professor: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<session::Session>> {
        sqlx::query_as!(
            session::Session,
            r#"
            select
                s.id as 'id?',
                s.virtual_platform_id,
                s.cfd_id,
                s.starting_time,
                s.ending_time,
                s.room_number
            from
                Edl.Session s, Edl.monitor_affectation ma, Edl.User p
            where
                ma.session_id = s.id    and
                ma.professor_id = p.id  and
                p.id = ?
            "#,
            professor.id
        ).fetch_all(pool)
        .await
    }

    pub async fn get_corrections(
        session_id: i32,
        professor: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<(result::Result,Option<String>)>> {
        sqlx::query!(
            r#"
            select
                r.applicant_id,
                r.module_id,
                r.session_id,
                r.corrector_1_id,
                r.corrector_2_id,
                r.corrector_3_id,
                r.note_1,
                r.note_2,
                r.note_3,
                aa.encoding
            from
                Edl.Session s, Edl.Result r, Edl.applicant_affectation aa
            where
                (
                    r.corrector_1_id = ?    or
                    r.corrector_2_id = ?    or
                    (
                        r.corrector_3_id = ? and
                        abs(r.note_1 - r.note_2) > 3
                    )
                ) and
                s.id = ?                and
                r.session_id = s.id     and
                aa.session_id = s.id    and
                aa.applicant_id = r.applicant_id
            "#,
            professor.id,
            professor.id,
            professor.id,
            session_id
        ).fetch_all(pool)
        .await
        .map(|vr| vr.into_iter().map(|r| {
            let mut note_1 = None;
            let mut note_2 = None;
            let mut note_3 = None;
            if r.corrector_1_id.unwrap() == professor.id.unwrap() {
                note_1 = r.note_1;
            }
            else if r.corrector_2_id.unwrap() == professor.id.unwrap() {
                note_2 = r.note_2;
            }
            else if r.corrector_3_id.unwrap() == professor.id.unwrap() {
                note_3 = r.note_3;
            }

            (result::Result {
                applicant_id: r.applicant_id,
                module_id: r.module_id,
                session_id: r.session_id,
                corrector_1_id: r.corrector_1_id.unwrap(),
                corrector_2_id: r.corrector_2_id.unwrap(),
                corrector_3_id: r.corrector_3_id.unwrap(),
                note_1,
                note_2,
                note_3,
                display_to_applicant: None,
                display_to_cfd: None
            },
            r.encoding)
        }).collect()) 
    }

    pub async fn add_mark(
        note: AddMarkInput,
        professor: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<()> {
        if sqlx::query!(
            r#"
            update Edl.Result
            set 
                note_1 = case
            when 
                corrector_1_id = ? then ?
                else note_1
            end,
                note_2 = case
            when
                corrector_2_id = ? then ?
                else note_2
            end,
                note_3 = case
            when
                corrector_3_id = ? then ?
                else note_3
            end,
                display_to_cfd = case
            when
                (note_1 - note_2) <= 3 or
                note_3 != null
                then true
                else display_to_cfd
            end
            where
                applicant_id = ? and
                module_id = ? and
                session_id = ? 
            "#,
            professor.id,note.note,
            professor.id,note.note,
            professor.id,note.note,
            note.applicant_id,
            note.module_id,
            note.session_id
        ).execute(pool)
        .await?
        .rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound)
        }
        Ok(())
    } 

    pub async fn get_themes(
        session_id: i32,
        professor: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<theme::Theme>> {
        sqlx::query_as!(
            theme::Theme,
            r#"
            select
                session_id,
                professor_id,
                title,
                content
            from
                Edl.Theme
            where
                session_id = ? and
                professor_id = ?
            "#,
            session_id,
            professor.id
        ).fetch_all(pool)
        .await
    }

    pub async fn add_theme(
        t: theme::Theme,
        professor: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<()> {
        if sqlx::query!(
            r#"
            insert into Edl.Theme
                (session_id,professor_id,title,content)
            select 
                s.id, ?, ?, ?
            from
                Edl.Session s, Edl.User u
            where
                s.cfd_id = u.id and
                u.specialty = ? 
            "#,
            professor.id,
            t.title,
            t.content,
            professor.specialty
        ).execute(pool)
        .await?
        .rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    pub async fn check_accepted_applicants(
        session_id: i32,
        professor: user::User, 
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<user::User>> {
        sqlx::query_as!(
            user::User,
            r#"
            select
                u.id as 'id?', 
                u.email,
                "" as 'password?', 
                u.role as 'role?: user::Role',
                u.domaine as 'domaine?',
                u.specialty as 'specialty?'
            from
                Edl.Choice c, Edl.User u
            where
                c.result = true and
                c.professor_id = ? and
                c.applicant_id = u.id and
                c.session_id = ?
            "#,
            professor.id,
            session_id
        ).fetch_all(pool)
        .await
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
    pub module_id: String,
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
use serde::{Serialize, Deserialize};
use actix_web::{get,post, web, HttpRequest, Either, HttpResponse, Responder};
use crate::{model::{theme, user, reclamation}, ServerState, apis::authentication::secure_function};

const API_ROLES : [user::Role;1] = [user::Role::Applicant];

#[derive(Debug,Serialize,Deserialize)]
pub struct ClassmentEntry {
    pub email: String,
    pub name: String,
    pub classment: usize,
    pub avg: Option<f64>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ThemeDisplay {
    pub t: theme::Theme,
    pub professor: String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ThemeId {
    pub session_id: i32,
    pub professor_id: i32,
    pub order: i32
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ResultDisplay{
    pub note: Option<f64>,
    pub module : String
}

mod db {
    use bigdecimal::*;

    use crate::model::{session, user, reclamation, theme, module};

    use super::{ClassmentEntry, ThemeDisplay, ThemeId, ResultDisplay};

    pub async fn get_sessions(
        applicant: user::User,
        pool:&sqlx::MySqlPool
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
                Edl.Session s, Edl.User edu, Edl.applicant_affectation aa
            where
                s.id = aa.session_id and
                edu.id = aa.applicant_id and
                aa.applicant_id = ?
            "#,
            applicant.id
        ).fetch_all(pool)
        .await
    }

    pub async fn get_modules(
        applicant: user::User,
        pool:&sqlx::MySqlPool,
        session_id:i32
    ) -> sqlx::Result<Vec<module::Module>> {
        sqlx::query_as!(
            module::Module,
            r#"
            select
                m.session_id as 'session_id?',
                code
            from
                Edl.Session s, Edl.User edu, Edl.applicant_affectation aa,Edl.Module m
            where
                m.session_id = s.id and
                s.id = aa.session_id and
                edu.id = aa.applicant_id and
                aa.applicant_id = ? and
                s.id = ?
            "#,
            applicant.id,
            session_id
        ).fetch_all(pool)
        .await
    }
    pub async fn get_announcements(
        applicant: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<session::Announcement>> {
        sqlx::query_as!(
            session::Announcement,
            r#"
            select
                a.id as 'id?',
                a.title,
                a.content,
                a.session_id
            from
                Edl.Announcement a, Edl.Session s, Edl.applicant_affectation aa
            where
                a.session_id = s.id and
                aa.session_id = s.id and
                aa.applicant_id = ? and
                s.id = ? 
            "#,
            applicant.id,
            session_id
        ).fetch_all(pool)
        .await
    }

    pub async fn get_classment(
        _applicant: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<ClassmentEntry>> {
        Ok(sqlx::query!(
                    r#"
                    select
                        u.name,
                        u.email,
                        avg(case
                                when r.note_3 is null then ((r.note_1 + r.note_2)/2)
                                when abs(r.note_3 - r.note_2) >= abs(r.note_3 - r.note_1) 
                                    then ((r.note_3 + r.note_1)/2) 
                                else ((r.note_3 + r.note_2)/2) 
                            end) as 'avg!:BigDecimal'
                    from
                        Edl.Result r, 
                        Edl.applicant_affectation aa, 
                        Edl.Session s, 
                        Edl.User u
                    where
                        r.session_id = s.id and
                        r.applicant_id = aa.applicant_id and
                        s.id = aa.session_id and
                        u.id = aa.applicant_id and
                        s.id = ?
                    group by u.id
                    order by 3 desc
                    "#,
                    session_id
                ).fetch_all(pool)
                .await?
                .into_iter()
                .enumerate()
                .map(|(i,r)| ClassmentEntry {
                    avg: r.avg.to_f64(),
                    classment:i+1,
                    email:r.email.to_owned(),
                    name: r.name.to_owned()
                })
                .collect())
    }

    pub async fn get_personal_results(
        applicant: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<ResultDisplay>> {
        Ok(sqlx::query!(
                    r#"
                        select
                            r.module_id as 'module',
                            case
                                when r.note_3 is null then ((r.note_1 + r.note_2)/2)
                                when abs(r.note_3 - r.note_2) >= abs(r.note_3 - r.note_1) 
                                    then ((r.note_3 + r.note_1)/2) 
                                else ((r.note_3 + r.note_2)/2) 
                            end as 'note! : BigDecimal'
                        from
                            Edl.Result r, Edl.User u, Edl.Session s
                        where
                            s.id = r.session_id and
                            r.applicant_id = u.id and
                            r.display_to_applicant = true and
                            s.id = ? and
                            u.id = ?
                        group by r.module_id
                    "#,
                    session_id,
                    applicant.id
                ).fetch_all(pool)
                .await?
                .into_iter()
                .map(|r| ResultDisplay {
                    module: r.module,
                    note: r.note.to_f64()
                })
                .collect())
    }

    pub async fn get_reclamations(
        applicant: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<reclamation::Reclamation>> {
        sqlx::query_as!(
            reclamation::Reclamation,
            r#"
            select
                re.applicant_id,
                re.module_id,
                re.session_id,
                re.content
            from
                Edl.Reclamation re
            where
                re.applicant_id = ? and
                re.module_id = ?
            "#,
            applicant.id,
            session_id
        ).fetch_all(pool)
        .await
    }

    pub async fn add_reclamation(
        applicant: user::User,
        rec: reclamation::Reclamation,
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<()> {
        if sqlx::query!(
            r#"
            insert into Edl.Reclamation
                (applicant_id,module_id,session_id,content)
            select
                u.id,
                r.module_id,
                r.session_id,
                ?
            from
                Edl.Result r, Edl.User u
            where
                r.applicant_id = u.id and
                u.id = ?
            "#,
            rec.content,
            applicant.id
        ).execute(pool)
        .await?
        .rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound); 
        }
        Ok(())
    }


    pub async fn get_themes(
        applicant: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<Vec<ThemeDisplay>> {
        Ok(sqlx::query!(
                    r#"
                    select  
                        t.session_id,
                        t.professor_id,
                        t.title,
                        t.content,
                        prof.email
                    from
                        Edl.Theme t, 
                        Edl.Session s, 
                        Edl.applicant_affectation aa, 
                        Edl.User prof
                    where
                        t.session_id = s.id and
                        aa.session_id = s.id and
                        t.professor_id = prof.id and
                        aa.applicant_id = ? and
                        s.id = ?
                    "#,
                    session_id,
                    applicant.id
                ).fetch_all(pool)
                .await?
                .iter()
                .map(|r| ThemeDisplay {
                    professor: r.email.to_owned(),
                    t: theme::Theme { 
                        session_id: r.session_id, 
                        professor_id: r.professor_id, 
                        title: r.title.to_owned(), 
                        content: r.content.to_owned() }
                })
                .collect())
    }

    pub async fn choose_theme(
        applicant: user::User,
        theme_id: ThemeId,
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<()> {
        if sqlx::query!(
            r#"
            insert into Edl.Choice
                (applicant_id,session_id,professor_id,order_of_priority,result)
            select
                aa.applicant_id,
                t.session_id,
                t.professor_id,
                ?,
                null
            from
                Edl.Theme t, Edl.applicant_affectation aa
            where
                t.session_id = aa.session_id and
                t.professor_id = ? and
                aa.applicant_id = ?
            "#,
            theme_id.order,
            theme_id.professor_id,
            applicant.id
        ).execute(pool)
        .await?
        .rows_affected() != 1 {
            return Err(sqlx::Error::RowNotFound)
        }
        Ok(())
    }

    pub async fn check_theme_result(
        applicant: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
    ) -> sqlx::Result<theme::Theme> {
        sqlx::query_as!(
            theme::Theme,
            r#"
            select
                t.session_id,
                t.professor_id,
                t.title,
                t.content
            from
                Edl.Theme t, Edl.Session s, Edl.Choice c
            where
                t.session_id = s.id and
                c.session_id = s.id and
                s.id = ? and
                c.applicant_id = ? and
                c.result = true
            "#,
            session_id,
            applicant.id
        ).fetch_one(pool)
        .await
    }

}

#[get("/session")]
pub async fn get_sessions(
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_sessions(u, &data.pool),
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

#[get("/module/session={id}")]
pub async fn get_modules(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_modules(u, &data.pool,session_id.0),
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

#[get("/announcement/session={id}")]
pub async fn get_announcements(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_announcements(u, session_id.0, &data.pool),
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

#[get("/classement/session={id}")]
pub async fn get_classment(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_classment(u, session_id.0, &data.pool),
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

#[get("/results/session={id}")]
pub async fn get_personal_results(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_personal_results(u, session_id.0, &data.pool),
        &API_ROLES, 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(s) = f.await else {
        return Either::Left(HttpResponse::NotFound().finish());
    };

    Either::Right(web::Json(s))
}

#[get("/reclamation/session={id}")]
pub async fn get_reclamations(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_reclamations(u, session_id.0, &data.pool),
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

#[post("/reclamation")]
pub async fn add_reclamations(
    rec: web::Json<reclamation::Reclamation>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::add_reclamation(u, rec.0, &data.pool),
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
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::get_themes(u, session_id.0, &data.pool),
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

#[post("/theme")]
pub async fn choose_theme(
    theme_id: web::Json<ThemeId>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::choose_theme(u, theme_id.0, &data.pool),
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
pub async fn check_theme_result(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder>  {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::check_theme_result(u, session_id.0, &data.pool),
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

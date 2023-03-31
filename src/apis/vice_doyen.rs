use crate::{
    apis::authentication::secure_function,
    model::{module, session, user},
    ServerState,
};
use actix_web::{delete, get, post, put, web, Either, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

mod db {
    use sqlx::MySqlPool;

    use crate::model::{session, user};

    use super::UpdateSessionInput;

    pub async fn create_session(
        vd: user::User,
        s: session::Session,
        pool: &MySqlPool,
    ) -> sqlx::Result<()> {
        if sqlx::query!(
            r#"
            insert into Edl.Session
                (virtual_platform_id,cfd_id,starting_time,ending_time,room_number)
            select
                ?,id,?,?,?
            from 
                Edl.User
            where 
                id = ? and 
                role like "CFD"
            "#,
            vd.id,
            s.starting_time,
            s.ending_time,
            s.room_number,
            s.cfd_id
        )
        .execute(pool)
        .await?
        .rows_affected()
            == 0
        {
            return Err(sqlx::Error::RowNotFound);
        };
        Ok(())
    }

    pub async fn get_session(
        vd: user::User,
        id: i32,
        pool: &MySqlPool,
    ) -> sqlx::Result<session::Session> {
        sqlx::query_as!(
            session::Session,
            r#"
            select
                id as 'id?',
                virtual_platform_id,
                cfd_id,
                starting_time,
                ending_time,
                room_number
            from 
                Edl.Session
            where
                id = ? and 
                virtual_platform_id = ?
            "#,
            id,
            vd.id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_sessions(
        vd: user::User,
        pool: &MySqlPool,
    ) -> sqlx::Result<Vec<session::Session>> {
        sqlx::query_as!(
            session::Session,
            r#"
            select 
                id as 'id?',
                virtual_platform_id,
                cfd_id,
                starting_time,
                ending_time,
                room_number
            from 
                Edl.Session
            where
                virtual_platform_id = ?
            "#,
            vd.id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update_session(
        vd: user::User,
        data: UpdateSessionInput,
        pool: &MySqlPool,
    ) -> sqlx::Result<()> {
        if sqlx::query!(
            r#"
            update 
                Edl.Session
            set 
                cfd_id = ?,
                starting_time = ?,
                ending_time = ?,
                room_number = ?
            where
                id = ? and
                virtual_platform_id in
                    (
                        select vd_id
                        from Edl.VirtualPlatform
                        where vd_id = ?
                    ) and
                ? in (
                    select id
                    from Edl.User
                    where role like "CFD"
                ) 
            "#,
            data.cfd_id,
            data.starting_time,
            data.ending_time,
            data.room_number,
            data.id,
            vd.id,
            data.cfd_id
        )
        .execute(pool)
        .await?
        .rows_affected()
            != 1
        {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    pub async fn delete_session(vd: user::User, id: i32, pool: &MySqlPool) -> sqlx::Result<()> {
        if sqlx::query!(
            r#"
            delete from Edl.Session
            where
                id = ? and
                virtual_platform_id in 
                    (
                        select vd_id
                        from VirtualPlatform
                        where vd_id = ?
                    )
            "#,
            id,
            vd.id
        )
        .execute(pool)
        .await?
        .rows_affected()
            != 1
        {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    pub mod dmodule {
        use sqlx::MySqlPool;

        use crate::model::{module, user};

        pub async fn create_module(
            vd: user::User,
            m: module::Module,
            pool: &MySqlPool,
        ) -> sqlx::Result<()> {
            if sqlx::query!(
                r#"
                insert into Edl.Module (code,session_id)
                select distinct
                    ?,
                    id
                from 
                    Edl.Session
                where
                    id = ? and
                    virtual_platform_id = ?
                "#,
                m.code,
                m.session_id,
                vd.id
            )
            .execute(pool)
            .await?
            .rows_affected()
                != 1
            {
                return Err(sqlx::Error::RowNotFound);
            }
            Ok(())
        }

        pub async fn delete_module(
            vd: user::User,
            module: module::Module,
            pool: &MySqlPool,
        ) -> sqlx::Result<()> {
            if sqlx::query!(
                r#"
                delete from Edl.Module
                where
                    code = ? and
                    session_id = ? and
                    session_id in 
                        (
                            select 
                                id
                            from 
                                Edl.Session
                            where
                                virtual_platform_id = ?
                        )
                "#,
                module.code,
                module.session_id,
                vd.id
            )
            .execute(pool)
            .await?
            .rows_affected()
                != 1
            {
                return Err(sqlx::Error::RowNotFound);
            }
            Ok(())
        }
    }

    pub mod applicant {
        use sqlx::{MySql, MySqlPool};

        use crate::{apis::vice_doyen::ApplicantAffectation, model::user};

        pub async fn get_possible_applicants(
            vd: user::User,
            pool: &MySqlPool,
        ) -> sqlx::Result<Vec<user::User>> {
            todo!()
        }
        pub async fn get_current_applicants(
            vd: user::User,
            session_id: i32,
            pool: &MySqlPool,
        ) -> sqlx::Result<Vec<user::User>> {
            todo!()
        }
        pub async fn get_applicant(
            vd: user::User,
            app_id: i32,
            pool: &MySqlPool,
        ) -> sqlx::Result<user::User> {
            todo!()
        }

        pub async fn affect_applicant(
            vd: user::User,
            af: ApplicantAffectation,
            pool: &MySqlPool,
        ) -> sqlx::Result<()> {
            if sqlx::query!(
                r#"
                insert into applicant_affectation
                    (applicant_id,session_id,encoding)
                select distinct
                    u.id,s.id,?
                from Edl.User u, Edl.Session s, Edl.User cfd
                where
                    u.id = ? and
                    u.role = "Applicant" and
                    u.specialty = cfd.specialty and
                    cfd.id = s.cfd_id and
                    s.id = ? and
                    s.virtual_platform_id = ?
                "#,
                af.encoding,
                af.applicant_id,
                af.session_id,
                vd.id
            )
            .execute(pool)
            .await?
            .rows_affected()
                != 1
            {
                return Err(sqlx::Error::RowNotFound);
            }
            Ok(())
        }

        pub async fn delete_applicant(
            vd: user::User,
            af: ApplicantAffectation,
            pool: &MySqlPool,
        ) -> sqlx::Result<()> {
            if sqlx::query!(
                r#"
                delete from Edl.applicant_affectation
                where
                    applicant_id = ? and
                    session_id = ? and
                    session_id in 
                        (
                            select id
                            from Edl.Session
                            where
                                virtual_platform_id = ?
                        )
                "#,
                af.applicant_id,
                af.session_id,
                vd.id
            )
            .execute(pool)
            .await?
            .rows_affected()
                != 1
            {
                return Err(sqlx::Error::RowNotFound);
            }
            Ok(())
        }
    }

    pub mod announcement {
        use sqlx::MySqlPool;

        use crate::model::{session, user};

        pub async fn create_announcement(
            vd: user::User,
            a: session::Announcement,
            pool: &MySqlPool,
        ) -> sqlx::Result<()> {
            if sqlx::query!(
                r#"
                insert into Edl.Announcement
                    (title,content,session_id)
                select distinct
                    ?,?,?
                from 
                    Edl.Session
                where
                    id = ? and
                    virtual_platform_id = ?
                "#,
                a.title,
                a.content,
                a.session_id,
                a.session_id,
                vd.id
            )
            .execute(pool)
            .await?
            .rows_affected()
                != 1
            {
                return Err(sqlx::Error::RowNotFound);
            }
            Ok(())
        }

        pub async fn delete_announcement(
            vd: user::User,
            id: i32,
            pool: &MySqlPool,
        ) -> sqlx::Result<()> {
            if sqlx::query!(
                r#"
                delete from Edl.Announcement
                where
                    id = ? and
                    session_id in 
                        (
                            select id
                            from 
                                Edl.Session
                            where
                                virtual_platform_id = ?
                        )
                "#,
                id,
                vd.id
            )
            .execute(pool)
            .await?
            .rows_affected()
                != 1
            {
                return Err(sqlx::Error::RowNotFound);
            }
            Ok(())
        }
    }
}

// Session management
#[post("/")]
pub async fn create_session(
    session: web::Json<session::Session>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(|_| true, |u| db::create_session(u, session.0, &data.pool), &[user::Role::ViceDoyen], request) else {
        return HttpResponse::Forbidden().finish();
    };

    let Ok(_) = f.await else {
        return HttpResponse::Forbidden().finish()
    };

    HttpResponse::Ok().finish()
}

#[get("/{id}")]
pub async fn get_session(
    id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(|_| true, |u| db::get_session(u,id.0, &data.pool), &[user::Role::ViceDoyen], request) else {
        return Either::Left(HttpResponse::NotFound().finish());
    };
    let Ok(s) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    Either::Right(web::Json(s))
}

#[get("/")]
pub async fn get_sessions(
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(|_| true, |u| db::get_sessions(u, &data.pool), &[user::Role::ViceDoyen], request) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(ss) = f.await else {
        return Either::Left(HttpResponse::NotFound().finish());
    };
    Either::Right(web::Json(ss))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSessionInput {
    pub id: i32,
    pub cfd_id: i32,
    pub starting_time: u64,
    pub ending_time: u64,
    pub room_number: u64,
}

#[put("/")]
pub async fn update_session(
    session: web::Json<UpdateSessionInput>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(|_| true, |u| db::update_session(u, session.0, &data.pool), &[user::Role::ViceDoyen], request) else {
        return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
        return HttpResponse::NotFound().finish();
    };
    HttpResponse::Ok().finish()
}

#[delete("/{id}")]
pub async fn delete_session(
    id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(|_| true, |u| db::delete_session(u,id.0, &data.pool), &[user::Role::ViceDoyen], request) else {
        return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
        return HttpResponse::NotFound().finish();
    };
    HttpResponse::Ok().finish()
}

// Module management

#[post("/module")]
pub async fn create_module(
    module: web::Json<module::Module>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(|_| true, |u| db::dmodule::create_module(u, module.0, &data.pool), &[user::Role::ViceDoyen], request) else {
        return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
        return HttpResponse::Forbidden().finish();
    };
    HttpResponse::Ok().finish()
}

#[delete("/module")]
pub async fn delete_module(
    module: web::Json<module::Module>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(|_| true, |u| db::dmodule::delete_module(u, module.0, &data.pool), &[user::Role::ViceDoyen], request) else {
        return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
        return HttpResponse::NotFound().finish();
    };
    HttpResponse::Ok().finish()
}

// Applicant management
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicantAffectation {
    pub session_id: i32,
    pub applicant_id: i32,
    pub encoding: String,
}

#[post("/applicant")]
pub async fn affect_applicant(
    af: web::Json<ApplicantAffectation>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(
        |_| true,
        |u| db::applicant::affect_applicant(u, af.0, &data.pool),
        &[user::Role::ViceDoyen],
        request,
    ) else {
        return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
        return HttpResponse::Forbidden().finish();
    };
    HttpResponse::Ok().finish()
}

#[delete("/applicant")]
pub async fn delete_applicant(
    af: web::Json<ApplicantAffectation>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(
        |_| true,
        |u| db::applicant::delete_applicant(u, af.0, &data.pool),
        &[user::Role::ViceDoyen],
        request,
    ) else {
        return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
        return HttpResponse::NotFound().finish();
    };
    HttpResponse::Ok().finish()
}

// Announcement Management
#[post("/announcement")]
pub async fn create_announcement(
    announcement: web::Json<session::Announcement>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(
        |_| true,
        |u| db::announcement::create_announcement(u, announcement.0, &data.pool),
        &[user::Role::ViceDoyen],
        request,
    ) else {
        return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
        return HttpResponse::Forbidden().finish();
    };
    HttpResponse::Ok().finish()
}

#[delete("/announcement/{id}")]
pub async fn delete_announcement(
    id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(
        |_| true,
        |u| db::announcement::delete_announcement(u,id.0, &data.pool),
        &[user::Role::ViceDoyen],
        request,
    ) else {
        return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
        return HttpResponse::NotFound().finish();
    };
    HttpResponse::Ok().finish()
}

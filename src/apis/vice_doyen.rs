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

    pub async fn get_cfd(
        vd: user::User,
        pool: &MySqlPool,
    ) -> sqlx::Result<user::User> {
        sqlx::query_as!(
            user::User,
            r#"
            select
                id as 'id?', 
                email,
                name,
                "" as 'password?', 
                role as 'role?: user::Role',
                domaine as 'domaine?',
                specialty as 'specialty?'
            from
                Edl.User
            where
                domaine = ? and
                role = "CFD"
            "#,
            vd.domaine
        ).fetch_one(pool)
        .await
    }

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

        pub async fn get_modules(
            vd: user::User,
            s_id: i32,
            pool: &MySqlPool
        ) -> sqlx::Result<Vec<module::Module>> {
            sqlx::query_as!(
                module::Module,
                r#"
                select
                    m.code,
                    m.session_id as 'session_id?'
                from 
                    Edl.Module m, Edl.Session s
                where
                    m.session_id = ? and
                    m.session_id = s.id and
                    s.virtual_platform_id = ?
                "#,
                s_id,
                vd.id
            ).fetch_all(pool)
            .await
        }

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
        use sqlx::MySqlPool;

        use crate::{apis::vice_doyen::ApplicantAffectation, model::user};

        pub async fn get_possible_applicants(
            vd: user::User,
            pool: &MySqlPool,
        ) -> sqlx::Result<Vec<user::User>> {
            sqlx::query_as!(
                user::User,
                r#"
                select
                    id as 'id?', 
                    email,
                    name,
                    "" as 'password?', 
                    role as 'role?: user::Role',
                    domaine as 'domaine?',
                    specialty as 'specialty?'
                from
                    Edl.User
                where
                    domaine = ? and
                    role = "Applicant"
                "#,
                vd.domaine
            ).fetch_all(pool)
            .await
        }

        pub async fn get_applicant_affectation(
            vd: user::User,
            session_id: i32,
            pool: &MySqlPool
        ) -> sqlx::Result<Vec<ApplicantAffectation>> {
            Ok(sqlx::query!(
                            r#"
                                select
                                    session_id,
                                    applicant_id,
                                    encoding as 'encoding?'
                                from
                                    Edl.applicant_affectation
                                where
                                    session_id = ?
                            "#,
                            session_id
                        ).fetch_all(pool)
                        .await?
                        .iter()
                        .map(|r| {
                            let encoding = match r.encoding.as_ref() {
                                Some(e) => e.clone(),
                                None => "".to_string(),
                            };
                            ApplicantAffectation {
                                applicant_id: r.applicant_id,
                                session_id: r.session_id,
                                encoding
                            }
                        }).collect()
                    )
        }

        pub async fn get_current_applicants(
            vd: user::User,
            session_id: i32,
            pool: &MySqlPool,
        ) -> sqlx::Result<Vec<user::User>> {
            sqlx::query_as!(
                user::User,
                r#"
                select
                    u.id as 'id?', 
                    u.email,
                    u.name,
                    "" as 'password?', 
                    u.role as 'role?: user::Role',
                    u.domaine as 'domaine?',
                    u.specialty as 'specialty?'
                from
                    Edl.User u, Edl.Session s, Edl.applicant_affectation a
                where 
                    s.id = ? and
                    s.virtual_platform_id = ? and
                    a.session_id = s.id and
                    a.applicant_id = u.id
                "#,
                session_id,
                vd.id
            ).fetch_all(pool)
            .await
        }
        pub async fn get_applicant(
            vd: user::User,
            app_id: i32,
            pool: &MySqlPool,
        ) -> sqlx::Result<user::User> {
            sqlx::query_as!(
                user::User,
                r#"
                select
                    id as 'id?', 
                    email,
                    name,
                    "" as 'password?', 
                    role as 'role?: user::Role',
                    domaine as 'domaine?',
                    specialty as 'specialty?'
                from 
                    Edl.User
                where
                    id = ? and
                    domaine = ? and
                    role = "Applicant"
                "#,
                app_id,
                vd.domaine
            ).fetch_one(pool)
            .await
        }

        pub async fn affect_applicant(
            vd: user::User,
            af: ApplicantAffectation,
            pool: &MySqlPool,
        ) -> sqlx::Result<()> {
            if sqlx::query!(
                r#"
                insert into applicant_affectation
                    (applicant_id,session_id,presence)
                select distinct
                    u.id,s.id,false
                from Edl.User u, Edl.Session s, Edl.User cfd
                where
                    u.id = ? and
                    u.role = "Applicant" and
                    u.specialty = cfd.specialty and
                    cfd.id = s.cfd_id and
                    s.id = ? and
                    s.virtual_platform_id = ?
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

        pub async fn encode_applicant(
            vd: user::User,
            applicant_affectation: ApplicantAffectation,
            pool: &MySqlPool
        ) -> sqlx::Result<()> {
            if sqlx::query!(
                r#"
                update Edl.applicant_affectation
                set
                    encoding = ?,
                    presence = true
                where
                    applicant_id = ? and
                    session_id = ? and
                    session_id in (
                        select
                            id
                        from Edl.Session
                        where virtual_platform_id = ?
                    );
                "#,
                applicant_affectation.encoding,
                applicant_affectation.applicant_id,
                applicant_affectation.session_id,
                vd.id
            ).execute(pool)
            .await?
            .rows_affected() != 1 {
                return Err(sqlx::Error::RowNotFound)
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
        
        pub async fn get_announcement(
            vd: user::User,
            session_id: i32,
            pool: &MySqlPool
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
                    Edl.Announcement a, Edl.Session s
                where
                    a.session_id = s.id and
                    s.id = ? and
                    s.virtual_platform_id = ?
                "#,
                session_id,
                vd.id
            ).fetch_all(pool)
            .await
        }

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

// getting the cfd
#[get("/getcfd")]
pub async fn get_cfd(
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(|_| true, |u| db::get_cfd(u, &data.pool), &[user::Role::ViceDoyen], request) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(ss) = f.await else {
        return Either::Left(HttpResponse::NotFound().finish());
    };
    Either::Right(web::Json(ss))
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

#[get("/module/session={id}")]
pub async fn get_modules(
    s_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
    let s_id = s_id.0;
    let Some(f) = secure_function(
        |_| true,
        |u| db::dmodule::get_modules(u, s_id, &data.pool),
        &[user::Role::ViceDoyen], 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(ms) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    Either::Right(web::Json(ms))
}

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
    pub encoding: String
}

#[get("/applicant")]
pub async fn get_possible_applicants(
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::applicant::get_possible_applicants(u, &data.pool), 
        &[user::Role::ViceDoyen], 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(apps) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    Either::Right(web::Json(apps))
}
#[get("/applicantaffected/{id}")]
pub async fn get_applicant_affectation(
    s_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
    let Some(f) = secure_function(
        |_| true, 
        |u| db::applicant::get_applicant_affectation(u, s_id.0,&data.pool), 
        &[user::Role::ViceDoyen], 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(apps) = f.await else {
        return Either::Left(HttpResponse::NotFound().finish());
    };
    Either::Right(web::Json(apps))
}

#[get("/applicant/session={id}")]
pub async fn get_current_applicants(
    s_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
    let s_id = s_id.0;
    let Some(f) = secure_function(
        |_| true, 
        |u| db::applicant::get_current_applicants(u, s_id, &data.pool), 
        &[user::Role::ViceDoyen], 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(apps) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    Either::Right(web::Json(apps))
}

#[get("/applicant/{id}")]
pub async fn get_applicant(
    u_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
    let u_id = u_id.0;
    let Some(f) = secure_function(
        |_| true, 
        |u| db::applicant::get_applicant(u, u_id, &data.pool), 
        &[user::Role::ViceDoyen], 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(apps) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    Either::Right(web::Json(apps))
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

#[put("/applicant/encode")]
pub async fn encode_applicant(
    af: web::Json<ApplicantAffectation>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> HttpResponse {
    let Some(f) = secure_function(
        |_| true,
        |u| db::applicant::encode_applicant(u, af.0, &data.pool),
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

#[get("/announcement/session={id}")]
pub async fn get_announcement(
    s_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
    let s_id = s_id.0;
    let Some(f) = secure_function(
        |_| true, 
        |u| db::announcement::get_announcement(u, s_id, &data.pool), 
        &[user::Role::ViceDoyen], 
        request
    ) else {
        return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(ans) = f.await else {
        return Either::Left(HttpResponse::Forbidden().finish()); 
    };
    Either::Right(web::Json(ans))
}

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

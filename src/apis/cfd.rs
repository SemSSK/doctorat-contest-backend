use actix_web::{delete, get, post, web, Either, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{ServerState, model::user, apis::authentication::secure_function};

const API_RULES: [user::Role;1] = [user::Role::CFD];


#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorAffectation {
    session_id: i32,
    professor_id: i32,
}

mod db {
    use crate::model::{session, user};


    pub async fn get_sessions(
      cfd: user::User,
      pool: &sqlx::MySqlPool
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
          cfd_id = ?
        "#,
        cfd.id
      ).fetch_all(pool)
      .await
    }

    pub async fn get_session(
      cfd: user::User,
      session_id: i32,
      pool: &sqlx::MySqlPool
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
          cfd_id = ? and
          id = ?
        "#,
        cfd.id,
        session_id
      ).fetch_one(pool)
      .await
    }
  
    pub mod monitors {
        use crate::{apis::cfd::MonitorAffectation, model::user};

        pub async fn get_possible_monitors(
            cfd: user::User,
            session_id: i32,
            pool: &sqlx::MySqlPool,
        ) -> sqlx::Result<Vec<user::User>> {
            sqlx::query_as!(
                user::User,
                r#"
            select
              id as 'id?', 
              email,
              "" as 'password?', 
              role as 'role?: user::Role',
              domaine as 'domaine?',
              specialty as 'specialty?'
            from
              Edl.User u
            where
              u.role = "Professor" and
              u.specialty = ? and
              u.id not in 
                (
                  select professor_id
                  from Edl.monitor_affectation
                  where session_id = ?
                )
            "#,
                cfd.specialty,
                session_id
            )
            .fetch_all(pool)
            .await
        }

        pub async fn get_affected_monitors(
          cfd: user::User,
          session_id: i32,
          pool: &sqlx::MySqlPool,
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
            Edl.User u, Edl.monitor_affectation ma, Edl.Session s
          where
            u.id = ma.professor_id and
            ma.session_id = ? and
            s.id = ma.session_id and
            s.cfd_id = ?
          "#,
            session_id,
            cfd.id
          )
          .fetch_all(pool)
          .await
        }


        pub async fn add_monitor(
            cfd: user::User,
            ma: MonitorAffectation,
            pool: &sqlx::MySqlPool,
        ) -> sqlx::Result<()> {
            sqlx::query!(
              r#"
              insert into Edl.monitor_affectation
              select distinct
                s.id,p.id
              from
                Edl.User p, Edl.Session s
              where
                p.id = ? and
                s.id = ? and
                s.cfd_id = ? and
                p.specialty = ?
              "#,
              ma.professor_id,
              ma.session_id,
              cfd.id,
              cfd.specialty
            ).execute(pool)
            .await?;
          Ok(())
        }

        pub async fn delete_monitor(
            cfd: user::User,
            ma: MonitorAffectation,
            pool: &sqlx::MySqlPool,
        ) -> sqlx::Result<()> {
            sqlx::query!(
              r#"
              delete 
              from 
                Edl.monitor_affectation
              where
                session_id in 
                  (
                    select 
                      s.id
                    from
                      Edl.Session s
                    where
                      s.id = ? and
                      s.cfd_id = ?
                  ) and
                professor_id in 
                  (
                    select
                      p.id
                    from
                      Edl.User p
                    where
                      p.id = ? and
                      p.specialty = ?
                  )
              "#,
              ma.session_id,
              cfd.id,
              ma.professor_id,
              cfd.specialty
            ).execute(pool)
            .await?;
            
            Ok(())
        }
    }

    pub mod result {}
}


#[get("/")]
pub async fn get_sessions(
  data: web::Data<ServerState>,
  request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::get_sessions(u, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  let Ok(ms) = f.await else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  Either::Right(web::Json(ms))
}

#[get("/session={id}")]
pub async fn get_session(
  session_id: web::Path<(i32,)>,
  data: web::Data<ServerState>,
  request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::get_session(u, session_id.0, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  let Ok(ms) = f.await else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  Either::Right(web::Json(ms))
}



#[get("/monitor/session={id}")]
pub async fn get_possible_monitors(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
  
  let Some(f) = secure_function(
    |_| true, 
    |u| db::monitors::get_possible_monitors(u, session_id.0, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  let Ok(ms) = f.await else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  Either::Right(web::Json(ms))

}


#[get("/monitor/session={id},affected")]
pub async fn get_affected_monitors(
    session_id: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
  
  let Some(f) = secure_function(
    |_| true, 
    |u| db::monitors::get_affected_monitors(u, session_id.0, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  let Ok(ms) = f.await else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  Either::Right(web::Json(ms))

}


#[post("/monitor")]
pub async fn add_monitor(
  ma: web::Json<MonitorAffectation>,
  data: web::Data<ServerState>,
  request: HttpRequest
) -> HttpResponse {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::monitors::add_monitor(u, ma.0, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return HttpResponse::Forbidden().finish();
  };

  let Ok(_) = f.await else {
    return HttpResponse::NotFound().finish();
  };

  HttpResponse::Ok().finish()
}


#[delete("/monitor")]
pub async fn delete_monitor(
  ma: web::Json<MonitorAffectation>,
  data: web::Data<ServerState>,
  request: HttpRequest
) -> HttpResponse {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::monitors::delete_monitor(u, ma.0, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return HttpResponse::Forbidden().finish();
  };

  let Ok(_) = f.await else {
    return HttpResponse::NotFound().finish();
  };

  HttpResponse::Ok().finish()
}



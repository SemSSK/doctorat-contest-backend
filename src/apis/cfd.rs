use actix_web::{delete, get, post, put, web, Either, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{ServerState, model::{user, result}, apis::authentication::secure_function};

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
            if sqlx::query!(
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
            .await?
            .rows_affected() != 1 {
              return Err(sqlx::Error::RowNotFound);
            };
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

    pub mod result {
        use crate::model::{user, module, result};

      pub async fn get_possible_correctors(
        cfd: user::User,
        _session_id: i32,
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
            Edl.User u
          where
            u.role = "Professor" and
            u.specialty = ? 
          "#,
          cfd.specialty
        ).fetch_all(pool)
        .await
      }

      pub async fn get_modules(
        cfd: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
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
            s.id = ? and
            m.session_id  = s.id and
            s.cfd_id = ?
          "#,
          session_id,
          cfd.id
        ).fetch_all(pool)
        .await
      }

      pub async fn get_applicants(
        cfd: user::User,
        session_id: i32,
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
            Edl.User u, Edl.Session s, Edl.applicant_affectation af
          where
            u.id = af.applicant_id and
            af.presence = true and
            s.id = ? and
            s.cfd_id = ?
          "#,
          session_id,
          cfd.id
        ).fetch_all(pool)
        .await
      }

      pub async fn create_result(
        cfd: user::User,
        res: result::Result,
        pool: &sqlx::MySqlPool
      ) -> sqlx::Result<()> {
        if sqlx::query!(
          r#"
          insert into Edl.Result
            (
              applicant_id,
              module_id,
              session_id,
              corrector_1_id,
              corrector_2_id,
              corrector_3_id
            )
          select distinct
            af.applicant_id,
            m.code,
            s.id,
            c1.id,
            c2.id,
            c3.id
          from
            Edl.User c1,
            Edl.User c2,
            Edl.User c3,
            Edl.applicant_affectation af, 
            Edl.Module m, 
            Edl.Session s
          where
            af.session_id = s.id and
            m.session_id = s.id and
            af.applicant_id = ? and
            af.presence = true and
            m.code = ? and
            s.cfd_id = ? and
            s.id = ? and
            c1.id = ? and
            c2.id = ? and
            c3.id = ? and
            c1.id not in (c2.id, c3.id) and
            c2.id != c3.id and
            c1.role = "Professor" and
            c2.role = "Professor" and
            c3.role = "Professor"
          "#,
          res.applicant_id,
          res.module_id,
          cfd.id,
          res.session_id,
          res.corrector_1_id,
          res.corrector_2_id,
          res.corrector_3_id,
        ).execute(pool)
        .await?
        .rows_affected() != 1 {
          return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
      }

      pub async fn get_results(
        cfd: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
      ) -> sqlx::Result<Vec<result::Result>> {
        sqlx::query_as!(
          result::Result,
          r#"
          select 
            r.applicant_id,
            r.module_id,
            r.session_id,
            r.corrector_1_id as 'corrector_1_id!',
            r.corrector_2_id as 'corrector_2_id!',
            r.corrector_3_id as 'corrector_3_id!',
            r.note_1 as 'note_1?',
            r.note_2 as 'note_2?',
            r.note_3 as 'note_3?',
            r.display_to_applicant as 'display_to_applicant?: bool',
            r.display_to_cfd as 'display_to_cfd?: bool'
          from
            Edl.Result r, Edl.Session s
          where
            s.id = ? and
            s.cfd_id = ? and
            r.display_to_cfd = true and
            r.session_id = s.id
          "#,
          session_id,
          cfd.id
        ).fetch_all(pool)
        .await
      }

      pub async fn check_if_correction_ended(
        cfd: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
      ) -> sqlx::Result<bool> {
        Ok(sqlx::query!(
                  r#"
                  select 
                    (case
                      when abs(r.note_1 - r.note_2) <= 3 or
                      r.note_3 is not null then
                      true
                    else
                      false
                    end) as 'corrected!: bool'
                  from
                    Edl.Result r, Edl.Session s
                  where
                    s.id = ? and
                    s.cfd_id = ? and
                    r.session_id = s.id
                  "#,
                  session_id,
                  cfd.id
                ).fetch_all(pool)
                .await?
                .iter()
                .all(|r| {
                  r.corrected
                }))
      }

      pub async fn end_session(
        cfd: user::User,
        session_id: i32,
        pool: &sqlx::MySqlPool
      ) -> sqlx::Result<()> {
        if sqlx::query!(
          r#"
          update Edl.Result
          set display_to_applicant = true
          where 
            display_to_cfd = TRUE and
            session_id = ? and
            session_id in 
              (
                select id
                from Edl.Session
                where cfd_id = ?
              )
          "#,
          session_id,
          cfd.id
        ).execute(pool)
        .await?
        .rows_affected() != 1 {
          return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
      }
      
    }
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


#[get("/result/correctors/session={id}")]
pub async fn get_possible_correctors(
  session_id: web::Path<(i32,)>,
  data: web::Data<ServerState>,
  request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::result::get_possible_correctors(u, session_id.0, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  let Ok(cs) = f.await else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  Either::Right(web::Json(cs))
}



#[get("/result/module/session={id}")]
pub async fn get_modules(
  session_id: web::Path<(i32,)>,
  data: web::Data<ServerState>,
  request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::result::get_modules(u, session_id.0, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  let Ok(cs) = f.await else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  Either::Right(web::Json(cs))
}

#[get("/result/applicants/session={id}")]
pub async fn get_applicants(
  session_id: web::Path<(i32,)>,
  data: web::Data<ServerState>,
  request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::result::get_applicants(u, session_id.0, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  let Ok(us) = f.await else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  Either::Right(web::Json(us))
}


#[get("/result/session={id}")]
pub async fn get_results(
  session_id: web::Path<(i32,)>,
  data: web::Data<ServerState>,
  request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::result::get_results(u, session_id.0, &data.pool), 
    &API_RULES, 
    request
  ) else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  let Ok(rs) = f.await else {
    return Either::Left(HttpResponse::Forbidden().finish());
  };

  Either::Right(web::Json(rs))
}

#[post("/result")]
pub async fn create_result(
  res: web::Json<result::Result>,
  data: web::Data<ServerState>,
  request: HttpRequest
) -> HttpResponse {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::result::create_result(u, res.0, &data.pool), 
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

#[get("/result/ended_session={id}")]
pub async fn check_if_correction_ended( 
  session_id: web::Path<(i32,)>,
  data: web::Data<ServerState>,
  request: HttpRequest
) -> Either<HttpResponse,impl Responder> {
    let Some(f) = secure_function(
      |_| true, 
      |u| db::result::check_if_correction_ended(u, session_id.0, &data.pool), 
      &API_RULES, 
      request
    ) else {
      return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(b) = f.await else {
      return Either::Left(HttpResponse::Forbidden().finish());
    };

  Either::Right(web::Json(b))
}


#[put("/result/session={id}")]
pub async fn end_session(
  session_id: web::Path<(i32,)>,
  data: web::Data<ServerState>,
  request: HttpRequest
) -> HttpResponse {
  let Some(f) = secure_function(
    |_| true, 
    |u| db::result::end_session(u, session_id.0, &data.pool), 
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
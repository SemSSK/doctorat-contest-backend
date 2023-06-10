use crate::model::user;
use crate::ServerState;
use crate::{apis::authentication::secure_function, model::virtual_platform::VirtualPlatform};
use actix_web::{delete, get, post, put, web, Either, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

mod db {
    use crate::model::{user, virtual_platform::VirtualPlatform};
    use rand::{distributions::Alphanumeric, Rng};

    /// Gets one user from database by id
    pub async fn get_user(id: i32, pool: &sqlx::MySqlPool) -> sqlx::Result<user::User> {
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
          from Edl.User
          where id = ?
        "#,
            id
        )
        .fetch_one(pool)
        .await
    }

    /// Get all users from database
    pub async fn get_users(pool: &sqlx::MySqlPool) -> sqlx::Result<Vec<user::User>> {
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
        from Edl.User
      "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn _get_user_by_email(
        email: &str,
        pool: &sqlx::MySqlPool,
    ) -> sqlx::Result<user::User> {
        sqlx::query_as!(
            user::User,
            r#"
            select 
              id as 'id?',
              name,
              email, 
              "" as 'password?', 
              role as 'role?: user::Role',
              domaine as 'domaine?',
              specialty as 'specialty?'
              from Edl.User
            where email = ?
          "#,
            email
        )
        .fetch_one(pool)
        .await
    }

    fn generate_password() -> String {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        // let s = bcrypt::hash(s, DEFAULT_COST).unwrap();
        s
    }

    /// Creates a new user and adds it to database
    /// Returns error in case of duplicate.
    pub async fn insert_user(
        email: String,
        name: String,
        role: user::Role,
        domaine: String,
        specialty: String,
        pool: &sqlx::MySqlPool,
    ) -> sqlx::Result<()> {
        let password = generate_password();
        sqlx::query!(
            r#"
          insert into Edl.User (email,name,password,role,domaine,specialty) values
          (?,?,?,?,?,?)
        "#,
            email,
            name,
            password,
            role,
            domaine,
            specialty
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// updates one user given a user object
    pub async fn update_user(u: user::User, pool: &sqlx::MySqlPool) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
    update Edl.User
    set email = ?,name= ?, role = ?, domaine = ?,specialty = ?
    where id = ?
  "#,
            u.email,
            u.name,
            u.role,
            u.domaine,
            u.specialty,
            u.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    /// deletes a user given an id returns error in case of user not found
    pub async fn delete_user(id: i32, pool: &sqlx::MySqlPool) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
    delete from Edl.User
    where id = ?
  "#,
            id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn create_users(
      users: Vec<user::User>,
      pool: &sqlx::MySqlPool
    ) -> sqlx::Result<()> {

      let users = users.into_iter().map(|u| user::User {
        password: Some(generate_password()),
        ..u
      });

      sqlx::QueryBuilder::new("insert into Edl.User(email,password,role,domaine,specialty)")
          .push_values(users, |mut b,u| {
            b.push_bind(u.email)
              .push_bind(u.password)
              .push_bind(u.role)
              .push_bind(u.domaine)
              .push_bind(u.specialty);
          })
          .build()
          .execute(pool)
          .await?;

      Ok(())
    }

    /// creates a virtual platform
    ///
    /// Fails in case of
    pub async fn create_virtual_platform(
        vp: VirtualPlatform,
        pool: &sqlx::MySqlPool,
    ) -> sqlx::Result<()> {
        if sqlx::query!(
            r#"
          Insert into Edl.VirtualPlatform
          (vd_id, name)
          Select id, ?
          from Edl.User
          where id = ? and role = ?
        "#,
            vp.name,
            vp.vd_id,
            user::Role::ViceDoyen
        )
        .execute(pool)
        .await?
        .rows_affected() == 0 {
          return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }
 
    pub async fn get_virtual_platforms(pool:&sqlx::MySqlPool) -> sqlx::Result<Vec<VirtualPlatform>> {
      sqlx::query_as!(
        VirtualPlatform,
        r#"
          select
            vd_id,
            name
          from
            Edl.VirtualPlatform
        "#
      ).fetch_all(pool)
      .await
    }

    pub async fn delete_virtual_platform(pool:&sqlx::MySqlPool,id:i32) -> sqlx::Result<()> {
      if sqlx::query!(
        r#"
          delete from Edl.VirtualPlatform
          where vd_id = ?
        "#,
        id
      ).execute(pool)
      .await?
      .rows_affected() == 1 {
        return Err(sqlx::Error::RowNotFound);
      }
      Ok(())
    }
}

#[get("/{id}")]
pub async fn get_user(
    path: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let id = path.0;
    let Some(f) = secure_function(
        |_| true,
        |_| db::get_user(id, &data.pool),
        &[user::Role::Admin],
        request,
    ) else {
      return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(u) = f.await else {
      return Either::Left(HttpResponse::NotFound().finish());
    };
    Either::Right(web::Json(u))
}

#[get("/")]
async fn get_users(
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(|_| true, |_| db::get_users(&data.pool),&[user::Role::Admin], request) else {
      return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(users) = f.await else {
      return Either::Left(HttpResponse::NotFound().finish());
    };

    Either::Right(web::Json(users))
}

#[derive(Serialize, Deserialize)]
struct CreateUserInput {
    email: String,
    name:String,
    role: user::Role,
    domaine: String,
    specialty: String,
}

#[post("/")]
async fn create_user(
    u: web::Json<CreateUserInput>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let CreateUserInput {
        email,
        name,
        role,
        domaine,
        specialty,
    } = u.0;

    let Some(f) = secure_function(
        |_| true,
        |_| db::insert_user(email, name,role, domaine, specialty, &data.pool),
        &[user::Role::Admin],
        request,
    ) else {
      return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(_) = f.await else {
      return Either::Left(HttpResponse::NotAcceptable().finish());
    };

    Either::Right(HttpResponse::Ok().finish())
}

#[post("/multiple")]
async fn create_users(
  users: web::Json<Vec<user::User>>,
  data: web::Data<ServerState>,
  request: HttpRequest,
) -> HttpResponse {
  let users = users.0;
  let Some(f) = secure_function(
    |_| true, 
    |_| db::create_users(users, &data.pool), 
    &[user::Role::Admin], 
    request) else {
      return HttpResponse::Forbidden().finish();
  };

  let Ok(_) = f.await else {
    return HttpResponse::BadRequest().finish();
  };

  HttpResponse::Ok().finish()
}

#[put("/")]
async fn update_user(
    u: web::Json<user::User>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(
        |_| true,
        |_| db::update_user(u.0, &data.pool),
        &[user::Role::Admin],
        request,
    ) else {
      return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(_) = f.await else {
      return Either::Left(HttpResponse::NotAcceptable().finish());
    };

    Either::Right(HttpResponse::Ok().finish())
}

#[delete("/{id}")]
pub async fn delete_user(
    path: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let id = path.0;
    let Some(f) = secure_function(
        |_| true,
        |_| db::delete_user(id, &data.pool),
        &[user::Role::Admin],
        request,
    ) else {
      return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(_) = f.await else {
      return Either::Left(HttpResponse::NotFound().finish());
    };
    Either::Right(HttpResponse::Ok().finish())
}

#[get("/virtual-platform")]
async fn get_virtual_platforms(
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let Some(f) = secure_function(|_| true, |_| db::get_virtual_platforms(&data.pool),&[user::Role::Admin], request) else {
      return Either::Left(HttpResponse::Forbidden().finish());
    };

    let Ok(users) = f.await else {
      return Either::Left(HttpResponse::NotFound().finish());
    };

    Either::Right(web::Json(users))
}

#[post("/virtual-platform")]
pub async fn create_virtual_platform(
    vp: web::Json<VirtualPlatform>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> impl Responder {
    let vp = vp.0;
    
    let Some(f) = secure_function(
      |_| true, 
      |_| db::create_virtual_platform(vp, &data.pool),
      &[user::Role::Admin], 
      request) else {
      return HttpResponse::Forbidden().finish();
    };

    let Ok(_) = f.await else {
      return HttpResponse::NotFound().finish();
    };

    HttpResponse::Ok().finish()
}

#[delete("/virtual-platform/{id}")]
pub async fn delete_virtual_platform(
    path: web::Path<(i32,)>,
    data: web::Data<ServerState>,
    request: HttpRequest,
) -> Either<HttpResponse, impl Responder> {
    let id = path.0;
    let Some(f) = secure_function(
        |_| true,
        |_| db::delete_virtual_platform(&data.pool,id),
        &[user::Role::Admin],
        request,
    ) else {
      return Either::Left(HttpResponse::Forbidden().finish());
    };
    let Ok(_) = f.await else {
      return Either::Left(HttpResponse::NotFound().finish());
    };
    Either::Right(HttpResponse::Ok().finish())
}



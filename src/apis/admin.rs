use crate::apis::authentication::secure_function;
use crate::model::user;
use crate::ServerState;
use actix_web::{delete, get, post, put, web, Either, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

mod db {
    use crate::model::user;
    use rand::{distributions::Alphanumeric, Rng};

    /// Gets one user from database by id
    pub async fn get_user(id: i32, pool: &sqlx::MySqlPool) -> sqlx::Result<user::User> {
        sqlx::query_as!(
            user::User,
            r#"
          select 
            id as 'id?', 
            email, password as 'password?', 
            role as 'role?: user::Role',
            encoded as 'encoded?',
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
          email, password as 'password?', 
          role as 'role?: user::Role',
          encoded as 'encoded?',
          specialty as 'specialty?'
        from Edl.User
      "#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_user_by_email(
        email: &str,
        pool: &sqlx::MySqlPool,
    ) -> sqlx::Result<user::User> {
        sqlx::query_as!(
            user::User,
            r#"
            select 
              id as 'id?', 
              email, password as 'password?', 
              role as 'role?: user::Role',
              encoded as 'encoded?',
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

    pub async fn insert_user(
        email: String,
        role: user::Role,
        specialty: String,
        pool: &sqlx::MySqlPool,
    ) -> sqlx::Result<()> {
        let password = generate_password();
        sqlx::query!(
            r#"
          insert into Edl.User (email,password,role,specialty) values
          (?,?,?,?)
        "#,
            email,
            password,
            role,
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
    set email = ?, role = ?, specialty = ?
    where id = ?
  "#,
            u.email,
            u.role,
            u.specialty,
            u.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

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
    role: user::Role,
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
        role,
        specialty,
    } = u.0;

    let Some(f) = secure_function(
        |_| true,
        |_| db::insert_user(email, role, specialty, &data.pool),
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

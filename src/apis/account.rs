use actix_web::{put, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::apis::authentication::secure_function;
use crate::{model::user, ServerState};

mod db {
    use crate::model::user;
    pub async fn change_email(
        user: user::User,
        email: String,
        pool: &sqlx::MySqlPool,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
            update Edl.User
            set email = ?
            where id = ? 
          "#,
            email,
            user.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn change_password(
        user: user::User,
        password: String,
        pool: &sqlx::MySqlPool,
    ) -> sqlx::Result<()> {
        sqlx::query!(
            r#"
          update Edl.User
          set password = ?
          where id = ? 
        "#,
            password,
            user.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[put("/email")]
pub async fn change_email(
    request: HttpRequest,
    email: web::Json<String>,
    data: web::Data<ServerState>,
) -> impl Responder {
    let email = email.0;
    let Some(f) = secure_function(|_| true, |u| db::change_email(u, email, &data.pool), &user::ALL_ROLES  , request) else {
      return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
      return HttpResponse::NotFound().finish();
    };

    HttpResponse::Ok().finish()
}

#[put("/password")]
pub async fn change_password(
    request: HttpRequest,
    password: web::Json<String>,
    data: web::Data<ServerState>,
) -> impl Responder {
    let password = password.0;
    let Some(f) = secure_function(|_| true, |u| db::change_password(u, password, &data.pool), &user::ALL_ROLES  , request) else {
      return HttpResponse::Forbidden().finish();
    };
    let Ok(_) = f.await else {
      return HttpResponse::NotFound().finish();
    };

    HttpResponse::Ok().finish()
}

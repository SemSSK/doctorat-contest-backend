use crate::apis::authentication::secure_function;
use crate::{user, ServerState};
use actix_web::{get, web, Either, HttpRequest, HttpResponse, Responder};

//Database interaction
async fn get_user_from_db(id: i32, pool: &sqlx::MySqlPool) -> sqlx::Result<user::User> {
    sqlx::query_as!(
        user::User,
        r#"
          select id as 'id?', email, password as 'password?', role as 'role?: user::Role'
          from Edl.User
          where id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await
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
        |_| get_user_from_db(id, &data.pool),
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

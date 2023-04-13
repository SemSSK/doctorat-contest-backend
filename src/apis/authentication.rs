use crate::model::user;
use crate::{
    jwt_handler::{self, encode_to_jwt},
    ServerState,
};
use actix_web::{get, post, web, Either, HttpRequest, HttpResponse, Responder};

/// Checks if the user input is the same as the user in the database
async fn verify_user(user: user::User, pool: &sqlx::MySqlPool) -> Option<user::User> {
    match sqlx::query_as!(
        user::User,
        "select 
            id as `id?`,
            email, password as `password?`, 
            role as `role?: user::Role`,
            domaine as 'domaine?',
            specialty as 'specialty'
        from Edl.User where email = ?",
        user.email
    )
    .fetch_one(pool)
    .await
    {
        Ok(real_user) if real_user.password == user.password => Some(real_user),
        _ => None,
    }
}

#[post("/login")]
pub async fn login(
    user: web::Json<user::User>,
    data: web::Data<ServerState>,
) -> Either<HttpResponse, impl Responder> {
    match verify_user(user.0, &data.pool)
        .await
        .map(jwt_handler::encode_to_jwt)
    {
        Some(jwt) => Either::Right(jwt),
        None => Either::Left(HttpResponse::Forbidden().body("Wrong credentials")),
    }
}

#[get("/refresh")]
pub async fn refresh(request: HttpRequest) -> Either<HttpResponse, impl Responder> {
    match secure_function(|_| true, encode_to_jwt, &user::ALL_ROLES, request) {
        Some(jwt) => Either::Right(jwt),
        None => Either::Left(HttpResponse::Forbidden().finish()),
    }
}

/// Returns the user data from the claim
/// When the claim is valide
fn get_claim_from_header(request: HttpRequest) -> Option<user::User> {
    let Ok(jwt) = request
      .headers()
      .get("Auth")?
      .to_str()
      else {
        return None;
      };

    jwt_handler::validate_jwt(jwt)
}

/// Should be used when securing an api with role access based control returns ```None``` if the tests are invalide returns ```Some(T)``` otherwise ```T``` here depends on the caller
///
/// ```checks``` : is a closure that should implement the boolean tests made to verify if the information like owenership of data
///
/// ```logic``` : is a closure that implements the business logic it is called only when ```checks``` evaluates to ```true```
///
/// ```roles``` : is an array slice that contains the roles that are allowed to access the api
pub fn secure_function<F, G, T>(
    checks: F,
    logic: G,
    roles: &[user::Role],
    request: HttpRequest,
) -> Option<T>
where
    F: FnOnce(&user::User) -> bool,
    G: FnOnce(user::User) -> T,
{
    get_claim_from_header(request)
        .filter(|u| match &u.role {
            Some(r) => roles.contains(r) && checks(u),
            None => false,
        })
        .map(logic)
}

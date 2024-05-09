use ntex::{web, http};
use ntex_identity::Identity;

use crate::store::DBState;
use crate::error::HttpError;
use crate::models::token::Token;
use crate::models::user::{User, UserPartial, UserCredential};

#[utoipa::path(
  post,
  tag = "Users",
  path = "/users",
  request_body = UserPartial,
  responses(
    (status = 201, description = "The new user", body = User),
  ),
)]
#[web::post("/users")]
async fn create_user(
  db: DBState,
  new_user: web::types::Json<UserPartial>,
) -> Result<web::HttpResponse, HttpError> {
  let user = User::create(&new_user, &db).await?;
  Ok(web::HttpResponse::Created().json(&user))
}

#[utoipa::path(
  post,
  tag = "Users",
  path = "/users/login",
  request_body = UserCredential,
  responses(
    (status = 200, description = "The user", body = Token),
  ),
)]
#[web::post("/users/login")]
async fn login_user(
  db: DBState,
  id: Identity,
  credential: web::types::Json<UserCredential>,
) -> Result<web::HttpResponse, HttpError> {
  let token = User::login(&credential, &db).await?;
  id.remember(token.value.clone());
  Ok(web::HttpResponse::Ok().json(&token))
}

#[utoipa::path(
  get,
  tag = "Users",
  path = "/users",
  responses(
    (status = 200, description = "The list of users", body = Vec<User>),
  ),
)]
#[web::get("/users")]
async fn list_user(db: DBState) -> Result<web::HttpResponse, HttpError> {
  let users = User::list(&db).await?;
  Ok(web::HttpResponse::Ok().json(&users))
}

#[utoipa::path(
  get,
  tag = "Users",
  path = "/users/me",
  responses(
    (status = 200, description = "The user", body = User),
  ),
)]
#[web::get("/users/me")]
async fn get_me(
  db: DBState,
  id: Identity,
) -> Result<web::HttpResponse, HttpError> {
  let token = id.identity().ok_or(HttpError {
    status: http::StatusCode::UNAUTHORIZED,
    msg: "Unauthorized".into(),
  })?;
  let token =
    Token::find_by_value(&token, &db)
      .await
      .map_err(|_| HttpError {
        status: http::StatusCode::UNAUTHORIZED,
        msg: "Unauthorized".into(),
      })?;
  let user = User::find_by_id(&token.user_id, &db).await?;
  Ok(web::HttpResponse::Ok().json(&user))
}

#[utoipa::path(
  get,
  tag = "Users",
  path = "/users/{id}",
  responses(
    (status = 200, description = "The user", body = User),
  ),
)]
#[web::get("/users/{id}")]
async fn inspect_user() -> Result<web::HttpResponse, HttpError> {
  Ok(web::HttpResponse::Ok().finish())
}

mod user;
mod role;
pub mod openapi;
pub mod swagger;

use ntex::web;
use ntex::http;
use crate::error::HttpError;

pub async fn unhandled() -> Result<web::HttpResponse, HttpError> {
  Err(HttpError {
    status: http::StatusCode::NOT_FOUND,
    msg: "Not Found".to_string(),
  })
}

pub fn configure(config: &mut web::ServiceConfig) {
  config
    .service(user::create_user)
    .service(user::login_user)
    .service(user::get_me)
    .service(user::list_user)
    .service(user::inspect_user);
}

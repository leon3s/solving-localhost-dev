use utoipa::OpenApi;

use super::user;

use crate::models::user::{User, UserPartial, UserCredential};
use crate::models::token::Token;

#[derive(OpenApi)]
#[openapi(paths(
  user::create_user,
  user::login_user,
  user::list_user,
  user::get_me,
),   components(schemas(
  User,
  UserPartial,
  UserCredential,
  Token,
)), tags(
  (name = "Users", description = "Users management endpoints."),
))]
pub(crate) struct ApiDoc;

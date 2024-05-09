use ntex::web;

#[utoipa::path(
  get,
  tag = "Roles",
  path = "/users",
  responses(
    (status = 200, description = "The list existing roles", body = Vec<User>),
  ),
)]
#[web::get("/roles")]
pub async fn list_role() -> Result<web::HttpResponse, crate::error::HttpError> {
  Ok(web::HttpResponse::Ok().finish())
}

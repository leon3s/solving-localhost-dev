#[macro_use]
extern crate diesel;

use ntex::web;
use ntex_cors::Cors;
use ntex_identity::{IdentityService, CookieIdentityPolicy};
use utoipa::OpenApi;

mod error;
mod store;
mod schema;
mod models;
mod services;
mod repositories;

use services::swagger;
use services::openapi::ApiDoc;

#[ntex::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
{
  if std::env::var("LOG_LEVEL").is_err() {
    std::env::set_var("LOG_LEVEL", "info");
  }
  env_logger::Builder::new()
    .parse_env("LOG_LEVEL")
    .format_target(false)
    .init();

  let db_url = std::env::var("DATABASE_URL")?;
  let pool = store::init(&db_url).await?;

  let api_doc = ApiDoc::openapi();
  let swagger_conf =
    swagger::SwaggerConfig::new(api_doc, "/explorer/swagger.json");

  web::HttpServer::new(move || {
    web::App::new()
      .wrap(IdentityService::new(
        CookieIdentityPolicy::new(&[0; 32])
          .name("nhtiam")
          .domain("next-hat.internal")
          .path("/")
          .secure(false),
      ))
      .wrap(
        Cors::new()
          .max_age(3600)
          .supports_credentials()
          .finish(),
      )
      .service(
        web::scope("/explorer/")
          .state(swagger_conf.clone())
          .configure(swagger::register),
      )
      .state(pool.clone())
      .configure(services::configure)
      .default_service(web::route().to(services::unhandled))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await?;

  Ok(())
}

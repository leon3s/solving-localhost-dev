use std::sync::Arc;

use ntex::web;
use ntex::http;
use ntex::util::Bytes;

use crate::error::HttpError;

#[derive(Clone)]
pub struct SwaggerConfig<'a> {
  definition: utoipa::openapi::OpenApi,
  config: Arc<utoipa_swagger_ui::Config<'a>>,
}

impl<'a> SwaggerConfig<'a> {
  pub fn new(definition: utoipa::openapi::OpenApi, def_url: &'a str) -> Self {
    Self {
      definition,
      config: Arc::new(
        utoipa_swagger_ui::Config::new([def_url]).use_base_layout(),
      ),
    }
  }
}

#[web::get("/swagger.json")]
async fn get_specs(
  openapi_conf: web::types::State<SwaggerConfig<'static>>,
) -> Result<web::HttpResponse, HttpError> {
  let spec = openapi_conf.definition.to_json().map_err(|err| HttpError {
    status: http::StatusCode::INTERNAL_SERVER_ERROR,
    msg: format!("Error generating OpenAPI spec: {}", err),
  })?;
  return Ok(
    web::HttpResponse::Ok()
      .content_type("application/json")
      .body(spec),
  );
}

#[web::get("/{tail}*")]
async fn get_swagger(
  tail: web::types::Path<String>,
  openapi_conf: web::types::State<SwaggerConfig<'static>>,
) -> Result<web::HttpResponse, HttpError> {
  match utoipa_swagger_ui::serve(&tail, openapi_conf.config.clone()).map_err(
    |err| HttpError {
      status: http::StatusCode::INTERNAL_SERVER_ERROR,
      msg: format!("Error generating Swagger UI: {}", err),
    },
  )? {
    None => Err(HttpError {
      status: http::StatusCode::NOT_FOUND,
      msg: "Not Found".to_string(),
    }),
    Some(file) => Ok({
      let bytes = Bytes::from(file.bytes.to_vec());
      web::HttpResponse::Ok()
        .content_type(file.content_type)
        .body(bytes)
    }),
  }
}

pub fn register(config: &mut web::ServiceConfig) {
  config.service(get_specs);
  config.service(get_swagger);
}

use rand::prelude::*;
use base64::{engine, Engine, alphabet};
use serde::Serialize;

use crate::schema::tokens;

#[derive(
  Clone, Debug, Serialize, Identifiable, Queryable, utoipa::ToSchema,
)]
#[diesel(table_name = tokens)]
pub struct Token {
  pub id: uuid::Uuid,
  #[schema(example = "nh_H.m1AkVz7r8RaC0zPFmchayJD1AqzSv4swudYBIBc3C")]
  pub value: String,
  pub kind: String,
  pub user_id: uuid::Uuid,
  pub created_at: chrono::NaiveDateTime,
  pub expires_at: Option<chrono::NaiveDateTime>,
  pub session: Option<String>,
}

#[derive(Clone, Debug, Insertable)]
#[diesel(table_name = tokens)]
pub struct TokenPartial {
  pub kind: String,
  pub value: String,
  pub user_id: uuid::Uuid,
  pub expires_at: Option<chrono::NaiveDateTime>,
  pub session: Option<String>,
}

impl TokenPartial {
  pub fn generate(user_id: &uuid::Uuid, kind: &str) -> Self {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    // Encode the bytes as a Base64 string
    const CUSTOM_ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(
      &alphabet::BCRYPT,
      engine::general_purpose::NO_PAD,
    );
    let value = CUSTOM_ENGINE.encode(bytes);
    let value = format!("nh_{value}");
    TokenPartial {
      value,
      kind: kind.to_string(),
      user_id: *user_id,
      expires_at: None,
      session: None,
    }
  }
}

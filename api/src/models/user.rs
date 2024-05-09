use serde::{Serialize, Deserialize};
use argon2::{
  password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
  Argon2, PasswordVerifier,
};

use crate::error::NError;
use crate::schema::{users, user_roles};

#[derive(Clone, Debug, Serialize, Deserialize, Queryable, utoipa::ToSchema)]
#[diesel(table_name = users)]
pub struct User {
  pub id: uuid::Uuid,
  pub name: String,
  pub email: String,
  pub password: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub is_email_verified: bool,
}

impl User {
  pub fn hash_password(password: &str) -> Result<String, NError> {
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    let hash = argon2
      .hash_password(password.as_bytes(), &salt)?
      .to_string();
    Ok(hash)
  }

  pub fn verify_password(&self, password: &str) -> Result<(), NError> {
    let argon2 = Argon2::default();
    let password_hash = argon2::PasswordHash::new(&self.password)?;
    argon2
      .verify_password(password.as_bytes(), &password_hash)
      .map_err(|_| NError::invalid_data("Credential incorrect"))?;
    Ok(())
  }
}

#[derive(
  Clone, Debug, Serialize, Deserialize, Insertable, utoipa::ToSchema,
)]
#[diesel(table_name = users)]
pub struct UserPartial {
  #[schema(example = "my_username")]
  pub name: String,
  #[schema(example = "my_email@my_domain.com")]
  pub email: String,
  #[schema(example = "my_strong_password123")]
  pub password: String,
}

impl UserPartial {
  pub fn validate(&self) -> Result<(), NError> {
    self.validate_name()?;
    self.validate_password()?;
    self.validate_email()?;
    Ok(())
  }

  fn validate_name(&self) -> Result<(), NError> {
    if self.name.len() < 4 {
      return Err(NError::invalid_data(
        "Username must be at least 4 characters long.",
      ));
    }
    if self.name.len() > 12 {
      return Err(NError::invalid_data(
        "Username must be at most 12 characters long.",
      ));
    }
    if !self.name.chars().all(|c| c.is_alphanumeric()) {
      return Err(NError::invalid_data("Username must be alphanumeric."));
    }
    Ok(())
  }

  fn validate_password(&self) -> Result<(), NError> {
    if self.password.len() < 8 {
      return Err(NError::invalid_data(
        "Password must be at least 8 characters long.",
      ));
    }
    if self.password.len() > 32 {
      return Err(NError::invalid_data(
        "Password must be at most 32 characters long.",
      ));
    }
    // allow alphanumeric and special characters for the password
    if !self.password.chars().all(|c| c.is_ascii()) {
      return Err(NError::invalid_data("Password contain invalid characters."));
    }
    Ok(())
  }

  fn validate_email(&self) -> Result<(), NError> {
    if !self.email.contains('@') {
      return Err(NError::invalid_data("Invalid email address"));
    }
    if !self.email.contains('.') {
      return Err(NError::invalid_data("Invalid email address"));
    }
    Ok(())
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserCredential {
  #[schema(example = "my_username | my_email@my_domain.com")]
  pub key: String,
  #[schema(example = "my_strong_password123")]
  pub password: String,
}

#[derive(Clone, Debug, Insertable, Queryable)]
#[diesel(table_name = user_roles)]
pub struct UserRole {
  pub user_id: uuid::Uuid,
  pub role_id: uuid::Uuid,
}

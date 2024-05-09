use ntex::web;
use diesel::prelude::*;

use crate::store;
use crate::{
  models::token::{Token, TokenPartial},
  store::Pool,
  error::HttpError,
};

impl Token {
  pub async fn create(
    token: &TokenPartial,
    db: &Pool,
  ) -> Result<Token, HttpError> {
    use crate::schema::tokens::dsl;
    let db = db.clone();
    let token = token.clone();
    let token = web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      let token = diesel::insert_into(dsl::tokens)
        .values(token)
        .get_result(&mut conn)?;
      Ok::<_, HttpError>(token)
    })
    .await?;
    Ok(token)
  }

  pub async fn find_by_value(
    value: &str,
    db: &Pool,
  ) -> Result<Token, HttpError> {
    use crate::schema::tokens::dsl;
    let db = db.clone();
    let value = value.to_owned();
    let exists = web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      let exists = dsl::tokens
        .filter(dsl::value.eq(value))
        .get_result::<Token>(&mut conn)?;
      Ok::<_, HttpError>(exists)
    })
    .await?;
    Ok(exists)
  }
}

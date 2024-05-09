use ntex::web;
use diesel::prelude::*;

use crate::store;
use crate::error::HttpError;
use crate::models::role::{Role, RolePartial};

impl Role {
  pub async fn create(
    role: &RolePartial,
    db: &store::Pool,
  ) -> Result<Role, HttpError> {
    use crate::schema::roles::dsl;
    let db = db.clone();
    let role = role.clone();
    let role = web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      let role = diesel::insert_into(dsl::roles)
        .values(role)
        .get_result(&mut conn)?;
      Ok::<_, HttpError>(role)
    })
    .await?;
    Ok(role)
  }
}

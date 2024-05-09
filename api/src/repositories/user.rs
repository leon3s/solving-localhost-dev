use ntex::web;
use diesel::prelude::*;

use crate::models::token::{Token, TokenPartial};
use crate::store;
use crate::error::HttpError;
use crate::models::user::{User, UserPartial, UserCredential, UserRole};

impl User {
  pub async fn create(
    user: &UserPartial,
    db: &store::Pool,
  ) -> Result<User, HttpError> {
    use crate::schema::users::dsl;
    user.validate()?;
    let db = db.clone();
    let mut user = user.clone();
    user.password = User::hash_password(&user.password)?;
    let user = web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      let user = diesel::insert_into(dsl::users)
        .values(user)
        .get_result(&mut conn)?;
      Ok::<_, HttpError>(user)
    })
    .await?;
    Ok(user)
  }

  pub async fn list(db: &store::Pool) -> Result<Vec<User>, HttpError> {
    use crate::schema::users::dsl;
    let db = db.clone();
    let users = web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      let users = dsl::users.load(&mut conn)?;
      Ok::<_, HttpError>(users)
    })
    .await?;
    Ok(users)
  }

  pub async fn find_by_id(
    id: &uuid::Uuid,
    db: &store::Pool,
  ) -> Result<User, HttpError> {
    use crate::schema::users::dsl;
    let db = db.clone();
    let id = *id;
    let user = web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      let user = dsl::users.find(id).get_result(&mut conn)?;
      Ok::<_, HttpError>(user)
    })
    .await?;
    Ok(user)
  }

  pub async fn find_by_email(
    email: &str,
    db: &store::Pool,
  ) -> Result<User, HttpError> {
    use crate::schema::users::dsl;
    let db = db.clone();
    let email = email.to_owned();
    let user = web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      let user = dsl::users
        .filter(dsl::email.eq(email))
        .get_result(&mut conn)?;
      Ok::<_, HttpError>(user)
    })
    .await?;
    Ok(user)
  }

  pub async fn find_by_name(
    name: &str,
    db: &store::Pool,
  ) -> Result<User, HttpError> {
    use crate::schema::users::dsl;
    let db = db.clone();
    let name = name.to_owned();
    let user = web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      let user = dsl::users
        .filter(dsl::name.eq(name))
        .get_result(&mut conn)?;
      Ok::<_, HttpError>(user)
    })
    .await?;
    Ok(user)
  }

  pub async fn login(
    credential: &UserCredential,
    db: &store::Pool,
  ) -> Result<Token, HttpError> {
    let user = if let Ok(user) = User::find_by_name(&credential.key, db).await {
      user
    } else {
      User::find_by_email(&credential.key, db).await?
    };
    user.verify_password(&credential.password)?;
    let token = TokenPartial::generate(&user.id, "auth");

    let token = Token::create(&token, db).await?;

    Ok(token)
  }
}

impl UserRole {
  pub async fn create(
    user_role: &UserRole,
    db: &store::Pool,
  ) -> Result<UserRole, HttpError> {
    use crate::schema::user_roles::dsl;
    let db = db.clone();
    let user_role = user_role.clone();
    let user_role = web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      let user_role = diesel::insert_into(dsl::user_roles)
        .values(user_role)
        .get_result(&mut conn)?;
      Ok::<_, HttpError>(user_role)
    })
    .await?;
    Ok(user_role)
  }

  pub async fn delete(
    user_id: &uuid::Uuid,
    role_id: &uuid::Uuid,
    db: &store::Pool,
  ) -> Result<(), HttpError> {
    use crate::schema::user_roles::dsl;
    let db = db.clone();
    let user_id = *user_id;
    let role_id = *role_id;
    web::block(move || {
      let mut conn = store::get_pool_conn(&db)?;
      diesel::delete(
        dsl::user_roles
          .filter(dsl::user_id.eq(user_id))
          .filter(dsl::role_id.eq(role_id)),
      )
      .execute(&mut conn)?;
      Ok::<_, HttpError>(())
    })
    .await?;
    Ok(())
  }
}

use serde::{Serialize, Deserialize};

use crate::schema::roles;

#[derive(
  Clone, Debug, Serialize, Identifiable, Queryable, utoipa::ToSchema,
)]
#[diesel(table_name = roles)]
pub struct Role {
  pub id: uuid::Uuid,
  pub name: String,
  pub realm: String,
  pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Insertable, utoipa::ToSchema)]
#[diesel(table_name = roles)]
pub struct RolePartial {
  pub name: String,
  pub realm: String,
  pub description: Option<String>,
}

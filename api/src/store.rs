use ntex::web;
use diesel::PgConnection;
use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::error::NError;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type DBState = web::types::State<Pool>;

pub async fn create_pool(url: &str) -> Result<Pool, NError> {
  let db_url = url.to_owned();
  let res = web::block(move || {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder().build(manager)
  })
  .await?;

  Ok(res)
}

pub fn get_pool_conn(pool: &Pool) -> std::io::Result<DBConn> {
  let conn = pool
    .get()
    .map_err(|err| std::io::Error::new(std::io::ErrorKind::TimedOut, err))?;
  Ok(conn)
}

/// Ensure existance of a container for our store
/// we use cockroachdb with a postgresql connector.
/// we also run latest migration on our database to have the latest schema.
/// It will return a connection Pool that will be use in our State.
pub(crate) async fn init(url: &str) -> Result<Pool, NError> {
  const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");
  // let url = utils::store::get_url(docker_api).await?;
  log::info!("Connecting to store at: {url}");
  // Connect to postgresql
  let pool = create_pool(url).await?;
  let mut conn = get_pool_conn(&pool)?;
  log::info!("Store connected");
  // This will run the necessary migrations.
  // See the documentation for `MigrationHarness` for
  // all available methods.
  log::info!("Running migrations");
  conn.run_pending_migrations(MIGRATIONS).map_err(|err| {
    NError::other(format!("Error when running migrations: {err:?}"))
  })?;
  log::info!("Migrations done");
  Ok(pool)
}

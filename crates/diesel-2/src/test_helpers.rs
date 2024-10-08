use std::sync::{Mutex, MutexGuard};

use diesel::prelude::*;
use diesel_migrations::{
  FileBasedMigrations, MigrationHarness,
};
use lazy_static::lazy_static;

pub fn connection() -> PgConnection {
  let url = dotenvy::var("PG_DATABASE_URL")
    .or_else(|_| dotenvy::var("DATABASE_URL"))
    .expect("DATABASE_URL must be set");
  let mut conn =
    PgConnection::establish(&url).unwrap();
  conn.begin_test_transaction().unwrap();
  diesel::sql_query(
    "DROP TABLE IF EXISTS users CASCADE",
  )
  .execute(&mut conn)
  .unwrap();
  diesel::sql_query(
    "DROP TABLE IF EXISTS posts CASCADE",
  )
  .execute(&mut conn)
  .unwrap();
  diesel::sql_query(
    "DROP TABLE IF EXISTS comments CASCADE",
  )
  .execute(&mut conn)
  .unwrap();

  let migrations =
    FileBasedMigrations::find_migrations_directory()
      .unwrap();
  conn
    .run_pending_migrations(migrations)
    .unwrap();
  conn
}

pub fn this_test_modifies_env(
) -> MutexGuard<'static, ()> {
  let _ = dotenvy::var("FORCING_DOTENV_LOAD");
  lazy_static! {
    static ref ENV_LOCK: Mutex<()> =
      Mutex::new(());
  }
  ENV_LOCK.lock().unwrap()
}

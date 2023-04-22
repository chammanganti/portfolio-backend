use rocket_sync_db_pools::database;

pub mod projects;

#[database("portfolio")]
pub struct Db(diesel::PgConnection);

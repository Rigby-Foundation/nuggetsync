use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::env;
use std::error::Error;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn run_migrations(pool: &DbPool) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut conn = pool.get()?;
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

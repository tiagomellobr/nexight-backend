// Módulo de configurações de banco de dados
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool")
}

pub fn run_migrations(pool: &DbPool) {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    
    let mut conn = pool.get().expect("Failed to get connection from pool");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    
    log::info!("✅ Database migrations completed successfully");
}
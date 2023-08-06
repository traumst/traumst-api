use std::time::Duration;
use log::info;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Executor, Pool, Sqlite};
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqlitePoolOptions;

const DB_URL: &str = "sqlite://main.db";
const DB_POOL_SIZE: u32 = 2;

/// Ensures there's a sqlite db and tables, or panics
pub async fn create() -> Pool<Sqlite> {
    let db_pool = create_database().await;
    let db_conn = db_pool.acquire().await
        .expect("Failed to acquire db connection");
    create_tables(db_conn).await;

    db_pool
}

async fn create_database() -> Pool<Sqlite> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        info!("Creating database...");
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => info!("Database created"),
            Err(error) => panic!("Failed to create database: {error:?}")
        }
    }

    SqlitePoolOptions::new()
        .max_connections(DB_POOL_SIZE)
        .acquire_timeout(Duration::from_secs(5))
        .connect(DB_URL).await.expect("Failed to connect to database")
}

async fn create_tables(mut db_conn: PoolConnection<Sqlite>) {
    match db_conn.execute(sqlx::query("
    create table if not exists main.users (
        name TEXT    not null,
        pass TEXT    not null,
        hash INTEGER not null
            constraint users_pk
                primary key
                    on conflict rollback
    ) without rowid;")).await {
        Ok(_) => info!("Tables verified to exist"),
        Err(error) => panic!("Failed to create table/s: {error:?}")
    }
}
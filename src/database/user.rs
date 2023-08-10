use log::{debug, error};
use sqlx::pool::PoolConnection;
use sqlx::{Executor, Execute, Row, Sqlite};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub pass: String,
    pub hash: u32,
}

pub async fn create(mut conn: PoolConnection<Sqlite>, user: User) -> Result<(), String> {
    let query = sqlx::query(
        "INSERT OR IGNORE INTO users (name, pass, hash) VALUES ('?', '?', ?)")
        .bind(user.name)
        .bind(user.pass)
        .bind(user.hash);

    debug!("executing query {}", query.sql());

    match conn.execute(query).await {
        Ok(_) => { Ok(()) }
        Err(err) => { Err(format!("Failed to insert user err, {err:?}")) }
    }
}

pub async fn read(mut conn: PoolConnection<Sqlite>, hash: u32) -> Option<User> {
    let query = sqlx::query(
        "SELECT name, pass, hash FROM users WHERE hash = $1")
        .bind(hash);

    match conn.fetch_one(query).await {
        Ok(row) => Some(User {
            name: row.get(0),
            pass: row.get(1),
            hash: row.get(2),
        }),
        Err(err) => {
            error!("Failed to read user by hash:{hash:?}, err:{err:?}");
            None
        }
    }
}

pub async fn update(mut conn: PoolConnection<Sqlite>, hash: u32, name: String) {
    todo!("Forgot to implement the update");
}
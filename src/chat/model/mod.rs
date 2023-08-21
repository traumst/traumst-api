mod auth;
mod chat;
mod user;

use serde::{Serialize, Deserialize};
pub use auth::{Auth, AuthType};
pub use chat::*;
pub use user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Avatar {
    pub id: u32,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: u32,
    pub name: String,
}
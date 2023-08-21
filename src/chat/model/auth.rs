use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    pub id: u32,
    pub user: u32,
    pub auth_type: AuthType,
    pub auth_value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    Email,
    Google,
    Eth,
}
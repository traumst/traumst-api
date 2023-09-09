use log::error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Auth {
    #[serde(default)]
    pub kind: String,
    pub hash: u32,
}

impl Auth {
    pub fn from_string(json: &str) -> Option<Self> {
        match serde_json::from_str(json) {
            Ok(user) => Some(user),
            Err(err) => {
                error!("Failed to parse Auth, {err:?}");
                None
            }
        }
    }

    // pub fn to_string(&self) -> String {
    //     match serde_json::to_string(self) {
    //         Ok(json) => json,
    //         Err(err) => {
    //             error!("Failed to stringify Auth, {err:?}");
    //             "".to_string()
    //         }
    //     }
    // }
}
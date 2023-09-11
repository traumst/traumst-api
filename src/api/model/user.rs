use log::error;
use serde::{
    Serialize,
    Deserialize
};

use super::Auth;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: u32,
    pub name: String,
    pub hash: u32,
    pub chats: Option<Vec<u32>>,
    pub auth: Option<Auth>,
}

impl User {
    pub fn from_string(json: &str) -> Option<Self> {
        match serde_json::from_str(json) {
            Ok(user) => Some(user),
            Err(err) => {
                error!("Failed to parse User, {err:?}");
                None
            }
        }
    }

    pub fn to_string(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(err) => {
                error!("Failed to stringify User, {err:?}");
                "".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_user() -> User {
        User {
            id: 1,
            name: "oioi".to_string(),
            hash: 227755,
            chats: Some(vec![222, 333, 444]),
            auth: Some(Auth {
                kind: "Hello".to_string(),
                hash: 6969,
            }),
        }
    }

    fn json_user() -> String {
        let user = test_user();
        serde_json::to_string(&user).expect("bad user - cant deserialize")
    }

    #[test]
    fn test_to_string() {
        let user_str = json_user();
        assert_ne!(user_str, String::new());
        assert!(user_str.contains("oioi"));
        assert!(user_str.contains("227755"));
        assert!(user_str.contains("222,333,444"));
        assert!(user_str.contains("Hello"));
        assert!(user_str.contains("6969"));
    }

    #[test]
    fn test_from_string_with_defaults() {
        let json = "{\"name\":\"hello\",\"hash\":123456}";
        let user = User::from_string(json).unwrap();
        assert_eq!(user.id, 0);
        assert_eq!(user.name, "hello");
        assert_eq!(user.hash, 123456);
        assert!(user.auth.is_none());
        assert!(user.chats.is_none());
    }

    #[test]
    fn test_from_string() {
        let test_user = test_user();
        let json = json_user();
        let user = User::from_string(json.as_str()).unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.name, test_user.name);
        assert_eq!(user.hash, test_user.hash);
        assert_eq!(user.auth, test_user.auth);
    }
}
use std::sync::Arc;
use log::info;
use crate::chat;
use crate::chat::model::Avatar;
use crate::db;
use crate::infra;
use crate::infra::email::EmailRequest;
use crate::server::response;

pub fn send_email(_: &str, body: &str) -> Result<response::Response, String> {
    match infra::email::parse_request(body) {
        Ok(json) => { handle_success(json) }
        Err(msg) => { Err(msg) }
    }
}

pub async fn create_user(head: &str, body: &str, shared_pool: Arc<db::pool::Bridge>)
    -> Result<response::Response, String>
{
    let user = chat::model::User::from_string(body);
    if user.is_none() {
        return Err(format!("Cannot parse input json"));
    }

    let user = user.unwrap();
    let new_user = db::model::User {
        id: 0,
        name: user.name,
        hash: user.hash,
        avatar: Avatar { id: 0, value: String::new()}.id,
    };

    let user_id = shared_pool.create_user(new_user).await;
    if user_id == 0 {
        return Err(format!("User creation failed: {body}"));
    }

    Ok(response::Response {
        status_code: "200",
        status_message: "Ok",
        headers: head.to_string(),
        body: user_id.to_string(),
    })
}

pub async fn get_user(head: &str, user_id: u32, shared_pool: Arc<db::pool::Bridge>) -> Result<response::Response, String> {
    match shared_pool.get_user(user_id).await {
        None => {
            info!("User not found by id: {user_id}");
            Err("User not found by id".to_string())
        }
        Some(user) => {
            Ok(response::Response {
                status_code: "200",
                status_message: "Ok",
                headers: head.to_string(),
                body: chat::model::User {
                    id: user.id,
                    name: user.name,
                    hash: user.hash,
                    auth: None, // find by user.auth.id
                    avatar: None,
                    contacts: None,
                    chats: None,
                }.to_string(),
            })
        }
    }
}

pub fn handle_success(json: EmailRequest) -> Result<response::Response, String> {
    match infra::email::send_email(json) {
        Ok(res) => Ok(response::Response {
            status_code: "200",
            status_message: "OK",
            headers: "".to_string(),
            body: res,
        }),
        Err(err) => Err(err)
    }
}
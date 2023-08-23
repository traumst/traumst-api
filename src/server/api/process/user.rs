use std::sync::Arc;
use log::info;
use crate::chat;
use crate::db;
use crate::server::response;

pub async fn create(head: &str, body: &str, shared_pool: Arc<db::pool::Bridge>)
                    -> Result<response::Response, response::Response>
{
    let user = chat::model::User::from_string(body);
    if user.is_none() {
        return Err(response::Response {
            status_code: "400".to_string(),
            status_message: "Bad Request".to_string(),
            headers: "".to_string(),
            body: format!("Cannot parse input json"),
        });
    }

    let user = user.unwrap();
    if user.id > 0 {
        return Err(response::Response {
            status_code: "400".to_string(),
            status_message: "Bad Request".to_string(),
            headers: "".to_string(),
            body: format!("User already exists: {body}"),
        });
    }

    let new_user = db::model::User {
        id: user.id,
        name: user.name,
        hash: user.hash,
        avatar: chat::model::Avatar { id: 0, value: String::new() }.id,
    };

    let user_id = shared_pool.create_user(new_user).await;
    if user_id == 0 {
        return Err(response::Response {
            status_code: "504".to_string(),
            status_message: "Gateway Timeout".to_string(),
            headers: "".to_string(),
            body: format!("User creation failed: {body}"),
        });
    }

    Ok(response::Response {
        status_code: "200".to_string(),
        status_message: "Ok".to_string(),
        headers: head.to_string(),
        body: user_id.to_string(),
    })
}

pub async fn get(head: &str, user_id: u32, shared_pool: Arc<db::pool::Bridge>)
                 -> Result<response::Response, response::Response>
{
    match shared_pool.get_user(user_id).await {
        None => {
            info!("User not found by id: {user_id}");
            Err(response::Response {
                status_code: "404".to_string(),
                status_message: "Not Found".to_string(),
                headers: "".to_string(),
                body: "".to_string(),
            })
        }
        Some(user) => {
            Ok(response::Response {
                status_code: "200".to_string(),
                status_message: "Ok".to_string(),
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
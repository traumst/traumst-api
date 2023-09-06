use std::str::FromStr;
use std::sync::Arc;
use log::warn;
use crate::chat;
use crate::db;
use crate::api::response;

pub async fn create(
    head: &str,
    body: &str,
    shared_pool: Arc<db::pool::Bridge>
) -> Result<response::Response, response::Response> {
    match parse_creation_request(body) {
        Err(err_response) => Err(err_response),
        Ok(new_auth) => {
            let auth_id = shared_pool.create_auth(new_auth).await;
            if auth_id == 0 {
                return Err(response::Response {
                    status_code: "504".to_string(),
                    status_message: "Gateway Timeout".to_string(),
                    headers: "".to_string(),
                    body: format!("Auth creation failed: {body}"),
                });
            }

            Ok(response::Response {
                status_code: "200".to_string(),
                status_message: "Ok".to_string(),
                headers: head.to_string(),
                body: auth_id.to_string(),
            })
        }
    }
}

fn parse_creation_request(body: &str) -> Result<db::model::Auth, response::Response> {
    let auth = chat::model::Auth::from_string(body);
    if auth.is_none() {
        return Err(response::Response {
            status_code: "400".to_string(),
            status_message: "Bad Request".to_string(),
            headers: "".to_string(),
            body: format!("Cannot parse input json"),
        });
    }

    let auth = auth.unwrap();
    if auth.id > 0 {
        return Err(response::Response {
            status_code: "400".to_string(),
            status_message: "Bad Request".to_string(),
            headers: "".to_string(),
            body: format!("User already exists: {body}"),
        });
    }

    let new_auth = db::model::Auth {
        id: auth.id,
        user_id: auth.user,
        auth_type: db::model::AuthType::from_str(auth.auth_type.as_str()).unwrap_or_default(),
        auth_value: auth.auth_value,
    };

    Ok(new_auth)
}

pub async fn get(
    head: &str,
    auth_type: String,
    auth_value: u32,
    shared_pool: Arc<db::pool::Bridge>
) -> Result<response::Response, response::Response> {
    match shared_pool.get_auth(
        auth_type.clone(),
        auth_value.clone()
    ).await {
        None => {
            warn!("Auth not found for {auth_type:?}:{auth_value:?}");
            Err(response::Response {
                status_code: "404".to_string(),
                status_message: "Not Found".to_string(),
                headers: "".to_string(),
                body: "".to_string(),
            })
        }
        Some(auth) => Ok(response::Response {
            status_code: "200".to_string(),
            status_message: "Ok".to_string(),
            headers: head.to_string(),
            body: chat::model::Auth {
                id: auth.id,
                user: auth.user_id,
                auth_type: auth.auth_type.to_string(),
                auth_value: auth.auth_value,
            }.to_string(),
        })
    }
}
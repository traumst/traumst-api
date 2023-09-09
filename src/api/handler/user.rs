use std::sync::Arc;
use log::warn;
use serde::{Deserialize, Serialize};
use crate::db;
use crate::api;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserAction {
    Create,
    Auth,
}

pub async fn process(
    action: UserAction,
    head: &str,
    body: &str,
    shared_pool: Arc<db::pool::Bridge>
) -> Result<api::response::Response, api::response::Response> {
    match action {
        UserAction::Create => create(head, body, shared_pool).await,
        UserAction::Auth => auth(head, body, shared_pool).await,
    }
}

async fn create(
    head: &str,
    body: &str,
    shared_pool: Arc<db::pool::Bridge>
) -> Result<api::response::Response, api::response::Response> {
    match parse_creation_request(body) {
        Err(err_response) => Err(err_response),
        Ok(user) => {
            let auth = user.auth.clone();
            let auth = match auth {
                Some(auth) => auth,
                None => {
                    return Err(api::response::Response {
                        status_code: "400".to_string(),
                        status_message: "Bad Request".to_string(),
                        headers: "".to_string(),
                        body: format!("missing auth details"),
                    });
                }
            };

            create_user(head, user, auth, shared_pool).await
        }
    }
}
pub async fn auth(head: &str, body: &str, shared_pool: Arc<db::pool::Bridge>)
    -> Result<api::response::Response, api::response::Response>
{
    match api::model::Auth::from_string(body) {
        None => Err(api::response::Response {
            status_code: "400".to_string(),
            status_message: "Bad Request".to_string(),
            headers: "".to_string(),
            body: format!("authentication failed"),
        }),
        Some(auth) => match shared_pool.get_auth(auth.kind, auth.hash).await {
            None => Err(api::response::Response {
                status_code: "404".to_string(),
                status_message: "Not Found".to_string(),
                headers: "".to_string(),
                body: "".to_string(),
            }),
            Some(auth) => match shared_pool.get_user(auth.user_id).await {
                None => {
                    warn!("User not found by id: {:?}", auth.user_id);
                    Err(api::response::Response {
                        status_code: "404".to_string(),
                        status_message: "Not Found".to_string(),
                        headers: "".to_string(),
                        body: "".to_string(),
                    })
                }
                Some(user) => Ok(api::response::Response {
                    status_code: "200".to_string(),
                    status_message: "Ok".to_string(),
                    headers: head.to_string(),
                    body: api::model::User {
                        id: user.id,
                        name: user.name,
                        hash: user.hash,
                        auth: None, // find by user.auth type and value
                        chats: None, // find by user.id
                    }.to_string(),
                })
            }
        }
    }
}

async fn create_user(
    head: &str,
    user: api::model::User,
    auth: api::model::Auth,
    shared_pool: Arc<db::pool::Bridge>
) -> Result<api::response::Response, api::response::Response> {
    let kind = auth.kind.parse().expect("missing auth kind");
    let hash = auth.hash;
    let user_id = shared_pool.create_user(
        db::model::User {
            id: user.id,
            name: user.name,
            hash: user.hash,
        },
        db::model::Auth {
            id: 0,
            user_id: user.id,
            kind,
            hash,
        }
    ).await;

    if user_id == 0 {
        return Err(api::response::Response {
            status_code: "504".to_string(),
            status_message: "Gateway Timeout".to_string(),
            headers: "".to_string(),
            body: format!("user creation failed"),
        });
    }

    Ok(api::response::Response {
        status_code: "200".to_string(),
        status_message: "Ok".to_string(),
        headers: head.to_string(),
        body: user_id.to_string(),
    })
}

fn parse_creation_request(body: &str) -> Result<api::model::User, api::response::Response> {
    let user = api::model::User::from_string(body);
    if user.is_none() {
        return Err(api::response::Response {
            status_code: "400".to_string(),
            status_message: "Bad Request".to_string(),
            headers: "".to_string(),
            body: format!("Cannot parse input json"),
        });
    }

    let user = user.unwrap();
    if user.id > 0 {
        return Err(api::response::Response {
            status_code: "400".to_string(),
            status_message: "Bad Request".to_string(),
            headers: "".to_string(),
            body: format!("User already exists: {body}"),
        });
    }

    Ok(user)
}
use std::sync::Arc;
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
    body: &str,
    shared_pool: Arc<db::pool::Bridge>
) -> Result<api::response::Response, api::response::Response> {
    match action {
        UserAction::Create => create(body, shared_pool).await,
        UserAction::Auth => auth(body, shared_pool).await,
    }
}

async fn create(
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
                    return Err(api::response::err400(format!("missing auth details")));
                }
            };

            create_user(user, auth, shared_pool).await
        }
    }
}

pub async fn auth(
    body: &str,
    shared_pool: Arc<db::pool::Bridge>
) -> Result<api::response::Response, api::response::Response> {
    match api::model::Auth::from_string(body) {
        None => Err(api::response::err400(
            format!("authentication failed"))),
        Some(auth) => match shared_pool.get_auth(auth.kind, auth.hash).await {
            None => Err(api::response::err404("invalid auth".to_string())),
            Some(auth) => match shared_pool.get_user(auth.user_id).await {
                None => Err(api::response::err404("invalid user".to_string())),
                // AL TODO provide header
                Some(user) => Ok(api::response::ok200(api::model::User {
                    id: user.id,
                    name: user.name,
                    hash: user.hash,
                    auth: None, // find by user.auth type and value
                    chats: None, // find by user.id
                }.to_string()))
            }
        }
    }
}

async fn create_user(
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
        return Err(api::response::err504(format!("user creation failed")));
    }

    // TODO provide header
    Ok(api::response::ok200(user_id.to_string()))
}

fn parse_creation_request(body: &str) -> Result<api::model::User, api::response::Response> {
    let user = api::model::User::from_string(body);
    if user.is_none() {
        return Err(api::response::err400(
            format!("Cannot parse input json")));
    }

    let user = user.unwrap();
    if user.id > 0 {
        return Err(api::response::err400(
            format!("User already exists: {body}")));
    }

    Ok(user)
}
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::api::response;
use crate::db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatAction {
    Create,
    //Delete,
    Join,
    //Leave,
    Send,
    //Unsend,
}

pub async fn process(
    action: ChatAction,
    head: &str,
    _body: &str,
    _shared_pool: Arc<db::pool::Bridge>
) -> Result<response::Response, response::Response> {
    match action {
        ChatAction::Create => Err(response::Response {
            status_code: "501".to_string(),
            status_message: "Not Implemented".to_string(),
            headers: head.to_string(),
            body: "".to_string(),
        }),
        ChatAction::Join => Err(response::Response {
            status_code: "501".to_string(),
            status_message: "Not Implemented".to_string(),
            headers: head.to_string(),
            body: "".to_string(),
        }),
        ChatAction::Send => Err(response::Response {
            status_code: "501".to_string(),
            status_message: "Not Implemented".to_string(),
            headers: head.to_string(),
            body: "".to_string(),
        }),
    }
}
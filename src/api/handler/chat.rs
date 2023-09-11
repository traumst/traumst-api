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
    _body: &str,
    _shared_pool: Arc<db::pool::Bridge>
) -> Result<response::Response, response::Response> {
    match action {
        ChatAction::Create => Err(response::err501()),
        ChatAction::Join => Err(response::err501()),
        ChatAction::Send => Err(response::err501()),
    }
}
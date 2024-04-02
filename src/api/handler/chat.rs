use serde::{Deserialize, Serialize};
use crate::api::response;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatAction {
    Create,
    Delete,
    Invite,
    Leave,
    Send,
    Unsend,
}

pub async fn _process(
    action: ChatAction,
    _body: &str,
) -> Result<response::Response, response::Response> {
    match action {
        ChatAction::Create => Err(response::err501()),
        ChatAction::Delete => Err(response::err501()),
        ChatAction::Invite => Err(response::err501()),
        ChatAction::Leave => Err(response::err501()),
        ChatAction::Send => Err(response::err501()),
        ChatAction::Unsend => Err(response::err501()),
    }
}
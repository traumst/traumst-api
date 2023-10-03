#[derive(Debug, Clone)]
pub struct Chat {
    pub id: u32,
    pub users: Vec<u32>,
    pub history: Option<Vec<ChatMessage>>,
}
impl Default for Chat {
    fn default() -> Self {
        Self {
            id: 0,
            users: vec![],
            history: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub header: ChatHeader,
    pub body: String,
    pub footer: ChatFooter,
}

#[derive(Debug, Clone)]
pub struct ChatHeader {
    pub avatar: u32,
    pub user: u32,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub struct ChatFooter {
    pub sent: bool,
    pub read: bool,
    pub options: Vec<ChatOption>
}

#[derive(Debug, Clone)]
pub struct ChatOption {
    pub emote: Vec<String>,
    pub act: Vec<String>,
}
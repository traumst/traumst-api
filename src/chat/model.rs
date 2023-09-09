// #[derive(Debug, Clone)]
// pub struct Chat {
//     id: u32,
//     users: Vec<u32>,
//     history: Option<Vec<ChatMessage>>,
// }
//
// #[derive(Debug, Clone)]
// pub struct ChatMessage {
//     header: ChatHeader,
//     body: String,
//     footer: ChatFooter,
// }
//
// #[derive(Debug, Clone)]
// pub struct ChatHeader {
//     avatar: u32,
//     user: u32,
//     timestamp: String,
// }
//
// #[derive(Debug, Clone)]
// pub struct ChatFooter {
//     sent: bool,
//     read: bool,
//     options: Vec<ChatOption>
// }
//
// #[derive(Debug, Clone)]
// pub struct ChatOption {
//     emote: Vec<String>,
//     act: Vec<String>,
// }
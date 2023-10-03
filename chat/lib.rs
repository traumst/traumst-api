mod model;

pub mod app {
    use super::model;

    pub struct App {
        chats: Vec<model::Chat>,
    }

    impl App {
        pub fn new() -> Self {
            Self {
                chats: vec![],
            }
        }

        pub fn create_chat(&self, user_id: u32) -> u32 {
            let last_chat_id = self.chats.iter()
                .max_by_key(|&chat| { return chat.id })
                .unwrap_or(&model::Chat::default())
                .id;

            let new_chat = model::Chat {
                id: last_chat_id + 1,
                users: vec![user_id],
                history: Some(vec![]),
            };

            new_chat.id
        }
    }
}
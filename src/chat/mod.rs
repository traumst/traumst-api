use model::User;

pub mod model;

pub struct App {
    owner: User,
    chats: Vec<model::Chat>,
}

impl App {
    pub fn new(owner: User) -> Self {
        Self {
            owner,
            chats: vec![],
        }
    }
}
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
    pub ok: bool,
    pub result: Vec<Update>,
}

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>, 
    pub inline_query: Option<InlineQuery>, 
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub chat: Chat,
    pub text: Option<String>,
    pub reply_to_message: Option<Box<Message>>,
}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub data: Option<String>,
    pub message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
    pub first_name: Option<String>,
    pub username: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub is_premium: Option<bool>,
}

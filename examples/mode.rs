use maestro_tgbot::{TelegramBot, bot};
use maestro_tgbot::types::*;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    
    let _ = TelegramBot::init("8520493598:AAFYiGyG_-Lvl7DW5YrTomCKCLEFuZxgsw8");

    let mut offset = 0;
    let keyboard = serde_json::json!({
    "inline_keyboard": [
        [
            {"text": "✅ Admin", "url": "t.me/famous_coder"},
            {"text": "👌 Click", "callback_data": "click"}
        ]
    ]
}).to_string();

    loop {
        let res = bot!("getUpdates", {
            "offset" => offset,
            "timeout" => 10
        }).unwrap();

        let data: Response = serde_json::from_str(&res).unwrap();

        for update in data.result {
            offset = update.update_id + 1;

            if let Some(message) = update.message {
                let cid = message.chat.id;
                let text = message.text.unwrap_or_default();
                let (first, last) = if let Some(user) = message.from {
        (
            user.first_name.unwrap_or_default(),
            user.last_name.unwrap_or_default(),
        )
    } else {
        ("".to_string(), "".to_string())
    };
                let name = format!("{} {}", first, last);
                if text == "/start" {
                let _res = bot!("sendMessage", {
    "chat_id" => cid ,
    "text" => format!("<b>👋 Hello</b> {}", name),
    "parse_mode" => "HTML",
    "reply_markup" => keyboard
});
}
            }
            if let Some(cb) = update.callback_query {
    let data = cb.data.unwrap_or_default();
let cid = cb.message.as_ref().unwrap().chat.id;
    let mid = cb.message.as_ref().unwrap().message_id;
    let qid = cb.id;
    let _ = bot!("answerCallbackQuery", {
                    "callback_query_id" => qid,
                    "text" => "OK 👍",
                    "show_alert"=>true,
                });
    if data == "click" {
    let _ = bot!("editMessageText", {
            "chat_id" => cid,
            "message_id" => mid,
            "text" => "Clicked 👌"
        });
    }
}
        }
        
       sleep(Duration::from_millis(800)).await;
    }

}

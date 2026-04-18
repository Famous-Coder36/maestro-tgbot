use maestro_tgbot::{TelegramBot, bot};
use serde_json::json;

#[tokio::main]
async fn main() {
    // Bot init
    let _ = TelegramBot::init("YOUR_BOT_TOKEN");

    // Inline keyboard
    let keyboard = json!({
        "inline_keyboard": [
            [
                {"text": "✅ ON", "callback_data": "mode_on"},
                {"text": "❌ OFF", "callback_data": "mode_off"}
            ]
        ]
    }).to_string();

    // Send message
    let res = bot!("sendMessage", {
        "chat_id" => "YOUR_CHAT_ID",
        "text" => "<b>Select mode:</b>",
        "parse_mode" => "HTML",
        "reply_markup" => keyboard
    });

    println!("Send result: {:?}", res);

}

#[macro_export]
macro_rules! bot {
    ($method:expr, { $($k:expr => $v:expr),* $(,)? }) => {{
        let mut data = std::collections::HashMap::new();

        $(
            data.insert($k.to_string(), $v.to_string());
        )*

        let bot = $crate::TelegramBot::get();

        bot.bot($method, data).await
    }};
}


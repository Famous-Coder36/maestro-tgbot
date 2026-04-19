# maestro-tgbot v0.0.2

Maestro-TgBot is a lightweight, high-performance Telegram Bot framework written in Rust.

It provides a simple yet powerful `bot!` macro for direct interaction with the Telegram Bot API, allowing developers to build bots quickly without complex abstractions.

Built on top of async Rust, it focuses on speed, simplicity, and minimal boilerplate while still giving full control over raw API calls.


---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
maestro-tgbot = "0.0.2"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"

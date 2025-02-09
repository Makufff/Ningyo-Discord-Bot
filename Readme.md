# 🌊 Ningyo - The Enchanted Mermaid Bot 🌊

![Ningyo Banner](/assets/banner.webp)

> "A mystical mermaid, emerging from the depths of the ocean, ready to guide and enchant your server..." 

## 🌊 About Ningyo
**Ningyo** (人形) is a mystical mermaid-themed Discord bot, built with **Rust** and powered by the **Serenity** library. Designed to bring an oceanic and fantasy-inspired experience to your Discord server, Ningyo is here to help, entertain, and create an enchanting community!

- 🌿 **Graceful & lightweight like the waves**
- 🌊 **Oceanic and mermaid-themed responses**
- 🌍 **Powered by Rust & Tokio async runtime**
- 🌟 **Customizable commands & mystical interactions**

---
## 🐠 Getting Started
### 🌊 Installation
#### Prerequisites:
- Install [Rust](https://www.rust-lang.org/tools/install)
- Set up a [Discord Bot Token](https://discord.com/developers/applications)
- Add `.env` file with your bot token:
  ```env
  DISCORD_TOKEN=your_bot_token_here
  ```

#### Clone & Run Ningyo:
```sh
git clone https://github.com/Makufff/Ningyo-Discord-Bot
cd Ningyo-Discord-Bot
cargo run
```

### 🌊 Project Structure
```
Ningyo-Discord-Bot/
├── Cargo.toml
├── .env
└── src/
    ├── main.rs
    ├── commands/
    │   ├── mod.rs
    │   ├── command_handler.rs
    │   ├── ping.rs
    │   └── hello.rs
    ├── handlers/
    │   ├── mod.rs
    │   └── message_handler.rs
    └── services/
        └── mod.rs
```

---
## 🌌 Commands
| Command  | Description |
|----------|------------|
| `!ping`  | Responds with **Echo from the deep!** |
| `!hello` | Greets with a mystical mermaid welcome |

---
## 🌊 Features
- 🎯 **SOLID Architecture**: Built with clean, maintainable code following SOLID principles
- 🔄 **Extensible Command System**: Easy to add new commands without modifying existing code
- 🌊 **Async Runtime**: Powered by Tokio for efficient async operations
- 🛡️ **Type-Safe**: Leveraging Rust's strong type system for reliability

---
## 🌊 Adding New Commands
1. Create a new file in `src/commands/` (e.g., `new_command.rs`)
2. Implement the `CommandExecutor` trait
3. Add the module to `commands/mod.rs`
4. Register the command in `MessageHandler::new()`

Example:
```rust
// src/commands/new_command.rs
use async_trait::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use super::command_handler::CommandExecutor;

pub struct NewCommand;

#[async_trait]
impl CommandExecutor for NewCommand {
    async fn execute(&self, ctx: &Context, msg: &Message) -> Result<(), String> {
        msg.channel_id.say(&ctx.http, "Your response here!").await
            .map_err(|e| format!("Error sending message: {e:?}"))
    }
}
```

---
## 🌌 Roadmap
- [ ] Slash Commands Support

---
## 🌊 Contributing
We welcome contributions! Here's how you can help:
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---
## 🌌 Connect With Us
Join our Discord server: Coming Soon

---
## 💀 License
MIT License — Free to use & modify!
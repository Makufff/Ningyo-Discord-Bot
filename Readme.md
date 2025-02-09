# 🌊 Ningyo - The Enchanted Mermaid Bot 🌊

![Ningyo Banner](/assets/banner.webp)

> "A mystical mermaid, emerging from the depths of the ocean, ready to guide and enchant your server..." 

## 🌊 About Ningyo
**Ningyo** (人形) is a mystical mermaid-themed Discord bot, built with **Rust** and powered by the **Serenity** library. Designed to bring an oceanic and fantasy-inspired experience to your Discord server, Ningyo is here to help, entertain, and create an enchanting community!

- 🌿 **Graceful & lightweight like the waves**
- 🌊 **Oceanic and mermaid-themed responses**
- 🌍 **Powered by Rust & Tokio async runtime**
- 🌟 **Modern Slash Commands for intuitive interaction**

---
## 🐠 Getting Started
### 🌊 Installation
#### Prerequisites:
- Install [Rust](https://www.rust-lang.org/tools/install)
- Set up a [Discord Bot Token](https://discord.com/developers/applications)
  - Enable "Message Content Intent" in your bot settings
  - Enable "Server Members Intent" in your bot settings
  - Enable "Presence Intent" in your bot settings
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
    │   └── interaction_handler.rs
    └── services/
        └── mod.rs
```

---
## 🌌 Commands
| Command    | Description |
|------------|------------|
| `/ping`    | Responds with latency information |
| `/hello`   | Greets with a mystical mermaid welcome |

---
## 🌊 Features
- 🎯 **Modern Architecture**: Built with clean, maintainable code
- ⚡ **Slash Commands**: Intuitive Discord-native command interface
- 🌊 **Async Runtime**: Powered by Tokio for efficient async operations
- 🛡️ **Type-Safe**: Leveraging Rust's strong type system for reliability

---
## 🌊 Adding New Slash Commands
1. Create a new command in your command handler:
```rust
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // Add your new command match case here
            let content = match command.data.name.as_str() {
                "ping" => "Pong!".to_string(),
                "hello" => "Greetings from the ocean depths! 🌊".to_string(),
                _ => "Command not found".to_string(),
            };
            
            command.create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
            .unwrap();
        }
    }
}
```

2. Register your command with Discord:
```rust
#[tokio::main]
async fn main() {
    // ... existing setup code ...
    
    let guild_id = GuildId(YOUR_GUILD_ID);
    
    GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        commands
            .create_application_command(|command| {
                command.name("your_command").description("Your command description")
            })
    })
    .await
    .unwrap();
}
```

---
## 🌌 Roadmap
- [x] Slash Commands Support

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
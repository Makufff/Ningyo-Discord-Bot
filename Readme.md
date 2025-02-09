# 🌊 Ningyo - The Enchanted Mermaid Bot 🌊

![Ningyo Banner](/assets/banner.webp)

> "A mystical mermaid, emerging from the depths of the ocean, ready to guide and enchant your server..." 

## 🌊 About Ningyo
**Ningyo** (人形) is a mystical mermaid-themed Discord bot, built with **Rust** and powered by the **Serenity** library. Designed to bring an oceanic and fantasy-inspired experience to your Discord server!

- 🌿 **Graceful & lightweight like the waves**
- 🌊 **Oceanic and mermaid-themed responses**
- 🌍 **Powered by Rust & Tokio async runtime**
- 🌟 **Modern Slash Commands for intuitive interaction**

## 🐠 Getting Started
### Prerequisites:
- Install [Rust](https://www.rust-lang.org/tools/install)
- Set up a [Discord Bot Token](https://discord.com/developers/applications)
- Create `.env` file with your bot token:
  ```env
  DISCORD_TOKEN=your_bot_token_here
  ```

### Installation:
```sh
git clone https://github.com/Makufff/Ningyo-Discord-Bot
cd Ningyo-Discord-Bot
cargo run
```

## 🌊 Adding New Commands

1. Create a new command file in `src/commands/` (e.g., `src/commands/greet.rs`):
```rust
use async_trait::async_trait;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use super::command_handler::SlashCommand;

pub struct GreetCommand;

#[async_trait]
impl SlashCommand for GreetCommand {
    fn name(&self) -> &'static str {
        "greet"
    }

    fn description(&self) -> &'static str {
        "Sends a warm greeting!"
    }

    async fn run(&self, ctx: Context, command: ApplicationCommandInteraction) -> Result<(), String> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| 
                    message.content("🌊 Warm greetings from the ocean! 🧜‍♀️")
                )
            })
            .await
            .map_err(|e| format!("Error sending response: {e:?}"))
    }
}
```

2. Add the module in `src/commands/mod.rs`:
```rust
pub mod command_handler;
pub mod ping;
pub mod hello;
pub mod greet; // <--- Add this line
```


3. Register the command in `src/handlers/interaction_handler.rs`:
```rust
impl InteractionHandler {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn SlashCommand + Send + Sync>> = HashMap::new();
        commands.insert("ping".to_string(), Box::new(PingCommand));
        commands.insert("hello".to_string(), Box::new(HelloCommand));
        commands.insert("greet".to_string(), Box::new(GreetCommand)); // <--- Add this line
        Self { commands }
    }
}
```

## 🌌 Available Commands
| Command    | Description |
|------------|------------|
| `/ping`    | Echo from the deep! |
| `/hello`   | Get a mystical greeting |

## 🌊 Project Structure
```
Ningyo-Discord-Bot/
├── Cargo.toml
├── .env
└── src/
    ├── main.rs
    ├── commands/
    │   ├── command_handler.rs
    │   ├── mod.rs
    │   ├── ping.rs
    │   └── hello.rs
    ├── handlers/
    │   ├── mod.rs
    │   └── interaction_handler.rs
    └── services/
        └── mod.rs
```

## 🌌 Contributing
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/NewFeature`)
3. Commit your changes (`git commit -m 'Add NewFeature'`)
4. Push to the branch (`git push origin feature/NewFeature`)
5. Open a Pull Request

## License
MIT License — Free to use & modify!
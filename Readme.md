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
- Create `.env` file with your bot token and application ID:
  ```env
  DISCORD_TOKEN=your_bot_token_here
  APPLICATION_ID=your_application_id_here
  ```

### Installation:
```sh
git clone https://github.com/YourUsername/Ningyo-Discord-Bot.git
cd Ningyo-Discord-Bot
cargo run
```

## 🌊 Adding New Commands

### 1. Simple Command (No Parameters)
Create a new file `src/commands/wave.rs`:
```rust
use async_trait::async_trait;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use super::command_handler::SlashCommand;

pub struct WaveCommand;

#[async_trait]
impl SlashCommand for WaveCommand {
    fn name(&self) -> &'static str {
        "wave"
    }

    fn description(&self) -> &'static str {
        "Wave at the mermaid!"
    }

    async fn run(&self, ctx: Context, command: ApplicationCommandInteraction) -> Result<(), String> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| 
                    message.content("🌊 *waves back with her magical tail* ✨")
                )
            })
            .await
            .map_err(|e| format!("Error sending response: {e:?}"))
    }
}
```

### 2. Command with Parameters
Create a new file `src/commands/fortune.rs`:
```rust
use async_trait::async_trait;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::command::CommandOptionType;
use serenity::builder::CreateApplicationCommand;
use serenity::prelude::*;
use super::command_handler::SlashCommand;

pub struct FortuneCommand;

#[async_trait]
impl SlashCommand for FortuneCommand {
    fn name(&self) -> &'static str {
        "fortune"
    }

    fn description(&self) -> &'static str {
        "Get your ocean fortune!"
    }

    async fn run(&self, ctx: Context, command: ApplicationCommandInteraction) -> Result<(), String> {
        let options = &command.data.options;
        
        // Get name parameter
        let name = match options.first() {
            Some(opt) => match opt.value.as_ref() {
                Some(val) => val.as_str().unwrap_or("stranger"),
                None => "stranger",
            },
            None => "stranger",
        };

        let fortune = format!("🔮 Dearest {}, the ocean whispers that great fortune awaits you! ✨", name);

        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| 
                    message.content(fortune)
                )
            })
            .await
            .map_err(|e| format!("Error sending response: {e:?}"))
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command
            .name(self.name())
            .description(self.description())
            .create_option(|option| {
                option
                    .name("name")
                    .description("Your name")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    }
}
```

### 3. Register New Commands
Add to `src/commands/mod.rs`:
```rust
pub mod command_handler;
pub mod command_registry;
pub mod ping;
pub mod hello;
pub mod tax;
pub mod wave;    // Add new commands
pub mod fortune;
```

Update `src/commands/command_registry.rs`:
```rust
fn register_default_commands(commands: &mut HashMap<String, Box<dyn SlashCommand + Send + Sync>>) {
    commands.insert("ping".to_string(), Box::new(PingCommand));
    commands.insert("hello".to_string(), Box::new(HelloCommand));
    commands.insert("tax".to_string(), Box::new(TaxCommand));
    commands.insert("wave".to_string(), Box::new(WaveCommand));     // Add new commands
    commands.insert("fortune".to_string(), Box::new(FortuneCommand));
}
```

## 🌌 Available Commands
| Command    | Description | Parameters |
|------------|------------|------------|
| `/ping`    | Echo from the deep! | None |
| `/hello`   | Get a mystical greeting | None |
| `/tax`     | Calculate income tax | income: number |

## 🌊 Project Structure
```
Ningyo-Discord-Bot/
├── Cargo.toml
├── .env
└── src/
    ├── main.rs
    ├── commands/
    │   ├── command_handler.rs
    │   ├── command_registry.rs
    │   ├── mod.rs
    │   ├── ping.rs
    │   ├── hello.rs
    │   └── tax.rs
    └── handlers/
        ├── mod.rs
        └── interaction_handler.rs
```

## 🌌 Contributing
1. Fork the repository
2. Create your feature branch (`git checkout -b feature/NewFeature`)
3. Commit your changes (`git commit -m 'Add NewFeature'`)
4. Push to the branch (`git push origin feature/NewFeature`)
5. Open a Pull Request

## License
MIT License — Free to use & modify!
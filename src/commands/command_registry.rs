use std::collections::HashMap;
use super::{
    command_handler::SlashCommand,
    ping::PingCommand,
    hello::HelloCommand,
    tax::TaxCommand,
};

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn SlashCommand + Send + Sync>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        Self::register_default_commands(&mut commands);
        Self { commands }
    }

    fn register_default_commands(commands: &mut HashMap<String, Box<dyn SlashCommand + Send + Sync>>) {
        commands.insert("ping".to_string(), Box::new(PingCommand));
        commands.insert("hello".to_string(), Box::new(HelloCommand));
        commands.insert("tax".to_string(), Box::new(TaxCommand));
    }

    pub fn get_command(&self, name: &str) -> Option<&(dyn SlashCommand + Send + Sync)> {
        self.commands.get(name).map(|cmd| cmd.as_ref())
    }

    pub fn commands(&self) -> impl Iterator<Item = &(dyn SlashCommand + Send + Sync)> {
        self.commands.values().map(|cmd| cmd.as_ref())
    }
} 
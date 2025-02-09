use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use crate::commands::{
    command_handler::CommandExecutor,
    ping::PingCommand,
    hello::HelloCommand
};
use std::collections::HashMap;

pub struct MessageHandler {
    commands: HashMap<String, Box<dyn CommandExecutor + Send + Sync>>,
}

impl MessageHandler {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn CommandExecutor + Send + Sync>> = HashMap::new();
        commands.insert("!ping".to_string(), Box::new(PingCommand) as Box<dyn CommandExecutor + Send + Sync>);
        commands.insert("!hello".to_string(), Box::new(HelloCommand) as Box<dyn CommandExecutor + Send + Sync>);
        Self { commands }
    }
}

#[async_trait]
impl EventHandler for MessageHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(command) = self.commands.get(&msg.content) {
            if let Err(why) = command.execute(&ctx, &msg).await {
                println!("Error executing command: {why}");
            }
        }
    }
}

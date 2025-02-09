use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::application::command::Command;
use serenity::prelude::*;
use std::collections::HashMap;
use crate::commands::{
    command_handler::SlashCommand,
    ping::PingCommand,
    hello::HelloCommand,
};

pub struct InteractionHandler {
    commands: HashMap<String, Box<dyn SlashCommand + Send + Sync>>,
}

impl InteractionHandler {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn SlashCommand + Send + Sync>> = HashMap::new();
        commands.insert("ping".to_string(), Box::new(PingCommand));
        commands.insert("hello".to_string(), Box::new(HelloCommand));
        Self { commands }
    }
}

#[async_trait]
impl EventHandler for InteractionHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Some(cmd) = self.commands.get(&command.data.name) {
                if let Err(why) = cmd.run(ctx, command).await {
                    println!("Error running command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Register global commands
        for command in self.commands.values() {
            Command::create_global_application_command(&ctx.http, |c| {
                c.name(command.name()).description(command.description())
            })
            .await
            .expect("Failed to create global command");
        }
    }
}
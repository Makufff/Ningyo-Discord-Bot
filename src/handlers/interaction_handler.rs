use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::application::command::Command;
use serenity::prelude::*;
use crate::commands::command_registry::CommandRegistry;

pub struct InteractionHandler {
    command_registry: CommandRegistry,
}

impl InteractionHandler {
    pub fn new() -> Self {
        Self {
            command_registry: CommandRegistry::new(),
        }
    }

    async fn handle_slash_command(
        &self,
        ctx: Context,
        command: serenity::model::application::interaction::application_command::ApplicationCommandInteraction,
    ) -> Result<(), String> {
        if let Some(cmd) = self.command_registry.get_command(&command.data.name) {
            cmd.run(ctx, command).await
        } else {
            Err(format!("Unknown command: {}", command.data.name))
        }
    }

    async fn register_commands(&self, ctx: &Context) {
        for command in self.command_registry.commands() {
            if let Err(why) = Command::create_global_application_command(&ctx.http, |c| {
                command.register(c)
            })
            .await {
                println!("Failed to create command '{}': {why}", command.name());
            } else {
                println!("Successfully registered command: {}", command.name());
            }
        }
    }
}

#[async_trait]
impl EventHandler for InteractionHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(why) = self.handle_slash_command(ctx, command).await {
                println!("Error handling command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        self.register_commands(&ctx).await;
    }
}
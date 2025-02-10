use async_trait::async_trait;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use super::command_handler::SlashCommand;
use serenity::builder::CreateApplicationCommand;

pub struct PingCommand;

#[async_trait]
impl SlashCommand for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn description(&self) -> &'static str {
        "Responds with a pong!"
    }

    async fn run(&self, ctx: Context, command: ApplicationCommandInteraction) -> Result<(), String> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| 
                    message.content("🌊 Echo from the deep! 🌊")
                )
            })
            .await
            .map_err(|e| format!("Error sending response: {e:?}"))
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command.name(self.name()).description(self.description())
    }
}
use async_trait::async_trait;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use super::command_handler::SlashCommand;

pub struct HelloCommand;

#[async_trait]
impl SlashCommand for HelloCommand {
    fn name(&self) -> &'static str {
        "hello"
    }

    fn description(&self) -> &'static str {
        "Get a mystical greeting!"
    }

    async fn run(&self, ctx: Context, command: ApplicationCommandInteraction) -> Result<(), String> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|message| 
                    message.content("🧜‍♀️ Greetings from the depths! 🌊")
                )
            })
            .await
            .map_err(|e| format!("Error sending response: {e:?}"))
    }
}
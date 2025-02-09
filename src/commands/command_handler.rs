use async_trait::async_trait;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

#[async_trait]
pub trait SlashCommand {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn run(&self, ctx: Context, command: ApplicationCommandInteraction) -> Result<(), String>;
}
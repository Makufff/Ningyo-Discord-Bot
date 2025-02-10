use async_trait::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

#[async_trait]
pub trait SlashCommand {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn run(&self, ctx: Context, command: ApplicationCommandInteraction) -> Result<(), String>;
    
    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command.name(self.name()).description(self.description())
    }
}
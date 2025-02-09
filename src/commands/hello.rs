use async_trait::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use super::command_handler::CommandExecutor;

pub struct HelloCommand;

#[async_trait]
impl CommandExecutor for HelloCommand {
    async fn execute(&self, ctx: &Context, msg: &Message) -> Result<(), String> {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Hello!").await {
            return Err(format!("Error sending message: {why:?}"));
        }
        Ok(())
    }
}

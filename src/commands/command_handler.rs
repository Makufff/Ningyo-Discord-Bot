use async_trait::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[async_trait]
pub trait CommandExecutor {
    async fn execute(&self, ctx: &Context, msg: &Message) -> Result<(), String>;
}

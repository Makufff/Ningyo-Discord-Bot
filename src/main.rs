use std::env;
use dotenv::dotenv;
use serenity::prelude::*;

mod commands;
mod handlers;
mod services;

use crate::handlers::interaction_handler::InteractionHandler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    
    let intents = GatewayIntents::GUILDS;
    
    let handler = InteractionHandler::new();
    
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
use dotenv::dotenv;
use serenity::prelude::*;
use std::env;
use crate::handlers::interaction_handler::InteractionHandler;

mod commands;
mod handlers;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();
    
    // Initialize bot configuration
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in environment");
    let intents = GatewayIntents::GUILDS;
    
    // Create and start the client
    if let Err(why) = create_and_start_client(&token, intents).await {
        println!("Client error: {why:?}");
    }
}

async fn create_and_start_client(token: &str, intents: GatewayIntents) -> Result<(), String> {
    let handler = InteractionHandler::new();
    
    let mut client = Client::builder(token, intents)
        .event_handler(handler)
        .await
        .map_err(|e| format!("Error creating client: {e:?}"))?;

    client.start().await
        .map_err(|e| format!("Error running client: {e:?}"))
}
use serenity::{
    client,
    prelude::GatewayIntents
};
use compile_bot::{
    EventHandler,
    HandlerFromEnv
};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    FmtSubscriber::builder()
        .init();

    let token = std::env::var("TOKEN").expect("Failed to get token from env. Maybe you not provided one?");

    let intents = GatewayIntents::DIRECT_MESSAGES;
    let mut client = client::Client::builder(token, intents)
        .event_handler(EventHandler::from_env())
        .await.expect("Failed to construct client");

    tracing::info!("Client built!");

    if let Err(e) = client.start().await {
        tracing::error!("Client error: {e}");
    }
}
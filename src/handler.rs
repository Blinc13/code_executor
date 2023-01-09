use std::mem;
use serenity::{
    client::{
        Context,
        EventHandler as SEventHandler
    },
    model::{
        application::{
            command::Command,
            interaction::Interaction
        },
        id::ChannelId,
        gateway::Ready
    },
    async_trait
};
use serenity::builder::CreateInteractionResponse;
use crate::{HandlerFromEnv, commands};
use tracing::{error, info};

#[derive(Debug)]
pub struct EventHandler {
    log_channel: Option<ChannelId>
}

impl EventHandler {
    pub fn new(id: u64) -> Self {
        Self {
            log_channel: Some(ChannelId::from(id))
        }
    }
}

impl HandlerFromEnv for EventHandler {
    fn from_env() -> Self {
        let log_channel = std::env::var("LOG_CHANNEL").ok()
            .and_then(| str | str.parse::<u64>().ok())
            .map(| id | ChannelId::from(id));

        Self {
            log_channel
        }
    }
}

#[async_trait]
impl SEventHandler for EventHandler {
    async fn ready(&self, ctx: Context, data: Ready) {
        info!("Handler authorization complete: user {}, application {}", data.user.name, data.application.id);

        let res = Command::set_global_application_commands(&ctx.http, | builder |
            builder
                .create_application_command(| command | commands::run::setup_command(command))
        ).await;

        match res {
            Err(err) => error!("Failed to register commands!: {err}"),
            Ok(_) => info!("Command initialization successful")
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        info!("Interaction received!: {}", interaction.token());

        if let Interaction::ApplicationCommand(int) = interaction {
            let resp = match int.data.name.as_str() {
                "run" => commands::run::command(&ctx, &int).await,
                _ => CreateInteractionResponse::default()
            };

            match int.create_interaction_response(&ctx.http, | f | {let _ = mem::replace(f, resp); f}).await {
                Ok(_) => info!("Interaction response sended: {}", int.token),
                Err(err) => error!("Failed to send response to interaction {}: {err:?}", int.token)
            }
        }
    }
}
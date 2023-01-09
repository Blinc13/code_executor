use std::mem;
use serenity::{
    client::{
        Context,
        EventHandler as SEventHandler
    },
    model::{
        application::command::{
            Command,
            CommandType,
            CommandOptionType
        },
        id::ChannelId,
        gateway::Ready,
        guild::Integration
    },
    async_trait
};
use crate::HandlerFromEnv;
use tracing::{error, event, info, info_span};

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
                .create_application_command(| command |
                    command
                        .name("run")
                        .description("Run code")
                        .create_option(| op |
                            op
                                .name("lang")
                                .description("Language")
                                .kind(CommandOptionType::String)
                        )
                        .create_option(| op |
                            op
                                .name("code")
                                .description("Code")
                                .kind(CommandOptionType::String)
                        )
                )
        ).await;

        match res {
            Err(err) => error!("Failed to register commands!: {err}"),
            Ok(_) => info!("Command initialization successful")
        }
    }

    async fn integration_create(&self, _ctx: Context, _integration: Integration) {
        info!("Interaction!");
    }
}
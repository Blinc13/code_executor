use std::{mem, sync::Arc};
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
    builder::CreateInteractionResponse,
    async_trait
};
use crate::{HandlerFromEnv, commands, utils};
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
                .create_application_command(| command | commands::system_load::setup_command(command))
        ).await;

        match res {
            Err(err) => error!("Failed to register commands!: {err}"),
            Ok(_) => info!("Command initialization successful")
        }


        if let Some(channel) = self.log_channel {
            info!("Log channel detected! Channel id: {channel}. Enabling logging");

            if let Err(err) = channel.say(&ctx.http, "Bot initialized and ready to use!").await {
                error!("Failed to send init log message! {err:?}");
            }

            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(14400)).await;

                    let result = channel.send_message(&ctx.http, |builder|
                        builder.embed(|builder| utils::build_system_load_embed(builder))
                    ).await;

                    if let Err(err) = result {
                        error!("Failed to send log message! {err:?}");
                    }
                }
            });
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        info!("Interaction received!: {}", interaction.token());

        if let Interaction::ApplicationCommand(int) = interaction {
            let ctx = Arc::new(ctx);

            // Maybe move await outside match?
            let resp = match int.data.name.as_str() {
                "run" => commands::run::command(ctx.clone(), &int).await,
                "system_load" => commands::system_load::command(ctx.clone(), &int).await,
                _ => CreateInteractionResponse::default()
            };

            match int.create_interaction_response(&ctx.http, | f | {let _ = mem::replace(f, resp); f}).await {
                Ok(_) => info!("Interaction response sended: {}", int.token),
                Err(err) => error!("Failed to send response to interaction {}: {err:?}", int.token)
            }
        }
    }
}
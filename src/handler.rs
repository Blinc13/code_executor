use std::{
    mem,
    sync::Arc,
    str::FromStr,
    path::PathBuf
};
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
        id::{ChannelId, UserId},
        gateway::Ready
    },
    builder::CreateInteractionResponse,
    async_trait
};
use crate::{HandlerFromEnv, commands, utils};
use tracing::{error, info};

#[derive(Debug)]
pub struct EventHandler {
    settings: Arc<utils::BotSettings>
}

impl HandlerFromEnv for EventHandler {
    fn from_env() -> Self {
        Self {
            settings: Arc::new(utils::BotSettings {
                temp_file_dir: std::env::var("TEMP_FILE_PATH").ok()
                    .map(| path | PathBuf::from(path))
                    .unwrap_or_else(|| PathBuf::from(".")),
                log_channel: std::env::var("LOG_CHANNEL").ok()
                    .and_then(| str | ChannelId::from_str(&str).ok()),
                owners: std::env::var("OWNERS").ok()
                    .map(| owners |
                        owners
                            .split(",")
                            .filter_map(| owner | UserId::from_str(owner).ok())
                            .collect()
                    ).unwrap_or_else(|| vec![])
            })
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

        ctx.data.write().await.insert::<utils::BotSettings>(self.settings.clone());

        if let Some(channel) = self.settings.log_channel {
            info!("Log channel detected! Channel id: {channel}. Enabling logging");

            tokio::spawn(async move {
                if let Err(err) = channel.say(&ctx.http, "Bot initialized and ready to use!").await {
                    error!("Failed to send init log message! {err:?}");
                }

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
use std::sync::Arc;
use serenity::{
    client::Context,
    builder::{
        CreateApplicationCommand,
        CreateInteractionResponse
    },
    model::prelude::interaction::application_command::ApplicationCommandInteraction
};
use tracing::info;
use crate::utils;

pub async fn command(ctx: Arc<Context>, int: &ApplicationCommandInteraction) -> CreateInteractionResponse {
    let mut response = CreateInteractionResponse::default();

    if ctx.data.read().await.get::<utils::BotSettings>().unwrap().owners.contains(&int.user.id) {
        info!("Owner requested system load!");

        response.interaction_response_data(| builder |
            builder.embed(| builder | utils::build_system_load_embed(builder))
        );
    } else {
        response.interaction_response_data(| builder|
            builder.embed(| builder | builder.title("Permission dined"))
        );
    }

    response
}

pub fn setup_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("system_load")
        .description("Get system load. Only owners can use")
}
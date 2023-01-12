use std::sync::Arc;
use serenity::{
    client::Context,
    builder::{
        CreateApplicationCommand,
        CreateInteractionResponse
    },
    model::prelude::interaction::application_command::ApplicationCommandInteraction
};
use crate::utils;

pub async fn command(_: Arc<Context>, _: &ApplicationCommandInteraction) -> CreateInteractionResponse {
    let mut response = CreateInteractionResponse::default();

    response.interaction_response_data(| builder |
        builder.embed(| builder | utils::build_system_load_embed(builder))
    );

    response
}

pub fn setup_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("system_load")
        .description("Get system load")
}
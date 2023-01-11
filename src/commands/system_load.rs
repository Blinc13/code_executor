use std::sync::Arc;
use serenity::{
    client::Context,
    utils::MessageBuilder,
    builder::{
        CreateApplicationCommand,
        CreateInteractionResponse
    },
    model::prelude::interaction::application_command::ApplicationCommandInteraction
};
use tracing::info;

pub async fn command(_: Arc<Context>, _: &ApplicationCommandInteraction) -> CreateInteractionResponse {
    info!("Collecting load info");
    let load = sys_info::loadavg().unwrap();
    let mem = sys_info::mem_info().unwrap();

    let (cores_count, freq) = (sys_info::cpu_num().unwrap(), sys_info::cpu_speed().unwrap());

    let mut response = CreateInteractionResponse::default();

    response.interaction_response_data(| builder |
        builder
            .embed(| builder |
                builder
                    .field(
                        "CPU info",
                        MessageBuilder::new()
                            .push("Total cores count: ")
                            .push_bold_line(cores_count.to_string())
                            .push("Frequency: ")
                            .push_bold_line(freq.to_string())
                            .push("System load: ")
                            .push_bold_line(load.five.to_string())
                            .build(),
                        false
                    ).field("Memory usage",
                            MessageBuilder::new()
                                .push("Total memory: ")
                                .push_bold_line(mem.total.to_string())
                                .push("Available memory: ")
                                .push_bold_line(mem.avail.to_string())
                                .push("Usage %: ")
                                .push_bold_line((mem.free / (mem.total/100)).to_string())
                                .build(),
                            false
                )
            )
    );

    response
}

pub fn setup_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("system_load")
        .description("Get system load")
}
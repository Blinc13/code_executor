use serenity::{
    client::Context,
    builder::{
        CreateApplicationCommand,
        CreateInteractionResponse
    },
    model::{
        application::command::CommandOptionType,
        prelude::interaction::application_command::ApplicationCommandInteraction
    },
    utils::MessageBuilder
};
use std::str::FromStr;
use std::sync::Arc;
use tracing::info;
use crate::executor::{Executor, Error, Language};
use super::generate_options_map;

pub async fn command(ctx: Arc<Context>, int: &ApplicationCommandInteraction) -> CreateInteractionResponse {
    let options = generate_options_map(&int.data.options);

    let code = options.get("code").unwrap().unwrap().as_str().expect("Failed to get code").to_owned();
    let lang = Language::from_str(options.get("lang").unwrap().unwrap().as_str().unwrap()).expect("Failed to parse lang");

    let (channel, user) = (int.channel_id, int.user.id);

    tokio::spawn(async move {
        let executor = Executor::new(lang, code, ".".parse().unwrap(), channel, user);

        let _ = match executor.compile_and_run(tokio::time::Duration::from_secs(10)).await {
            Ok((compile_out, exec_out)) => {
                info!("Code executed successful");

                channel.say(&ctx.http, format!("{}{}", compile_out, exec_out))
            }
            Err((compile_out, err)) => {
                info!("Failed to run code: {err:?}");

                match err {
                    Error::BuildError => channel.say(&ctx.http, format!("Build error!\n{}", compile_out.unwrap())),
                    Error::ExecError => channel.say(&ctx.http, format!("Execution error!\nCompile output:\n{}", compile_out.unwrap())),
                    Error::InvokeError => channel.say(&ctx.http, format!("Failed to call compiler!")),
                    Error::FsError => channel.say(&ctx.http, format!("File system error on server!")),
                    Error::Unsupported => channel.say(&ctx.http, format!("Currently unsupported")),
                    Error::TimeOut => channel.say(&ctx.http, format!("Time out. Write normal code and try again"))
                }
            }
        }.await;
    });


    let mut resp = CreateInteractionResponse::default();

    resp.interaction_response_data(| builder |
        builder
            .embed(| em |
                em.title("Compilling")
                    .description(MessageBuilder::new()
                        .push("Compilling ")
                        .user(int.user.id)
                        .push(" code")
                        .build()
                    )
        )
    );

    resp
}

pub fn setup_command(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("run")
        .description("Run code")
        .create_option(| op |
            op
                .name("lang")
                .description("Language")
                .required(true)
                .kind(CommandOptionType::String)
                .add_string_choice("Rust", "rust")
                .add_string_choice("C++", "cpp")
                .add_string_choice("C", "c")
        )
        .create_option(| op |
            op
                .name("code")
                .description("Code")
                .required(true)
                .kind(CommandOptionType::String)
        )
}
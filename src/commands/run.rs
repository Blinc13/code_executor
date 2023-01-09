use std::fs;
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
use crate::compiler_interface::{Compiler, Language};
use super::generate_options_map;

pub async fn command(ctx: Arc<Context>, int: &ApplicationCommandInteraction) -> CreateInteractionResponse {
    let options = generate_options_map(&int.data.options);

    let code = options.get("code").unwrap().unwrap().as_str().unwrap().to_owned();
    let lang = Language::from_str(
        options.get("lang").unwrap().unwrap().as_str().unwrap()
    ).unwrap();

    let (channel, user) = (int.channel_id, int.user.id);

    tokio::spawn(async move {
        let compiler = Compiler::new(lang, code, channel, user);
        let executable = compiler.compile().await.unwrap();

        println!("{}", executable.0.display());

        let exec_out = tokio::process::Command::new(executable.0).output().await;

        channel.say(&ctx.http, format!("{}{}", String::from_utf8(executable.1).unwrap(), String::from_utf8(exec_out.unwrap().stdout).unwrap())).await;
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
        )
        .create_option(| op |
            op
                .name("code")
                .description("Code")
                .required(true)
                .kind(CommandOptionType::String)
        )
}
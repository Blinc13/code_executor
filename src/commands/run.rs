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
use super::generate_options_map;

pub async fn command<'a>(_ctx: &Context, int: &'a ApplicationCommandInteraction) -> CreateInteractionResponse<'a> {
    let options = generate_options_map(&int.data.options);

    let lang = options.get("lang").unwrap().unwrap().as_str().unwrap();
    let _code = options.get("code").unwrap().unwrap().as_str().unwrap();


    let mut resp = CreateInteractionResponse::default();

    resp.interaction_response_data(| builder |
        builder
            .embed(| em |
                em.title("Compilling")
                    .description(MessageBuilder::new()
                        .push("Compilling ")
                        .user(int.user.id)
                        .push(" code on ")
                        .push_bold(lang)
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
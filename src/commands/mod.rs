pub mod run;
pub mod system_load;

use std::collections::HashMap;
use serenity::{
    json::Value,
    model::application::interaction::application_command::CommandDataOption
};

fn generate_options_map(options: &Vec<CommandDataOption>) -> HashMap<&str, Option<&Value>> {
    options.iter()
        .map(move | option | (option.name.as_str(), option.value.as_ref()))
        .collect()
}
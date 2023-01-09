pub mod run;

use std::collections::HashMap;
use serenity::json::Value;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

fn generate_options_map(options: &Vec<CommandDataOption>) -> HashMap<&str, Option<&Value>> {
    options.iter()
        .map(move | option | (option.name.as_str(), option.value.as_ref()))
        .collect()
}
use serenity::{
    builder::CreateEmbed,
    utils::MessageBuilder
};
use tracing::info;

pub fn build_system_load_embed(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    info!("Collecting load info");

    let load = sys_info::loadavg().unwrap();
    let mem = sys_info::mem_info().unwrap();

    let (cores_count, freq) = (sys_info::cpu_num().unwrap(), sys_info::cpu_speed().unwrap());

    embed
        .field(
            "CPU info",
            MessageBuilder::new()
                .push("Total cores count: ")
                .push_bold_line(cores_count)
                .push("Frequency: ")
                .push_bold_line(freq)
                .push("System load: ")
                .push_bold_line(load.fifteen)
                .build(),
            true,
        )
        .field(
            "Memory usage",
            MessageBuilder::new()
                .push("Total memory: ")
                .push_bold_line(mem.total)
                .push("Available memory: ")
                .push_bold_line(mem.avail)
                .push("Usage %: ")
                .push_bold_line(mem.free / (mem.total / 100))
                .build(),
            true,
        )
}
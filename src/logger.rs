use colored::*;
use log::LevelFilter;

pub fn init() {
    env_logger::Builder::new()
        .format(|buf, record| {
            use std::io::Write;
            let level = record.level();
            let level_color = match level {
                log::Level::Error => "red",
                log::Level::Warn => "yellow",
                log::Level::Info => "green",
                log::Level::Debug => "blue",
                log::Level::Trace => "purple",
            };

            // Info level is for user messages, so don't print level there
            if level == log::Level::Info {
                writeln!(buf, "{}", record.args())
            } else {
                writeln!(
                    buf,
                    "{}: {}",
                    level.to_string().color(level_color),
                    record.args()
                )
            }
        })
        .filter(None, LevelFilter::Info)
        .init()
}

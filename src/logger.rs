use log::{debug, error, info, warn};
use std::error::Error;
pub fn setup_logger() -> Result<(), Box<dyn Error>> {
    use chrono::Local;
    use fern::colors::{Color, ColoredLevelConfig};
    use std::io;

    // Configure colors for log levels
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Magenta);

    let colors_level = colors_line.clone();

    fern::Dispatch::new()
        // Set the default log level
        .level(log::LevelFilter::Debug)
        // Perform logging for all modules
        .level_for("reqwest", log::LevelFilter::Info)
        // Output to stdout
        .chain(io::stdout())
        // Apply formatting
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                // Timestamp
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                // Log level with color
                colors_level.color(record.level()),
                // Log message
                message
            ))
        })
        .apply()?;
    Ok(())
}

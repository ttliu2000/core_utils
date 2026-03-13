use std::sync::Once;

use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

static INIT: Once = Once::new();

const PATTERN: &str = "{d(%Y-%m-%d %H:%M:%S)} [{l}] {t} - {m}{n}";

pub struct LoggerConfig {
    pub log_file: String,
    pub console_level: log::LevelFilter,
    pub file_level: log::LevelFilter,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_file: "logs/app.log".to_string(),
            console_level: log::LevelFilter::Debug,
            file_level: log::LevelFilter::Warn,
        }
    }
}

fn init_logger_internal(cfg: LoggerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let _console = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(PATTERN)))
        .build(&cfg.log_file)?;

    let config = Config::builder()
        // .appender(Appender::builder().build("console", Box::new(console)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(cfg.file_level)))
                .build("file", Box::new(file)),
        )
        .build(
            Root::builder()
                //.appender("console")
                .appender("file")
                .build(cfg.console_level),
        )?;

    log4rs::init_config(config)?;
    Ok(())
}

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = LoggerConfig::default();
    let mut result = Ok(());
    INIT.call_once(|| {
        result = init_logger_internal(cfg);
    });

    result
}
use env_logger::{Builder, Env};
use std::io::Write;
use chrono::Local;
use colored::*;
use std::path::Path;
use std::thread;
pub use log::{Level, LevelFilter};

/// Logger builder
/// 
/// # Parameters
/// - `level`: Log level, such as info, error, warn, debug, trace (default is `info`)
/// - `only_project_logs`: Whether to output only project logs, not external module logs (default is `false`)
/// - `path_depth`: Path display depth (default is `1`)
/// - `time_format`: Time format such as `%Y-%m-%d %H:%M:%S` (default is `%Y-%m-%d %H:%M:%S`)
/// - `preset`: Logger preset such as `FULL`, `THREAD`, `SIMPLE` (default is `FULL`)
pub struct LoggerBuilder {
    pub level: String,
    pub only_project_logs: bool,
    pub path_depth: usize,
    pub time_format: String,
    pub preset: LoggerPreset,
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        LoggerBuilder {
            level: "info".to_string(),
            only_project_logs: false,
            path_depth: 0,
            time_format: "%Y-%m-%d %H:%M:%S".to_string(),
            preset: LoggerPreset::FULL,
        }
    }
}


pub enum LoggerPreset {
    FULL,
    THREAD,
    SIMPLE,
}

/// Initialize the logger
/// 
/// # Parameters
/// - `logger_builder`: Logger builder
pub fn init_logger(logger_builder: LoggerBuilder) {
    let level = logger_builder.level.to_lowercase();
    let only_project_logs = logger_builder.only_project_logs;
    let path_depth = logger_builder.path_depth;
    let time_format = logger_builder.time_format;
    let preset = logger_builder.preset;

    let project_name = env!("CARGO_PKG_NAME");

    let env = Env::default().filter_or("RUST_LOG", level.clone());
    let mut builder = Builder::from_env(env);

    builder.format(move |buf, record| {
        let file_path = record.file().unwrap_or("unknown");
        let project_relative_path = get_project_relative_path(file_path, path_depth);
        let line = record.line().unwrap_or(0);

        let level = match record.level() {
            log::Level::Error => "ERROR".red().bold(),
            log::Level::Warn => "WARN ".yellow().bold(),
            log::Level::Info => "INFO ".green().bold(),
            log::Level::Debug => "DEBUG".blue().bold(),
            log::Level::Trace => "TRACE".magenta().bold(),
        };

        let thread_name = thread::current().name().unwrap_or("unknown").to_string();
        let thread_colored = if thread_name == "main" {
            thread_name.bright_green()
        } else {
            thread_name.bright_blue()
        };

        let project = if record.target().starts_with(project_name) {
            format!("{}",
                project_relative_path.yellow())
        }else {
            format!("[{}] {}",
                record.target().yellow(), 
                project_relative_path.yellow())
        };

        let timestamp = Local::now().format(&time_format).to_string().cyan();

        let log_message = match preset {
            LoggerPreset::FULL => {
                format!(
                    "{} {} [{}] [{}:{}] {}",
                    timestamp,
                    level,
                    thread_colored,
                    project,
                    line.to_string().yellow(),
                    record.args()
                )
            }
            LoggerPreset::THREAD => {
                format!(
                    "{} {} [{}] {}",
                    timestamp,
                    level,
                    thread_colored,
                    record.args()
                )
            }
            LoggerPreset::SIMPLE => {
                format!(
                    "[ {} {}] {}",
                    timestamp,
                    level,
                    record.args()
                )
            }
        };

        // Write the log message, but do not add a newline
        write!(buf, "{}", log_message)
    });

    // Parse the log level and handle any errors
    let parsed_level = level.parse::<log::LevelFilter>().unwrap_or_else(|_| {
        eprintln!(
            "Invalid log level '{}', using default level Info",
            level
        );
        log::LevelFilter::Info
    });

    if only_project_logs {
        builder
            .filter(None, log::LevelFilter::Off)
            .filter(Some(project_name), parsed_level);
    } else {
        builder.filter(None, parsed_level);
    }

    builder.init();
}

/// Get the project relative path
fn get_project_relative_path(file_path: &str, depth: usize) -> String {
    let path = Path::new(file_path);
    let components: Vec<&str> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();

    // Find the index of the "src" directory
    let src_index = components.iter().position(|&c| c == "src");

    let relevant_components = if let Some(index) = src_index {
        components.iter()
            .skip(index)
            .cloned()
            .collect::<Vec<&str>>()
    } else {
        components.clone()
    };

    let total = relevant_components.len();
    if depth == 0 || depth >= total {
        // If the depth is 0 or greater than or equal to the total number of components, return the full path
        relevant_components.join("/")
    } else {
        // Keep the last `depth` components
        relevant_components[(total - depth)..].join("/")
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => ({
        log::info!(target: module_path!(), "{}\n", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => ({
        log::error!(target: module_path!(), "{}\n", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => ({
        log::warn!(target: module_path!(), "{}\n", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => ({
        log::debug!(target: module_path!(), "{}\n", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => ({
        log::trace!(target: module_path!(), "{}\n", format_args!($($arg)*));
    })
}


#[macro_export]
macro_rules! _log_info {
    ($($arg:tt)*) => ({
        log::info!(target: module_path!(), "{}", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! _log_error {
    ($($arg:tt)*) => ({
        log::error!(target: module_path!(), "{}", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! _log_warn {
    ($($arg:tt)*) => ({
        log::warn!(target: module_path!(), "{}", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! _log_debug {
    ($($arg:tt)*) => ({
        log::debug!(target: module_path!(), "{}", format_args!($($arg)*));
    })
}

#[macro_export]
macro_rules! _log_trace {
    ($($arg:tt)*) => ({
        log::trace!(target: module_path!(), "{}", format_args!($($arg)*));
    })
}

pub use log_info as info;
pub use log_error as error;
pub use log_warn as warn;
pub use log_debug as debug;
pub use log_trace as trace;

pub use _log_info as _info;
pub use _log_error as _error;
pub use _log_warn as _warn;
pub use _log_debug as _debug;
pub use _log_trace as _trace;

pub use log::{log, logger};
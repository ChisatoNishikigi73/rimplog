# Rimplog

A simple, colorful, and easy-to-use logging library for Rust

```tips
Rimplog means Rainbow Simple Logging
```

## Getting Started with rimplog

rimplog is a simple, colorful, and easy-to-use logging library for Rust. This guide will help you get started with rimplog in your Rust projects.

## Dependencies

To use rimplog in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rimplog = "0.0.3"
```

## Building the Logger

rimplog uses a builder pattern to configure the logger. Here's how to create a custom logger:

```rust
use rimplog::{LoggerBuilder, LoggerPreset};

let logger = LoggerBuilder {
    level: "debug".to_string(),
    only_project_logs: true,
    path_depth: 1,
    time_format: "%Y-%m-%d %H:%M:%S".to_string(),
    preset: LoggerPreset::FULL,
};
```

## Customizing the Logger

You can customize the following options:

- `level`: Log level (`error`, `warn`, `info`, `debug`, `trace`)
- `only_project_logs`: Whether to show only project logs (`true`/`false`)
- `path_depth`: Depth of file path to display
- `time_format`: Custom time format string
- `preset`: Logger preset (`FULL`, `THREAD`, `SIMPLE`)

## Initializing the Logger

Once you've built your logger, initialize it at the start of your program:

```rust
use rimplog::init_logger;

fn main() {
    init_logger(logger);
}
```

If you're happy with the default settings, you can simply use:

```rust
init_logger(LoggerBuilder::default());
```

## Using the Logger

After initialization, you can use the logging macros throughout your code:

```rust
use rimplog::{info, error, warn, debug, trace};

fn some_function() {
    info!("This is an info log");
    error!("This is an error log");
    warn!("This is a warning log");
    debug!("This is a debug log");
    trace!("This is a trace log");

    loggger::info!("This is an info log")
}
```

For logs without automatic newlines, use the underscore versions:

```rust
use rimplog::{_info};

fn another_function() {
    _info!("installing: ");
    println!("success");
}
```

That's it! You're now ready to use rimplog in your Rust projects. Enjoy colorful and customizable logging!

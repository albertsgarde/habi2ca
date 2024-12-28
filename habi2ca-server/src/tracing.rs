use std::{io, path::Path};

use anyhow::{Context, Result};
use tracing::{subscriber, Level};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{filter, fmt, layer::SubscriberExt, Layer};

pub fn setup_tracing(log_dir: impl AsRef<Path>) -> Result<WorkerGuard> {
    let log_dir = log_dir.as_ref();
    let (log_file, guard) = tracing_appender::non_blocking(rolling::daily(log_dir, "debug"));
    let log_layer = fmt::Layer::new()
        .with_writer(log_file)
        .with_ansi(false)
        .with_filter(
            filter::Targets::new()
                .with_default(Level::INFO)
                .with_targets([("habi2ca", Level::DEBUG)]),
        );

    let stdout_layer = fmt::Layer::new().with_writer(io::stdout).with_filter(
        filter::Targets::new()
            .with_default(Level::WARN)
            .with_targets([("habi2ca", Level::INFO)]),
    );

    let subscriber = tracing_subscriber::registry()
        .with(log_layer)
        .with(stdout_layer);

    subscriber::set_global_default(subscriber)
        .context("Failed to set global default tracing subscriber")?;

    Ok(guard)
}

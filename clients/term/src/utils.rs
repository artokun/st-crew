use std::{env, path::PathBuf};

use color_eyre::eyre::Result;
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use tracing::error;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    self, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
};

use crate::app::tui::Tui;

pub static NAME: &str = env!("CARGO_PKG_NAME");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub static REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
pub static HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

static DATA_FOLDER: Lazy<Option<PathBuf>> = Lazy::new(|| {
    std::env::var(format!("{}_DATA", NAME.to_uppercase()))
        .ok()
        .map(PathBuf::from)
});

static CONFIG_FOLDER: Lazy<Option<PathBuf>> = Lazy::new(|| {
    std::env::var(format!("{}_CONFIG", NAME.to_uppercase()))
        .ok()
        .map(PathBuf::from)
});

static PROJECT_DIRECTORY: Lazy<Option<ProjectDirs>> =
    Lazy::new(|| ProjectDirs::from("com", "codingforcookies", env!("CARGO_PKG_NAME")));

static LOG_FILE: Lazy<String> = Lazy::new(|| format!("{}.log", env!("CARGO_PKG_NAME")));

pub fn get_data_dir() -> PathBuf {
    let directory = if let Some(s) = DATA_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = PROJECT_DIRECTORY.as_ref() {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".data")
    };

    directory
}

pub fn get_config_dir() -> PathBuf {
    let directory = if let Some(s) = CONFIG_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = PROJECT_DIRECTORY.as_ref() {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".config")
    };

    directory
}

pub fn initialize_panic_handler() -> Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .capture_span_trace_by_default(false)
        .display_location_section(false)
        .display_env_section(false)
        .into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |panic_info| {
        if let Ok(mut t) = Tui::new() {
            if let Err(r) = t.exit() {
                error!("Unable to exit Terminal: {:?}", r);
            }
        }

        #[cfg(not(debug_assertions))]
        {
            use human_panic::{handle_dump, print_msg, Metadata};
            let meta = Metadata {
                version: env!("CARGO_PKG_VERSION").into(),
                name: env!("CARGO_PKG_NAME").into(),
                authors: env!("CARGO_PKG_AUTHORS").replace(':', ", ").into(),
                homepage: env!("CARGO_PKG_HOMEPAGE").into(),
            };

            let file_path = handle_dump(&meta, panic_info);

            // prints human-panic message
            print_msg(file_path, &meta)
                .expect("human-panic: printing error message to console failed");

            eprintln!("{}", panic_hook.panic_report(panic_info)); // prints color-eyre stack trace to stderr
        }

        let msg = format!("{}", panic_hook.panic_report(panic_info));

        tracing::error!("Error: {}", strip_ansi_escapes::strip_str(msg));

        std::process::exit(1);
    }));

    Ok(())
}

pub fn initialize_logging() -> Result<()> {
    let directory = get_data_dir();

    std::fs::create_dir_all(&directory)?;

    let log_path = directory.join(&*LOG_FILE);
    let log_file = std::fs::File::create(log_path)?;

    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME"))),
    );

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();

    Ok(())
}

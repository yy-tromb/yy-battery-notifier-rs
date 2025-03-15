use clap::Parser;
use colored::Colorize;

mod battery;
mod cli;
mod common;

#[derive(clap::Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    about,
    version,
    about,
    author = "yy-tromb",
    arg_required_else_help = true,
    long_about = None
)]
struct AppArgs {
    #[arg(
        short = 's',
        long = "settings",
        value_name = "path to settings.toml",
        default_value_t = String::from(".\\settings.toml")
    )]
    settings_toml_path: String,
}
fn main() -> anyhow::Result<()> {
    let app = AppArgs::parse();
    let settings = std::fs::read_to_string(&app.settings_toml_path).map_err(|e| {
        eprintln!(
            "{}",
            format!("Failed to read file '{}'.", &app.settings_toml_path).red()
        );
        e
    })?;
    let settings: crate::common::Settings = toml::from_str(&settings).map_err(|e| {
        eprintln!(
            "{}",
            format!("Failed to interpret '{}' as TOML.", &app.settings_toml_path).red()
        );
        e
    })?;
    crate::cli::Cli::new(settings).start()
}

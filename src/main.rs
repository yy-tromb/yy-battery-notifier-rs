use clap::Parser;
use colored::Colorize;

mod battery;
mod cli;
mod common;
mod notify;

#[derive(clap::Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
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
        default_value_t = String::from(".\\default_settings.toml")
    )]
    toml_settings_path: String,
}
fn main() -> anyhow::Result<()> {
    let app = AppArgs::parse();
    let toml_settings = std::fs::read_to_string(&app.toml_settings_path).map_err(|e| {
        eprintln!(
            "{}",
            format!("Failed to read file '{}'.", &app.toml_settings_path).red()
        );
        e
    })?;
    let toml_settings: crate::common::TOMLSettings =
        toml::from_str(&toml_settings).map_err(|e| {
            eprintln!(
                "{}",
                format!("Failed to interpret '{}' as TOML.", &app.toml_settings_path).red()
            );
            e
        })?;
    crate::cli::Cli::new(toml_settings)?.start()
}

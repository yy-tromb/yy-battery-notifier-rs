use clap::Parser;
use colored::Colorize;

mod battery;
mod cli;
mod common;
mod notify;

#[derive(clap::Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version,
    about = format!("\x1b[1m\
{} version {}\x1b[m
{}",env!("CARGO_PKG_NAME"),env!("CARGO_PKG_VERSION"),env!("CARGO_PKG_DESCRIPTION")),
    author = "yy-tromb",
    long_about = None,
    disable_help_subcommand(true)
)]
struct AppArgs {
    #[arg(
        short = 's',
        long = "settings",
        value_name = "path to settings.toml",
        default_value_t = String::from(".\\settings.toml")
    )]
    toml_settings_path: String,
    #[arg(
        short = 'd',
        long = "default_settings",
        value_name = "use default_settings.toml",
        required = false
    )]
    default_settings: bool,
}
fn main() -> anyhow::Result<()> {
    let app_args = AppArgs::parse();
    let toml_settings_path = if app_args.default_settings {
        std::env::current_exe()
            .inspect_err(|_e| eprintln!("{}", "Failed to get current execution file path.".red()))?
            .with_file_name("default_settings.toml")
    } else {
        std::path::PathBuf::from(&app_args.toml_settings_path)
    };
    dbg!(&toml_settings_path);
    let toml_settings = std::fs::read_to_string(&toml_settings_path).inspect_err(|_e| {
        eprintln!(
            "{}",
            format!("Failed to read file '{}'.", &toml_settings_path.display()).red()
        );
    })?;
    let toml_settings: crate::common::TOMLSettings =
        toml::from_str(&toml_settings).inspect_err(|_e| {
            eprintln!(
                "{}",
                format!(
                    "Failed to interpret '{}' as TOML.",
                    &app_args.toml_settings_path
                )
                .red()
            );
        })?;
    crate::cli::Cli::new(toml_settings)?.start()
}

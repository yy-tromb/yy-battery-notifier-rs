#![cfg_attr(feature = "gui", windows_subsystem = "windows")]

use clap::Parser;
use colored::Colorize;

mod battery;
mod cli;
mod common;
mod notify;
mod registry;
mod aumid;
mod startup;

use registry::{CURRENT_USER, LOCAL_MACHINE};

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

    #[arg(
        long = "msgbox",
        value_name = "enable message box",
        required = false,
        global = true
    )]
    msgbox: bool,

    #[command(subcommand)]
    subcommands: Option<SubCommand>,
}
#[derive(Debug, clap::Subcommand)]
enum SubCommand {
    Registry {
        #[command(subcommand)]
        subcommands: RegistrySubCommand,
    },
    Startup {
        #[command(subcommand)]
        subcommands: StartupSubCommand,
    },
}

#[derive(Debug, clap::Subcommand)]
enum RegistrySubCommand {
    Register,
    Delete,
}

#[derive(Debug, clap::Subcommand)]
enum StartupSubCommand {
    Register {
        #[arg(short = 's', long = "settings", value_name = "path to settings.toml")]
        toml_settings_path: Option<String>,
        #[arg(
            short = 'i',
            long = "input",
            value_name = "Whether you input path of settings.toml in terminal or not."
        )]
        input_mode: bool,
        #[arg(
            short = 'd',
            long = "default_settings",
            value_name = "use default_settings.toml",
            required = false
        )]
        default_settings: bool,
    },
    Delete,
}

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "gui")]
    let mut attach_console_error = windows::core::Result::Ok(());
    //Try enable console if feature is gui
    #[cfg(feature = "gui")]
    {
        use windows::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole, FreeConsole};
        use windows::Win32::UI::WindowsAndMessaging::{
            MB_ICONWARNING, MESSAGEBOX_RESULT, MessageBoxW,
        };
        use windows::core::w;
        match unsafe { FreeConsole().and_then(|_| AttachConsole(ATTACH_PARENT_PROCESS)) } {
            Ok(_) => println!("Enabled console"),
            Err(e) => {
                attach_console_error = windows::core::Result::Err(e);
            }
        }
    }
    let app_args = AppArgs::parse();
    #[cfg(feature = "gui")]
    if let windows::core::Result::Err(_e) = attach_console_error {
        use windows::Win32::UI::WindowsAndMessaging::{
            MB_ICONWARNING, MESSAGEBOX_RESULT, MessageBoxW,
        };
        use windows::core::w;
        if !app_args.msgbox
            && unsafe {
                MessageBoxW(
                    None,
                    w!("You execute this app not on terminal\n\
                                But there are not --msgbox flag"),
                    w!("yy-battery-notifier-rs"),
                    MB_ICONWARNING,
                )
            } == MESSAGEBOX_RESULT(0)
        {
            let error = anyhow::Error::from(windows::core::Error::from_thread());
            eprintln!("{}", error.to_string().red());
            crate::common::msgbox(&error).inspect_err(|e| eprintln!("{}", e.to_string().red()))?;
        }
    }
    if app_args.subcommands.is_some() {
        let subcommand = match app_args.subcommands.unwrap() {
            SubCommand::Registry { subcommands } => match subcommands {
                RegistrySubCommand::Register => crate::aumid::register_and_check_aumid(&CURRENT_USER),
                RegistrySubCommand::Delete => crate::aumid::delete_and_check_aumid(&CURRENT_USER),
            },
            SubCommand::Startup { subcommands } => match subcommands {
                StartupSubCommand::Register {
                    toml_settings_path,
                    input_mode,
                    default_settings,
                } => crate::startup::register_cli(toml_settings_path, input_mode, default_settings),
                StartupSubCommand::Delete => crate::startup::delete_and_check_startup(),
            },
        };
        if app_args.msgbox {
            return subcommand.or_else(|e| crate::common::msgbox(&e));
        } else {
            return subcommand;
        }
    }
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
    if app_args.msgbox {
        crate::cli::Cli::new(crate::common::Settings::try_from(toml_settings)?)?
            .run()
            .or_else(|e| crate::common::msgbox(&e))
    } else {
        crate::cli::Cli::new(crate::common::Settings::try_from(toml_settings)?)?.run()
    }
}

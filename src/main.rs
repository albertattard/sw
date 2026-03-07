use clap::{CommandFactory, Parser, Subcommand};
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "sw",
    about = "Sociable Weaver (SW)",
    disable_version_flag = true,
    disable_help_subcommand = true,
    after_help = "Still weaving the nest. Features are hatching soon."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Show top-level help.
    Help,
}

fn print_top_level_help() -> std::io::Result<()> {
    let mut cmd = Cli::command();
    cmd.print_help()?;
    println!();
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        None | Some(Commands::Help) => match print_top_level_help() {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("Failed to print help: {err}");
                ExitCode::from(1)
            }
        }
    }
}

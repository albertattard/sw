use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "sw",
    about = "Sociable Weaver (SW)",
    disable_version_flag = true,
    disable_help_subcommand = true,
    after_help = "Still weaving the nest. Features are hatching soon."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Render a runbook to output.
    Run(RunArgs),
    /// Show top-level help.
    Help,
    /// Validate a runbook file.
    Validate(ValidateArgs),
}

#[derive(Debug, clap::Args)]
pub struct RunArgs {
    /// Path to the input runbook file.
    #[arg(long)]
    pub input_file: Option<PathBuf>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = RunOutputFormat::Markdown)]
    pub output_format: RunOutputFormat,

    /// Path to the generated output file.
    #[arg(long)]
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
pub struct ValidateArgs {
    /// Path to the input runbook file.
    #[arg(long)]
    pub input_file: Option<PathBuf>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    pub output_format: OutputFormat,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum RunOutputFormat {
    Markdown,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Human,
    Json,
}

pub fn print_top_level_help() -> std::io::Result<()> {
    let mut cmd = Cli::command();
    cmd.print_help()?;
    println!();
    Ok(())
}

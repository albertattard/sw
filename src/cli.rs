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
    /// Print entry progress while running long workflows.
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Print diagnostic details for command rewrites and captures.
    #[arg(long, global = true)]
    pub debug: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check runbook prerequisites.
    Check(CheckArgs),
    /// Print a JSON example for a runbook topic.
    Example(ExampleArgs),
    /// Render a runbook to output.
    Run(RunArgs),
    /// Show help for the CLI or a specific subcommand.
    Help(HelpArgs),
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
pub struct CheckArgs {
    /// Path to the input runbook file.
    #[arg(long)]
    pub input_file: Option<PathBuf>,
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

#[derive(Debug, clap::Args)]
pub struct ExampleArgs {
    /// Example topic such as `Command`, `DisplayFile`, or `rewrite.keep_between`.
    pub topic: String,
}

#[derive(Debug, clap::Args)]
pub struct HelpArgs {
    /// Print help for all known subcommands.
    #[arg(long)]
    pub all: bool,

    /// Help topic such as `run` or `validate`.
    pub topic: Option<String>,
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

pub fn print_help_for_topic(topic: &str) -> Result<(), String> {
    let mut cmd = Cli::command();
    let Some(subcommand) = cmd.find_subcommand_mut(topic) else {
        return Err(format!("Unknown help topic: {topic}"));
    };

    subcommand
        .print_help()
        .map_err(|err| format!("Failed to print help for `{topic}`: {err}"))?;
    println!();
    Ok(())
}

pub fn print_all_help() -> Result<(), String> {
    print_top_level_help().map_err(|err| format!("Failed to print help: {err}"))?;

    let topic_names = command_topic_names();
    for topic in topic_names {
        println!();
        print_help_for_topic(topic)?;
    }

    Ok(())
}

fn command_topic_names() -> Vec<&'static str> {
    vec!["check", "example", "run", "help", "validate"]
}

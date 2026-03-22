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
    /// Explain a feature contract or discovery path.
    Explain(ExplainArgs),
    /// Render a runbook to output.
    Run(RunArgs),
    /// Show help for the CLI or a specific subcommand.
    Help(HelpArgs),
    /// Validate a runbook file.
    Validate(ValidateArgs),
}

#[derive(Debug, clap::Args)]
#[command(
    after_help = "Runbook-authored output fields such as `trim_empty_lines` are configured in the runbook, not as CLI flags.\nUse `sw example Command` for a current JSON snippet and `sw explain run` for behavior and defaults."
)]
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
#[command(
    after_help = "The `Command` example includes current nested output fields such as `trim_empty_lines`, along with rewrite, capture, and cleanup examples.\nUse `sw example DisplayFile` when you need the Java `collapse_method_body` transform for collapsing method bodies."
)]
pub struct ExampleArgs {
    /// Example topic such as `Command`, `DisplayFile`, or `rewrite.keep_between`.
    pub topic: String,
}

#[derive(Debug, clap::Args)]
pub struct ExplainArgs {
    /// Print explanations for all supported topics.
    #[arg(long)]
    pub all: bool,

    /// Explain output format.
    #[arg(long, value_enum, default_value_t = ExplainOutputFormat::Text)]
    pub output_format: ExplainOutputFormat,

    /// Write skill output to the default Codex skill path or to the provided path.
    #[arg(long, value_name = "PATH", num_args = 0..=1, require_equals = true)]
    pub output_file: Option<Option<PathBuf>>,

    /// Overwrite the output file if it already exists.
    #[arg(long)]
    pub force: bool,

    /// Explain topic such as `run`, `check`, or `example`.
    pub topic: Option<String>,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum ExplainOutputFormat {
    Text,
    Skill,
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
    vec!["check", "example", "explain", "run", "help", "validate"]
}

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "sw",
    about = "Sociable Weaver (SW)",
    version = env!("SW_CLI_VERSION"),
    disable_help_subcommand = true,
    after_help = "Still weaving the nest. Features are hatching soon."
)]
pub struct Cli {
    /// Print entry progress while running long workflows.
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Verbose progress mode. `auto` uses live redraws on a TTY and plain lines otherwise.
    #[arg(long, global = true, value_enum, default_value_t = VerboseMode::Auto)]
    pub verbose_mode: VerboseMode,

    /// Print diagnostic details for command rewrites and captures.
    #[arg(long, global = true)]
    pub debug: bool,

    #[command(flatten)]
    pub default_run_input: RunbookInputArgs,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check runbook prerequisites.
    Check(CheckArgs),
    /// Convert a runbook file to the opposite supported format.
    Convert(ConvertArgs),
    /// Print a runbook example for a topic.
    Example(ExampleArgs),
    /// Explain a feature contract or discovery path.
    Explain(ExplainArgs),
    /// Format a runbook file in place.
    Format(FormatArgs),
    /// Generate a starter runbook file.
    Init(InitArgs),
    /// Import a Markdown README into a starter runbook.
    Import(ImportArgs),
    /// Render a runbook to output.
    Run(RunArgs),
    /// Show help for the CLI or a specific subcommand.
    Help(HelpArgs),
    /// Print version/build identity.
    Version,
    /// Validate a runbook file.
    Validate(ValidateArgs),
}

#[derive(Debug, clap::Args)]
#[command(
    after_help = "Converts a runbook file from JSON to YAML or from YAML/YML to JSON.\nDefault input candidates are `./sw-runbook.json`, `./sw-runbook.yaml`, and `./sw-runbook.yml`.\nIf exactly one default runbook exists, `sw convert` uses it and writes the opposite format by default.\nIf more than one default runbook file exists, `sw convert` requires `--input-file`.\n`sw convert` is file-based only and does not accept `--input-file=-`.\nUse `--force` to overwrite an existing output file."
)]
pub struct ConvertArgs {
    /// Path to the input runbook file.
    #[arg(long)]
    pub input_file: Option<PathBuf>,

    /// Path to the converted output file.
    #[arg(long)]
    pub output_file: Option<PathBuf>,

    /// Output format for the converted runbook file.
    #[arg(long, value_enum)]
    pub output_format: Option<ConvertOutputFormat>,

    /// Overwrite the output file if it already exists.
    #[arg(long)]
    pub force: bool,
}

#[derive(Debug, clap::Args)]
#[command(
    after_help = "Formats JSON and YAML runbooks in place while preserving the existing file format.\nDefault input candidates are `./sw-runbook.json`, `./sw-runbook.yaml`, and `./sw-runbook.yml`.\nIf more than one default runbook file exists, `sw format` requires `--input-file`.\n`sw format` is file-based only and does not accept `--input-file=-`.\nUse `sw help validate` and `sw explain validate` when the question is about runbook correctness rather than formatting."
)]
pub struct FormatArgs {
    /// Path to the input runbook file.
    #[arg(long)]
    pub input_file: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
#[command(
    after_help = "Runbook-authored `Command` fields such as `trim_empty_lines`, `stream`, and `cleanup` are configured in the runbook, not as CLI flags.\n`Markdown`, `DisplayFile`, `Patch`, and `Command` entries may declare `indent` to nest rendered output inside surrounding Markdown structure.\nFile-based runbooks default to YAML, while `--input-file=-` defaults to JSON unless you set `--input-format=yaml`.\n`Command` entries default to a `2 minutes` timeout, while command-based prerequisite checks default to `5 seconds` unless the runbook sets `timeout`.\n`DisplayFile` fence detection currently recognizes `.java` as `java`, `.sql` as `sql`, and `.xml` as `xml`; other extensions render as `text`.\nUse `--verbose-mode=plain` for SSH-safe line-based progress output when terminal redraws are unreliable.\nUse `sw example Command` for a current YAML snippet and `sw example Command --output-format json` when you need the JSON shape.\nUse `sw explain run` for behavior and defaults."
)]
pub struct RunArgs {
    #[command(flatten)]
    pub input: RunbookInputArgs,

    /// Output format.
    #[arg(long, value_enum, default_value_t = RunOutputFormat::Markdown)]
    pub output_format: RunOutputFormat,

    /// Path to the generated output file.
    #[arg(long)]
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, clap::Args)]
#[command(
    after_help = "File-based runbooks default to YAML, while `--input-file=-` defaults to JSON unless you set `--input-format=yaml`.\nCommand-based prerequisite checks default to a `5 seconds` timeout unless the runbook sets `timeout`.\nUse `sw example Prerequisite` for a current prerequisite YAML snippet and `sw example Prerequisite --output-format json` when you need the JSON shape.\nUse `sw explain check` for behavior and defaults."
)]
pub struct CheckArgs {
    #[command(flatten)]
    pub input: RunbookInputArgs,
}

#[derive(Debug, clap::Args)]
#[command(
    after_help = "Validation accepts JSON, YAML, and YML files.\nFile-based runbooks default to YAML elsewhere in the CLI, while `--input-file=-` defaults to JSON unless you set `--input-format=yaml`.\nUse `sw explain validate` for behavior and defaults."
)]
pub struct ValidateArgs {
    #[command(flatten)]
    pub input: RunbookInputArgs,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    pub output_format: OutputFormat,
}

#[derive(Debug, Default, Clone, clap::Args)]
pub struct RunbookInputArgs {
    /// Path to the input runbook file. Use `-` to read from stdin.
    #[arg(long)]
    pub input_file: Option<PathBuf>,

    /// Input format for stdin runbooks. Ignored unless `--input-file=-` is used.
    #[arg(long, value_enum)]
    pub input_format: Option<InputFormat>,
}

#[derive(Debug, clap::Args)]
#[command(
    after_help = "Defaults to YAML output for file-based authoring.\nUse `--output-format yaml|json` to choose the example format explicitly.\nThe `Command` example includes current nested output fields such as `trim_empty_lines` and `stream`, along with rewrite, capture, and cleanup examples.\nUse `sw example DisplayFile` when you need the Java `collapse_method_body` transform for collapsing method bodies."
)]
pub struct ExampleArgs {
    /// Example topic such as `Command`, `DisplayFile`, or `rewrite.keep_between`.
    pub topic: String,

    /// Output format: `yaml` or `json`. Defaults to `yaml`.
    #[arg(long, value_name = "FORMAT")]
    pub output_format: Option<String>,
}

#[derive(Debug, clap::Args)]
#[command(
    after_help = "Defaults to `./sw-runbook.yaml`.\nYAML is the default file-based starter format.\nThe generated sample includes `Heading`, `Markdown`, `DisplayFile`, `Prerequisite`, and `Command` entries.\nRecognized output-file extensions are `.yaml`, `.yml`, and `.json`.\nUse `--force` to overwrite an existing target file.\nUse `sw explain init` for behavior and defaults."
)]
pub struct InitArgs {
    /// Path to the generated starter runbook file.
    #[arg(long)]
    pub output_file: Option<PathBuf>,

    /// Overwrite the output file if it already exists.
    #[arg(long)]
    pub force: bool,
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

#[derive(Debug, clap::Args)]
#[command(
    after_help = "Defaults to `./README.md` input and `./sw-runbook.yaml` output.\nYAML is the default file-based import format.\n`--output-format` accepts `yaml` or `json`; when omitted, `sw import` infers the format from a recognized output-file extension or defaults to YAML.\nHeadings map to `Heading` entries where possible, prose to `Markdown`, and fenced shell blocks to `Command` entries.\nUse `sw explain import` for the documented lossy-import contract."
)]
pub struct ImportArgs {
    /// Path to the Markdown README to import.
    #[arg(long)]
    pub input_file: Option<PathBuf>,

    /// Path to the generated runbook file.
    #[arg(long)]
    pub output_file: Option<PathBuf>,

    /// Output format for the generated runbook file.
    #[arg(long, value_enum)]
    pub output_format: Option<ImportOutputFormat>,

    /// Overwrite the output file if it already exists.
    #[arg(long)]
    pub force: bool,
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
pub enum InputFormat {
    Json,
    Yaml,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum ExplainOutputFormat {
    Text,
    Skill,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum ImportOutputFormat {
    Json,
    Yaml,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum ConvertOutputFormat {
    Json,
    Yaml,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum VerboseMode {
    Auto,
    Live,
    Plain,
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
    vec![
        "check", "convert", "example", "explain", "format", "init", "import", "run", "help",
        "version", "validate",
    ]
}

#![warn(clippy::print_stdout, clippy::print_stderr)]
#![deny(
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::complexity,
    clippy::correctness,
    clippy::suspicious,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::self_named_module_files,
    clippy::shadow_reuse
)]

use clap::Parser;
use clap_complete::Shell;
use std::ffi::OsString;

mod config_file;

#[derive(Parser, Debug)]
#[command(name = "clapfile", about, version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Run the CLI defined by the config file.
    #[command()]
    Run(RunArgs),

    /// Generate shell completions.
    #[command()]
    Completions(CompletionArgs),
}

#[derive(Parser, Debug)]
struct RunArgs {
    /// Configuration file.
    #[arg(short, long)]
    config: OsString,

    /// Arguments to pass to the CLI.
    #[arg(last = true)]
    args: Vec<OsString>,
}

#[derive(Parser, Debug)]
struct CompletionArgs {
    /// Target shell.
    #[arg()]
    shell: Shell,

    /// Configuration file.
    #[arg(short, long)]
    config: OsString,
}

fn main() {
    Args::parse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_command_parsing() {
        // Simply test that it does not crash.
        Args::parse_from(vec!["", "run", "--config", "config.toml"]);
    }
}

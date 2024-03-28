#![warn(clippy::print_stdout, clippy::print_stderr)]
#![deny(
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::complexity,
    clippy::correctness,
    clippy::suspicious,
    clippy::unwrap_used,
    clippy::self_named_module_files,
    clippy::shadow_reuse
)]
#![cfg_attr(not(test), deny(clippy::expect_used))]

use std::process::ExitCode;

use clap::Parser;
use tracing_subscriber::FmtSubscriber;

mod completions;
mod config_file;
mod runner;

#[derive(Parser, Debug)]
#[command(name = "clapfile", about, version)]
struct Args {
    #[clap(subcommand)]
    command: Command,

    /// Enable logging at the specified level.
    #[arg(long, global = true)]
    log_level: Option<tracing::Level>,
}

#[derive(Parser, Debug)]
enum Command {
    /// Run the CLI defined by the config file.
    #[command()]
    Run(runner::Args),

    /// Generate shell completions.
    #[command()]
    Completions(completions::Args),
}

fn main() -> anyhow::Result<ExitCode> {
    let args = Args::parse();

    if let Some(log_level) = args.log_level {
        tracing::subscriber::set_global_default(
            FmtSubscriber::builder().with_max_level(log_level).finish(),
        )?;
    }

    match args.command {
        Command::Completions(comp_args) => completions::gen_to_stdout(comp_args),
        Command::Run(run_args) => runner::run(run_args),
    }
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

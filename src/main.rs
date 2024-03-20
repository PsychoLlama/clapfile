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
use std::ffi::OsString;

mod completions;
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
    Completions(completions::Args),
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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Completions(comp_args) => completions::gen_to_stdout(comp_args)?,
        Command::Run(_) => todo!(),
    };

    Ok(())
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

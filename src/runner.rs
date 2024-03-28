use anyhow::Context;
use clap::Parser;
use std::{
    ffi::OsString,
    process::{Command, ExitCode, Stdio},
};

use crate::config_file::{self, Config};

const SHELL: &str = "sh";

#[derive(Parser, Debug)]
pub struct Args {
    /// Configuration file.
    #[arg(short, long)]
    config: OsString,

    /// Arguments to pass to the CLI.
    #[arg(last = true)]
    rest: Vec<OsString>,
}

#[tracing::instrument]
pub fn run(args: Args) -> anyhow::Result<ExitCode> {
    let config = config_file::load(args.config)?;
    let command: clap::Command = config.clone().into();

    let mut synthetic_argv = vec![command.get_name().into()];
    synthetic_argv.extend(args.rest);

    // Simply getting matches implements `--help` and friends.
    let matches = command.get_matches_from(synthetic_argv);
    let (target_command, _) = resolve_subcommand(config, matches)?;

    // TODO:
    // - Derive args
    // - Pass args to the script

    let script = target_command
        .run
        .context("Config file does not specify a script to execute")?;

    execute(&script)
}

/// Run a shell script and return the exit code. Stream stdin/stdout through the parent process.
fn execute(script: &String) -> anyhow::Result<ExitCode> {
    tracing::info!(script, "Executing shell script");

    let start_time = std::time::Instant::now();
    let status = Command::new(SHELL)
        .arg("-c")
        .arg(script)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    let exit_code: u8 = status
        .code()
        .context("Process killed")?
        .try_into()
        .context("Unexpected exit code")?;

    let duration = start_time.elapsed().as_millis();
    tracing::info!(exit_code, duration, "Shell script finished");

    Ok(exit_code.into())
}

/// Figure out which command is being executed and find the corresponding `Config`. The config
/// carries instructions on how to run the command.
fn resolve_subcommand(
    config: Config,
    matches: clap::ArgMatches,
) -> anyhow::Result<(Config, clap::ArgMatches)> {
    if let (Some(subcommands), Some((command_name, submatches))) =
        (config.clone().subcommands, matches.subcommand())
    {
        if let Some(subconfig) = subcommands.get(command_name) {
            return resolve_subcommand(subconfig.to_owned(), submatches.to_owned());
        }
    }

    Ok((config, matches))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn cmd(name: &str, subcommands: Vec<(String, Config)>) -> Config {
        Config {
            name: Some(name.into()),
            subcommands: if subcommands.is_empty() {
                None
            } else {
                Some(HashMap::from_iter(subcommands))
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_command_resolution_at_root() {
        let config = cmd("root", vec![]);

        let command: clap::Command = config.clone().into();
        let matches = command.get_matches_from(vec!["/cmd"]);
        let (subcommand, _) =
            resolve_subcommand(config, matches).expect("Could not resolve command");

        assert_eq!(subcommand.name, Some("root".into()));
    }

    #[test]
    fn test_command_resolution_at_subcommand() {
        let config = cmd(
            "root",
            vec![("child-command".into(), cmd("child-command", vec![]))],
        );

        let command: clap::Command = config.clone().into();
        let matches = command.get_matches_from(vec!["/cmd", "child-command"]);
        let (subcommand, _) =
            resolve_subcommand(config, matches).expect("Could not resolve command");

        assert_eq!(subcommand.name, Some("child-command".into()));
    }

    #[test]
    fn test_deep_subcommand_resolution() {
        let config = cmd(
            "root",
            vec![(
                "child-command".into(),
                cmd(
                    "child-command",
                    vec![(
                        "grandchild-command".into(),
                        cmd("grandchild-command", vec![]),
                    )],
                ),
            )],
        );

        let command: clap::Command = config.clone().into();
        let matches = command.get_matches_from(vec!["/cmd", "child-command", "grandchild-command"]);
        let (subcommand, _) =
            resolve_subcommand(config, matches).expect("Could not resolve command");

        assert_eq!(subcommand.name, Some("grandchild-command".into()));
    }
}

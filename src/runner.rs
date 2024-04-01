use anyhow::Context;
use clap::Parser;
use std::{
    collections::HashMap,
    ffi::OsString,
    process::{Command, ExitCode, Stdio},
};

use crate::{
    config_file::{self, CommandConfig},
    exporters::to_env_record,
};

#[derive(Parser, Debug)]
pub struct Args {
    /// Configuration file.
    #[arg(short, long)]
    config: OsString,

    /// Arguments to pass to the CLI.
    #[arg(last = true)]
    rest: Vec<OsString>,

    /// Shell used to execute scripts.
    #[arg(long, default_value = "sh")]
    shell: String,
}

#[tracing::instrument]
pub fn run(args: Args) -> anyhow::Result<ExitCode> {
    let config = config_file::load(args.config)?;
    let command: clap::Command = config.clone().into();

    let mut synthetic_argv = vec![command.get_name().into()];
    synthetic_argv.extend(args.rest);

    // Simply getting matches implements `--help` and friends.
    let matches = command.clone().get_matches_from(synthetic_argv);
    let (target_config, mut target_command, target_matches) =
        resolve_subcommand(config, command, matches)?;

    if let Some(script) = target_config.run {
        let env = to_env_record(&target_config.args.unwrap_or_default(), &target_matches);
        execute(&args.shell, &script, env)
    } else {
        target_command.print_help()?;
        Ok(ExitCode::FAILURE)
    }
}

/// Run a shell script and return the exit code. Stream stdin/stdout through the parent process.
fn execute(
    shell: &str,
    script: &String,
    env: HashMap<OsString, OsString>,
) -> anyhow::Result<ExitCode> {
    tracing::info!(script, "Executing shell script");

    let start_time = std::time::Instant::now();
    let status = Command::new(shell)
        .arg("-c")
        .arg(script)
        .envs(env)
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
    config: CommandConfig,
    command: clap::Command,
    matches: clap::ArgMatches,
) -> anyhow::Result<(CommandConfig, clap::Command, clap::ArgMatches)> {
    if let (Some(subcommands), Some((command_name, submatches))) =
        (config.clone().subcommands, matches.subcommand())
    {
        if let (Some(subconfig), Some(subcommand)) = (
            subcommands.get(command_name),
            command
                .get_subcommands()
                .find(|c| c.get_name() == command_name),
        ) {
            return resolve_subcommand(
                subconfig.to_owned(),
                subcommand.to_owned(),
                submatches.to_owned(),
            );
        }
    }

    Ok((config, command, matches))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn cmd(name: &str, subcommands: Vec<(String, CommandConfig)>) -> CommandConfig {
        CommandConfig {
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
        let matches = command.clone().get_matches_from(vec!["/cmd"]);
        let (subconfig, _, _) =
            resolve_subcommand(config, command, matches).expect("Could not resolve command");

        assert_eq!(subconfig.name, Some("root".into()));
    }

    #[test]
    fn test_command_resolution_at_subcommand() {
        let config = cmd(
            "root",
            vec![("child-command".into(), cmd("child-command", vec![]))],
        );

        let command: clap::Command = config.clone().into();
        let matches = command
            .clone()
            .get_matches_from(vec!["/cmd", "child-command"]);
        let (subconfig, _, _) =
            resolve_subcommand(config, command, matches).expect("Could not resolve command");

        assert_eq!(subconfig.name, Some("child-command".into()));
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
        let matches =
            command
                .clone()
                .get_matches_from(vec!["/cmd", "child-command", "grandchild-command"]);
        let (subconfig, _, _) =
            resolve_subcommand(config, command, matches).expect("Could not resolve command");

        assert_eq!(subconfig.name, Some("grandchild-command".into()));
    }
}

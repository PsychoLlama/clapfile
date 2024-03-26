use clap::Parser;
use std::ffi::OsString;

use crate::config_file::{self, Config};

#[derive(Parser, Debug)]
pub struct Args {
    /// Configuration file.
    #[arg(short, long)]
    config: OsString,

    /// Arguments to pass to the CLI.
    #[arg(last = true)]
    rest: Vec<OsString>,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let config = config_file::load(args.config)?;
    let command: clap::Command = config.clone().into();

    let mut synthetic_argv = vec![command.get_name().into()];
    synthetic_argv.extend(args.rest);

    // Simply getting matches implements `--help` and friends.
    let matches = command.get_matches_from(synthetic_argv);
    let (target_command, _) = resolve_subcommand(config, matches)?;

    // TODO:
    // - Run the command
    // - Commit the clapfile with real project tasks
    // - Add tracing
    // - Derive args
    // - Pass args to the script

    todo!("run command: {:?}", target_command);
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

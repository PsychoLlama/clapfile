use anyhow::Context;
use clap::Command;
use serde::Deserialize;
use std::{collections::HashMap, ffi::OsString};

/// Configuration file structure. Fields mirror those of `clap::Command`.
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(Default))]
pub struct CommandConfig {
    pub name: Option<String>,
    pub about: Option<String>,
    pub version: Option<String>,
    pub subcommands: Option<HashMap<String, CommandConfig>>,

    // Must be a vec because positional parameters expect order.
    pub args: Option<Vec<ArgumentConfig>>,

    /// This is the script that gets executed when the command runs. It can be a path to an
    /// executable file or a shell command.
    pub run: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(Default))]
pub struct ArgumentConfig {
    pub id: String,
    pub required: Option<bool>,
}

/// Load the configuration file and convert it to a `clap::Command`.
#[tracing::instrument]
pub fn load(file_path: OsString) -> anyhow::Result<CommandConfig> {
    tracing::info!(?file_path, "Reading config");

    let config_contents =
        std::fs::read_to_string(file_path).context("Failed to load config file")?;

    tracing::info!("Parsing config");
    Ok(toml::from_str::<CommandConfig>(&config_contents)?)
}

impl From<CommandConfig> for clap::Command {
    fn from(config: CommandConfig) -> Command {
        let mut command = Command::new(config.name.unwrap_or_default());

        /* --- Set Metadata --- */

        if let Some(about) = config.about {
            command = command.about(about);
        }

        if let Some(version) = config.version {
            command = command.version(version);
        }

        /* --- Register Arguments --- */

        for argument in config.args.unwrap_or_default() {
            command = command.arg(Into::<clap::Arg>::into(argument));
        }

        /* --- Register Subcommands --- */

        for (name, script) in config.subcommands.unwrap_or_default() {
            let mut subcommand: Command = script.into();

            // Inherit command name from hashmap key if unset.
            if subcommand.get_name() == "" {
                subcommand = subcommand.name(name);
            }

            command = command.subcommand(subcommand);
        }

        command
    }
}

impl From<ArgumentConfig> for clap::Arg {
    fn from(conf: ArgumentConfig) -> clap::Arg {
        let mut arg = clap::Arg::new(conf.id);

        if let Some(required) = conf.required {
            arg = arg.required(required);
        }

        arg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::builder::StyledStr;

    #[test]
    fn test_simple_command() {
        let app = CommandConfig {
            name: Some(String::from("cmd-test")),
            ..CommandConfig::default()
        };

        let command: Command = app.into();
        assert_eq!(command.get_name(), "cmd-test");
    }

    #[test]
    fn test_command_description() {
        let app = CommandConfig {
            about: Some(String::from("test command")),
            ..CommandConfig::default()
        };

        let command: Command = app.into();
        assert_eq!(command.get_about(), Some(&StyledStr::from("test command")));
    }

    #[test]
    fn test_command_version() {
        let app = CommandConfig {
            version: Some(String::from("0.1.0")),
            ..CommandConfig::default()
        };

        let command: Command = app.into();
        assert_eq!(command.get_version(), Some("0.1.0"));
    }

    #[test]
    fn test_subcommand_exists() {
        let app = CommandConfig {
            subcommands: Some(HashMap::from_iter(vec![(
                String::from("example"),
                CommandConfig {
                    run: Some("echo test".into()),
                    ..CommandConfig::default()
                },
            )])),
            ..CommandConfig::default()
        };

        let command: Command = app.into();
        let example = command
            .get_subcommands()
            .find(|command| command.get_name() == "example");

        assert!(example.is_some());
    }

    #[test]
    fn test_subcommand_name_override() {
        let app = CommandConfig {
            subcommands: Some(HashMap::from_iter(vec![(
                String::from("example"),
                CommandConfig {
                    name: Some(String::from("example-override")),
                    run: Some("echo test".into()),
                    ..CommandConfig::default()
                },
            )])),
            ..CommandConfig::default()
        };

        let command: Command = app.into();
        let example = command
            .get_subcommands()
            .find(|command| command.get_name() == "example-override");

        assert!(example.is_some());
    }

    #[test]
    fn test_simple_argument_instantiation() {
        let conf = ArgumentConfig {
            id: "random".into(),
            ..ArgumentConfig::default()
        };

        let arg: clap::Arg = conf.into();
        assert_eq!(arg.get_id(), "random");
    }

    #[test]
    fn test_argument_id_default() {
        let conf = ArgumentConfig::default();
        let arg: clap::Arg = conf.into();
        assert_eq!(arg.get_id(), "");
    }

    #[test]
    fn test_argument_required_setting() {
        let conf = ArgumentConfig {
            required: Some(true),
            ..ArgumentConfig::default()
        };

        let arg: clap::Arg = conf.into();
        assert!(arg.is_required_set());
    }
}

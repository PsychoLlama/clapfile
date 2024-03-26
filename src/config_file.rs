use anyhow::Context;
use clap::Command;
use serde::Deserialize;
use std::{collections::HashMap, ffi::OsString};

/// Configuration file structure. Fields mirror those of `clap::Command`.
#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(Default))]
pub struct Config {
    pub name: Option<String>,
    pub about: Option<String>,
    pub version: Option<String>,
    pub subcommands: Option<HashMap<String, Config>>,

    /// This is the script that gets executed when the command runs. It can be a path to an
    /// executable file or a shell command.
    pub run: Option<String>,
}

/// Load the configuration file and convert it to a `clap::Command`.
#[allow(dead_code)]
pub fn load(config_file: OsString) -> anyhow::Result<Config> {
    let config_contents =
        std::fs::read_to_string(config_file).context("Failed to load config file")?;

    Ok(serde_yaml::from_str::<Config>(&config_contents)?)
}

impl From<Config> for Command {
    fn from(config: Config) -> Command {
        let mut command = Command::new(config.name.unwrap_or_default());

        /* --- Set Metadata --- */

        if let Some(about) = config.about {
            command = command.about(about);
        }

        if let Some(version) = config.version {
            command = command.version(version);
        }

        /* --- Register Subcommands --- */

        if let Some(subcommands) = config.subcommands {
            for (name, script) in subcommands {
                let mut subcommand: Command = script.into();

                // Inherit command name from hashmap key if unset.
                if subcommand.get_name() == "" {
                    subcommand = subcommand.name(name);
                }

                command = command.subcommand(subcommand);
            }
        }

        command
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::builder::StyledStr;

    #[test]
    fn test_simple_command() {
        let app = Config {
            name: Some(String::from("cmd-test")),
            ..Config::default()
        };

        let command: Command = app.into();
        assert_eq!(command.get_name(), "cmd-test");
    }

    #[test]
    fn test_command_description() {
        let app = Config {
            about: Some(String::from("test command")),
            ..Config::default()
        };

        let command: Command = app.into();
        assert_eq!(command.get_about(), Some(&StyledStr::from("test command")));
    }

    #[test]
    fn test_command_version() {
        let app = Config {
            version: Some(String::from("0.1.0")),
            ..Config::default()
        };

        let command: Command = app.into();
        assert_eq!(command.get_version(), Some("0.1.0"));
    }

    #[test]
    fn test_subcommand_exists() {
        let app = Config {
            subcommands: Some(HashMap::from_iter(vec![(
                String::from("example"),
                Config {
                    run: Some("echo test".into()),
                    ..Config::default()
                },
            )])),
            ..Config::default()
        };

        let command: Command = app.into();
        let example = command
            .get_subcommands()
            .find(|command| command.get_name() == "example");

        assert!(example.is_some());
    }

    #[test]
    fn test_subcommand_name_override() {
        let app = Config {
            subcommands: Some(HashMap::from_iter(vec![(
                String::from("example"),
                Config {
                    name: Some(String::from("example-override")),
                    run: Some("echo test".into()),
                    ..Config::default()
                },
            )])),
            ..Config::default()
        };

        let command: Command = app.into();
        let example = command
            .get_subcommands()
            .find(|command| command.get_name() == "example-override");

        assert!(example.is_some());
    }
}

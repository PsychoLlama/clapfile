use anyhow::Context;
use clap::Command;
use serde::Deserialize;
use std::ffi::OsString;

/// Configuration file structure. Fields mirror those of `clap::Command`.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Default))]
struct Config {
    name: String,
    about: Option<String>,
    version: Option<String>,
}

/// Load the configuration file and convert it to a `clap::Command`.
#[allow(dead_code)]
pub fn load(config_file: OsString) -> anyhow::Result<Command> {
    let config_contents =
        std::fs::read_to_string(config_file).context("Failed to load config file")?;

    let app = serde_yaml::from_str::<Config>(&config_contents)?;
    Ok(app.into())
}

impl From<Config> for Command {
    fn from(config: Config) -> Command {
        let mut command = Command::new(config.name);

        if let Some(about) = config.about {
            command = command.about(about);
        }

        if let Some(version) = config.version {
            command = command.version(version);
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
            name: String::from("cmd-test"),
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
}

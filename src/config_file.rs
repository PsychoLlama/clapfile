use clap::Command;
use serde::Deserialize;
use std::ffi::OsString;

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    about: Option<String>,
    version: Option<String>,
}

#[allow(dead_code)]
pub fn load(config_file: OsString) -> anyhow::Result<Command> {
    let config_contents = std::fs::read_to_string(config_file)?;
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

    impl Default for Config {
        fn default() -> Self {
            Config {
                name: String::new(),
                about: None,
                version: None,
            }
        }
    }

    #[test]
    fn test_simple_command() {
        let mut app = Config::default();
        app.name = String::from("cmd-test");

        let command: Command = app.into();
        assert_eq!(command.get_name(), "cmd-test");
    }

    #[test]
    fn test_command_description() {
        let mut app = Config::default();
        app.about = Some(String::from("test command"));

        let command: Command = app.into();
        assert_eq!(command.get_about(), Some(&StyledStr::from("test command")));
    }

    #[test]
    fn test_command_version() {
        let mut app = Config::default();
        app.version = Some(String::from("0.1.0"));

        let command: Command = app.into();
        assert_eq!(command.get_version(), Some("0.1.0"));
    }
}

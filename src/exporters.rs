use std::{collections::HashMap, ffi::OsString};

use crate::config_file::ArgumentConfig;

/// Export arguments to environment variables.
pub fn to_env_record(
    args: &Vec<ArgumentConfig>,
    matches: &clap::ArgMatches,
) -> HashMap<OsString, OsString> {
    let mut map = HashMap::new();

    for arg in args {
        if let Some(value) = matches.get_one::<String>(&arg.id) {
            map.insert(arg.id.clone().into(), value.clone().into());
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::config_file::CommandConfig;

    #[test]
    fn test_env_record_exporter() {
        let config = CommandConfig {
            args: Some(vec![
                ArgumentConfig {
                    id: "arg1".into(),
                    ..Default::default()
                },
                ArgumentConfig {
                    id: "arg2".into(),
                    ..Default::default()
                },
            ]),
            ..CommandConfig::default()
        };

        let command: clap::Command = config.clone().into();
        let matches = command.get_matches_from(vec!["test", "value1", "value2"]);
        let env = to_env_record(&config.args.unwrap_or_default(), &matches);

        assert_eq!(
            env,
            HashMap::from_iter(vec![
                ("arg1".into(), "value1".into()),
                ("arg2".into(), "value2".into()),
            ])
        );
    }
}
